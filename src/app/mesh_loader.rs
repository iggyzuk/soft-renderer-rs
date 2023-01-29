use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use rayon::prelude::*;

use crate::{
    graphics::{mesh::Mesh, vertex::Vertex},
    math::linear_algebra::vector::Vector4,
};

#[derive(Debug, Default)]
pub struct OBJModel {
    pub vertices: Vec<Vector4>,
    pub tex_coords: Vec<Vector4>,
    pub normals: Vec<Vector4>,
    pub indices: Vec<OBJIndex>,
}
#[derive(Debug, Default)]
pub struct OBJIndex {
    pub vertex_index: usize,
    pub tex_coord_index: usize,
    pub normal_index: usize,
}

#[derive(Debug, Default)]
pub struct IndexedModel {
    pub vertices: Vec<Vector4>,
    pub tex_coords: Vec<Vector4>,
    pub normals: Vec<Vector4>,
    pub tangents: Vec<Vector4>,
    pub indices: Vec<usize>,
}

pub fn load_mesh(filepath: &str) -> Mesh {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut model = OBJModel::default();

    for line in reader.lines() {
        if let Ok(text) = line {
            let tokens: Vec<&str> = text.split(' ').collect();

            assert!(tokens.len() > 0);

            if tokens[0] == "#" {
                continue;
            }

            if tokens[0] == "v" {
                model.vertices.push(Vector4::new(
                    tokens[1].parse::<f32>().unwrap(),
                    tokens[2].parse::<f32>().unwrap(),
                    tokens[3].parse::<f32>().unwrap(),
                    1.0,
                ));
            } else if tokens[0] == "vt" {
                model.tex_coords.push(Vector4::new(
                    tokens[1].parse::<f32>().unwrap(),
                    1.0 - tokens[2].parse::<f32>().unwrap(),
                    0.0,
                    0.0,
                ));
            } else if tokens[0] == "vn" {
                model.normals.push(Vector4::new(
                    tokens[1].parse::<f32>().unwrap(),
                    tokens[2].parse::<f32>().unwrap(),
                    tokens[3].parse::<f32>().unwrap(),
                    0.0,
                ));
            } else if tokens[0] == "f" {
                for i in 1..tokens.len() {
                    let index_tokens: Vec<&str> = tokens[i].split('/').collect();

                    let mut index = OBJIndex::default();
                    index.vertex_index = index_tokens[0].parse::<usize>().unwrap() - 1;
                    index.tex_coord_index = index_tokens[1].parse::<usize>().unwrap() - 1;
                    index.normal_index = index_tokens[2].parse::<usize>().unwrap() - 1;

                    model.indices.push(index);
                }
            }
        }
    }

    let indexed_model = to_indexed_model(model);

    // make a new mesh and add all indexed-vertices/tex-coords/normals into it
    let mut mesh = Mesh::default();

    for i in 0..indexed_model.vertices.len() {
        mesh.vertices.push(Vertex::new(
            indexed_model.vertices[i],
            indexed_model.tex_coords[i],
            indexed_model.normals[i],
        ));
    }

    mesh.indices = vec![0; indexed_model.indices.len()];

    for i in 0..indexed_model.indices.len() {
        mesh.indices[i] = indexed_model.indices[i];
    }

    return mesh;
}

pub fn to_indexed_model(obj: OBJModel) -> IndexedModel {
    let mut model = IndexedModel::default();
    let mut index_map = HashMap::new(); // OBJModel.indices -> IndexedModel.indices
    let mut curr_vertex_index = 0;

    // index the model in parallel
    let results: Vec<(usize, Option<usize>, Vector4, Vector4, Vector4)> = obj
        .indices
        .par_iter()
        .enumerate()
        .map(|(i, curr_index)| {
            let curr_pos = obj.vertices[curr_index.vertex_index];
            let curr_tex_coord = obj.tex_coords[curr_index.tex_coord_index];
            let curr_normal = obj.normals[curr_index.normal_index];

            // Check for duplicates O(n^2)
            let mut prev_index = None;

            for j in 0..i {
                let old_index = &obj.indices[j];

                if curr_index.vertex_index == old_index.vertex_index
                    && curr_index.tex_coord_index == old_index.tex_coord_index
                    && curr_index.normal_index == old_index.normal_index
                {
                    prev_index = Some(j);
                    break;
                }
            }

            return (i, prev_index, curr_pos, curr_tex_coord, curr_normal);
        })
        .collect();

    // add results to the model
    for (i, prev_index, curr_pos, curr_tex_coord, curr_normal) in results {
        if let Some(value) = prev_index {
            model.indices.push(*index_map.get(&value).unwrap());
        } else {
            index_map.insert(i, curr_vertex_index);

            model.vertices.push(curr_pos);
            model.tex_coords.push(curr_tex_coord);
            model.normals.push(curr_normal);
            model.indices.push(curr_vertex_index);

            curr_vertex_index += 1;
        }
    }

    return model;
}
