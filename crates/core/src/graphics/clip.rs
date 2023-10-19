use super::{gradients::Triangle, vertex::Vertex};

// clip vertices and return the vertices that are visible
// 1. can be the same as the input
// 2. can be the none when they are completely out of view
// 3. can be new vertices due to clipping on all axis
pub fn clip_triangle(v1: Vertex, v2: Vertex, v3: Vertex) -> Option<Vec<Triangle>> {
    // # 3d homogenous clipping
    // https://fabiensanglard.net/polygon_codec/
    //
    // 1d clipping example:
    // -1 |-----a----b--| +1  *c*   <--- point out of range
    //
    // a > -1 and a < +1
    // b > -1 and b < +1
    // c > -1 and c < +1 !!!
    //
    // d = lerp from b to c so that result is exactly 1
    //
    // -1 |-----a----b--d +1
    //
    // lerp formula
    // L = linear interpolation factor
    // 1 = `B`(1-L)+`C`*L
    //
    // extracted and simplified
    // L = 1-B / (1-B)-(1-C)
    //
    // with perspective divide changes
    // L = Wb - B / (Wb - B) - (Wc - C)
    //
    // note, we clip before perspective divide to avoid issues with linear interpolations / gradients

    let mut vertices = vec![v1, v2, v3];

    // try clip vertices x
    if !clip_polygon_axis(&mut vertices, 0) {
        return None;
    }
    // try clip vertices y
    if !clip_polygon_axis(&mut vertices, 1) {
        return None;
    }
    // try clip vertices z
    if !clip_polygon_axis(&mut vertices, 2) {
        return None;
    }

    let mut triangles = Vec::new();

    // construct new vertices and fill the triangle
    let initial_vertex = vertices[0];

    // # creating triangles from multiple vertices
    // given the points: A,B,C,D,E
    // use formula: [A,B,C], [A,C,D], [A,D,E], etc
    // start from 1(A) and connect it to 2 next ones (B,C)
    for i in 1..vertices.len() - 1 {
        let v1 = initial_vertex;
        let v2 = vertices[i];
        let v3 = vertices[i + 1];

        // # debug: draw with green triangles that are not broken.
        // let mut bitmap = Bitmap::new(1, 1);
        // bitmap.fill(&Color::RED);

        // fill the triangle
        triangles.push(Triangle::new(v1, v2, v3));
        // self.fill_triangle(v1, v2, v3, &material, light);
    }

    return Some(triangles);
}

// clips for one particular axis
fn clip_polygon_axis(vertices: &mut Vec<Vertex>, component: usize) -> bool {
    let mut new_vertices = Vec::new();

    // clip on specific component on the +w
    //
    //          w (factor)
    // prev v _ |
    //  .       | -
    //   .      |    - curr v
    //    .     |  /
    //     .    |/
    //      .  /|
    //          |

    // the result will be in new_vertices
    clip_polygon_component(vertices, component, 1.0, &mut new_vertices);
    vertices.clear();

    // no new-vertices so there are no vertices are in the screen
    if new_vertices.is_empty() {
        return false;
    }

    // clip on specific component on the -w
    // with the newly creates vertices the result will be in the original vertices list
    clip_polygon_component(&mut new_vertices, component, -1.0, vertices);
    new_vertices.clear();

    // return true when there are new vertices
    return !vertices.is_empty();
}

// clips on components: x,y,z
fn clip_polygon_component(
    vertices: &Vec<Vertex>,   // vertices to clip
    component_index: usize,   // which component to clip on (x:0,y:1,z:2)
    factor: f32,              // -w or +w
    result: &mut Vec<Vertex>, // resulting clipped vertices
) {
    // start with the very last vertex in the list
    // compare loop checks (prev-curr) v3-v1, v1-v2, v2->v3
    let mut prev_vertex = &vertices[vertices.len() - 1];
    // previous vertex component (x,y,z)
    // factor allows us to reuse this code for -x and +x, (and -y +y, -z +z)
    let mut prev_component = prev_vertex.get(component_index) * factor;
    // whether or not the previous vertex is inside the cliping range
    let mut prev_inside = prev_component <= prev_vertex.position.w;

    for curr_vertex in vertices {
        let curr_component = curr_vertex.get(component_index) * factor;
        let curr_inside = curr_component <= curr_vertex.position.w;

        // XOR if only one of the vertices is inside (current or previous)

        if curr_inside ^ prev_inside {
            // find the lerp amount to clip the vertex
            // L = Wb - B / (Wb - B) - (Wc - C)
            let b = prev_vertex.position.w - prev_component;
            let c = curr_vertex.position.w - curr_component;
            let lerp_amt = b / (b - c);

            // clip vertex by lerping and push it into the result list
            result.push(prev_vertex.lerp(curr_vertex, lerp_amt));
        }

        // current is inside the clipping range so add it into the result list
        if curr_inside {
            result.push(curr_vertex.clone());
        }

        prev_vertex = curr_vertex;
        prev_component = curr_component;
        prev_inside = curr_inside;
    }
}
