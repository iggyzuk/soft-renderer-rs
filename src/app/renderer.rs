use pixels::Pixels;
use std::{mem};

use crate::{
    graphics::{
        bitmap::Bitmap,
        color::Color,
        edge::Edge,
        gradients::{Gradients, Triangle},
        material::Material,
        mesh::Mesh,
        vertex::Vertex,
    },
    math::linear_algebra::matrix::Matrix4,
};

#[derive(Debug, Default)]
pub struct Debug {
    wireframe: bool,
    solid: bool,
    depth: bool,
    scanline_fill: bool,
    depth_miss: bool,
}

#[derive(Debug)]
pub struct Renderer {
    pub width: u32,  // width in pixels
    pub height: u32, // height in pixels
    pub screenspace: Matrix4,
    pub color_buffer: Bitmap, // the main color buffer (r,g,b,a)
    // @todo: could be an array/slice
    pub depth_buffer: Vec<f32>, // the z buffer (1 - 0) -> (far - close)
    pub pixels: Pixels,         // pixels to that are actually drawn to the screen
    pub debug: Debug,           // debug variables for displaying extra information
}

impl Renderer {
    pub fn new(width: u32, height: u32, pixels: Pixels) -> Self {
        let mut renderer = Self {
            width,
            height,
            screenspace: Matrix4::screenspace(width as f32, height as f32),
            color_buffer: Bitmap::new(width, height),
            depth_buffer: vec![1.0; (width * height) as usize],
            pixels: pixels,
            debug: Default::default(),
        };

        // renderer.debug.wireframe = true;
        // renderer.debug.solid = true;
        // renderer.debug.depth = true;
        // renderer.debug.depth_miss = true;
        // renderer.debug.scanline_fill = true;

        renderer.clear_depth_buffer();
        renderer
    }

    pub fn clear_depth_buffer(&mut self) {
        let size = (self.width * self.height) as usize;
        for i in 0..size {
            self.depth_buffer[i] = 1.0;
        }
    }

    pub fn draw_mesh(&mut self, mesh: &Mesh, view_projection: &Matrix4, transform: &Matrix4, material: &Material) {
        let mvp = Matrix4::multiply(view_projection, transform);

        let pos = Matrix4::multiply_vector(&mvp, transform.translation());

        self.color_buffer.set_pixel(pos.x as u32, pos.y as u32, &Color::WHITE);

        // @todo: run this in parallel, will need a RwLock for color/depth buffers
        for chunk in mesh.indices.chunks_exact(3) {
            let v1 = mesh.vertices[chunk[0]].transform(&mvp, transform);
            let v2 = mesh.vertices[chunk[1]].transform(&mvp, transform);
            let v3 = mesh.vertices[chunk[2]].transform(&mvp, transform);
            self.draw_triangle(v1, v2, v3, material);
        }
    }

    // try draw a triangle can be partially visible, fully visible, or completely invisible
    pub fn draw_triangle(&mut self, v1: Vertex, v2: Vertex, v3: Vertex, material: &Material) {
        let v1_visible = v1.is_inside_view_frustum();
        let v2_visible = v2.is_inside_view_frustum();
        let v3_visible = v3.is_inside_view_frustum();

        // all vertices are visible so draw the triangle as is
        if v1_visible && v2_visible && v3_visible {
            self.fill_triangle(v1, v2, v3, material);

            // # debug: draw with green triangles that are not broken
            // let mut fill = Bitmap::new(1, 1);
            // fill.fill(&Color::WHITE);
            // self.fill_triangle(v1, v2, v3, &fill);
        } else {
            // one or more (or all) vertices are not visible, we must clip them
            self.clip_triangle(v1, v2, v3, material);
        }
    }

    fn clip_triangle(&mut self, v1: Vertex, v2: Vertex, v3: Vertex, material: &Material) {
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
        // note, we clip before perspective divide to avoid issues with linear interpolations

        let mut vertices = vec![v1, v2, v3];

        // a list to store vertices while we're clipping
        let mut aux = Vec::new();

        // println!("triangle");
        // fails here...

        // try clip vertices on all axis (x,y,z)
        if !self.clip_polygon_axis(&mut vertices, &mut aux, 0) {
            return;
        }
        if !self.clip_polygon_axis(&mut vertices, &mut aux, 1) {
            return;
        }
        if !self.clip_polygon_axis(&mut vertices, &mut aux, 2) {
            return;
        }

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
            self.fill_triangle(v1, v2, v3, &material);
        }
    }

    // @todo: try taking ownership of vertices
    // clips for one particular axis
    fn clip_polygon_axis(&self, vertices: &mut Vec<Vertex>, aux: &mut Vec<Vertex>, component: usize) -> bool {
        // clip on specific component on the +w
        // the result will be in aux
        self.clip_polygon_component(vertices, component, 1.0, aux);
        vertices.clear();

        // no aux-vertices so there are no vertices are in the screen
        if aux.is_empty() {
            return false;
        }

        // clip on specific component on the -w
        // the result will be in vertices
        self.clip_polygon_component(aux, component, -1.0, vertices);
        aux.clear();

        // return true when there are new vertices
        return !vertices.is_empty();
    }

    // clips on components: x,y,z
    fn clip_polygon_component(
        &self,
        vertices: &Vec<Vertex>,   // vertices to clip
        index: usize,             // which component to clip on (x:0,y:1,z:2)
        factor: f32,              // -w or +w
        result: &mut Vec<Vertex>, // resulting clipped vertices
    ) {
        // start with the very last vertex in the list
        // compare loop checks (prev-curr) v3-v1, v1-v2, v2->v3
        let mut prev_vertex = &vertices[vertices.len() - 1];
        // previous vertex component (x,y,z)
        let mut prev_component = prev_vertex.get(index) * factor;
        // whether or not the previous vertex is inside the cliping range
        let mut prev_inside = prev_component <= prev_vertex.position.w;

        for curr_vertex in vertices {
            let curr_component = curr_vertex.get(index) * factor;
            let curr_inside = curr_component <= curr_vertex.position.w;

            // XOR if only one of the vertices is inside
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

    // given 3 vertices we will fill everything in between with pixels
    pub fn fill_triangle(&mut self, v1: Vertex, v2: Vertex, v3: Vertex, material: &Material) {
        let identity = Matrix4::new_identity();

        // transform vertices from world-space to screen-space using matrices.
        // z is used for depth, and w is used for perspective
        // perspective divide puts us into image space
        //      +1  +1
        //       | /
        //  -1 ------ +1
        //     / |
        //   -1 -1

        let mut min = v1.transform(&self.screenspace, &identity).perspective_divide();
        let mut mid = v2.transform(&self.screenspace, &identity).perspective_divide();
        let mut max = v3.transform(&self.screenspace, &identity).perspective_divide();

        // back face culling
        // cross product: min->max and min->min will give us the handedness: right > 0 and left < 0
        if min.triangle_area_times_two(&max, &mid) >= 0.0 {
            return;
        }

        // vertices can come in any order so we must sort them, in ideal case they are as the following:

        // min .
        //  .     .
        //   .     mid
        //    .    .
        //     .  .
        //      max

        // but what about a some bad sorting, let's follow it through an example:
        // max -> y: 0
        // mid -> y: 1
        // min -> y: 2

        if max.position.y < mid.position.y {
            mem::swap(&mut max, &mut mid);
        }

        // swap max(0) with mid(1)
        // max -> y: 1 *
        // mid -> y: 0 *
        // min -> y: 2

        if mid.position.y < min.position.y {
            mem::swap(&mut mid, &mut min);
        }

        // swap mid(0) with min(2)
        // max -> y: 1
        // mid -> y: 2 *
        // min -> y: 0 *

        if max.position.y < mid.position.y {
            mem::swap(&mut max, &mut mid);
        }

        // `again` swap max(1) with mid(2)
        // max -> y: 2 *
        // mid -> y: 1 *
        // min -> y: 0

        // recompute handedness
        let handedness = min.triangle_area_times_two(&max, &mid) >= 0.0;

        // # debug: draw vertices min(red), mid(green), max(blue)
        // self.color_buffer
        //     .set_pixel(min.position.x as u32, min.position.y as u32, &Color::RED);
        // self.color_buffer
        //     .set_pixel(mid.position.x as u32, mid.position.y as u32, &Color::GREEN);
        // self.color_buffer
        //     .set_pixel(max.position.x as u32, max.position.y as u32, &Color::BLUE);

        self.scan_triangle(min, mid, max, handedness, material);
    }

    pub fn scan_triangle(&mut self, min: Vertex, mid: Vertex, max: Vertex, handedness: bool, material: &Material) {
        // construct gradients for the triangle
        // it contains tex-coords, one-over-z, depth, light-amt for all 3 vertices
        let gradients = Gradients::new(Triangle::new(min.clone(), mid.clone(), max.clone()));

        // # debug: switch textures to see how triangles are drawn
        // make sure to change &bitmap to &debug_tex_1/&debug_tex_1 in scan_edges(...)
        // let mut debug_tex_1 = Bitmap::new(1, 1);
        // let mut debug_tex_2 = Bitmap::new(1, 1);

        // // right edge is longest (mid point on the left)
        // if handedness {
        //     debug_tex_1.fill(&Color::from_hex(0xFF0000FF));
        //     debug_tex_2.fill(&Color::from_hex(0xFFFF00FF));
        // }
        // // left edge is longest (mid point on the right)
        // else {
        //     debug_tex_1.fill(&Color::from_hex(0x0000FFFF));
        //     debug_tex_2.fill(&Color::from_hex(0x00FFFFFF));
        // }

        // construct three edges for the triangle

        // @todo: handedness is actually a start_index? what is that?

        // if handedness is 0 then the top to bottom is the left edge, everything else is a right edge.

        // edge that goes from top to bottom, gradients start at the minimum vertex (0)
        let mut min_to_max = Edge::new(&gradients, &min, &max, 0);
        // edge that goes from top to middle, gradients start at the minimum vertex (0)
        let mut min_to_mid = Edge::new(&gradients, &min, &mid, 0);
        // edge that goes from middle to bottom, gradients start at the middle vertex (1)
        let mut mid_to_max = Edge::new(&gradients, &mid, &max, 1);

        // draw edges:

        // first half of the triangle (before the mid vertex)
        //
        // min .
        //  .------.
        //   .------mid
        //    .
        //     .
        //      max

        self.scan_edges(&gradients, &mut min_to_max, &mut min_to_mid, handedness, &material);

        // second half of the triangle (after the mid vertex)
        //
        // min
        //  .
        //   .------mid
        //    .----.
        //     .-.
        //      max

        self.scan_edges(&gradients, &mut min_to_max, &mut mid_to_max, handedness, &material);
    }

    pub fn scan_edges(&mut self, gradients: &Gradients, edge_a: &mut Edge, edge_b: &mut Edge, handedness: bool, material: &Material) {
        // all edges must be draw from left to right
        let mut left = edge_a;
        let mut right = edge_b;

        // # debug: show edges that change handedness
        // let mut debug_bitmap = Bitmap::new(1, 1);
        // debug_bitmap.fill(&Color::BLACK);
        // let mut bitmap = bitmap;
        // if handedness { bitmap = &debug_bitmap; }

        // if we don't swap then the only things we draw are the longest left edge triangles
        if handedness {
            mem::swap(&mut left, &mut right);
        }

        // find min and max y positions that both edges have so that we can draw scan lines in sync
        //
        // min
        //  .
        //   .------mid (y_start)
        //    .    .
        //     . .
        //      max (y_end)

        let y_start = left.y_start.max(right.y_start);
        let y_end = left.y_end.min(right.y_end);

        // all scan lines are drawn from left to right
        // so the sorting in the previous block was necessary
        for y in y_start..y_end {
            // // # debug: see how scan lines are drawn
            // if self.debug.scanline_fill {
            //     let v = (y as f32 - y_start as f32) / (y_end as f32 - y_start as f32);
            //     let mut bitmap = Bitmap::new(1, 1);
            //     bitmap.fill(&Color::newf(v, v, v, 1.0));

            //     let material = Material::new(false, Rc::new(Box::new(bitmap)));

            //     self.draw_scan_line(&gradients, &left, &right, y, &material);
            // } else {
            //     self.draw_scan_line(&gradients, &left, &right, y, &material)
            // }

            // // # debug: draw wireframe for the real triangles
            // let x_min = left.x.ceil() as u32;
            // let x_max = right.x.ceil() as u32;
            // let color = Color::newf(0.0, 1.0, 0.0, 0.75);
            // if y == y_start || y == y_end {
            //     for x in x_min..x_max {
            //         self.color_buffer.set_pixel(x, y, &color);
            //     }
            // } else {
            //     self.color_buffer.set_pixel(x_min, y, &color);
            //     self.color_buffer.set_pixel(x_max, y, &color);
            // }

            self.draw_scan_line(&gradients, &left, &right, y, &material);

            // step to the next pixel on both edges
            left.step();
            right.step();
        }
    }

    // fn step<T: AddAssign + Copy + Clone>(
    //     edge_step: &crate::graphics::edge::Step<T>,
    //     gradient_step: &crate::graphics::gradients::Step<T>,
    //     x_prestep: f32,
    // ) -> f32 {
    //     0.0
    // }

    pub fn draw_scan_line(&mut self, gradients: &Gradients, left: &Edge, right: &Edge, y: u32, material: &Material) {
        // fill convention: if the pixel center is inside the shape it's drawn, otherwise it isn't
        let x_min = left.x.ceil() as u32;
        let x_max = right.x.ceil() as u32; // not inclusive so ceil is fine
        let x_prestep = x_min as f32 - left.x;

        // define some gradient lerp values for the current scan line
        let mut tex_coord_x = left.texcoords.value.x + gradients.texcoords.step.x.x * x_prestep;
        let mut tex_coord_y = left.texcoords.value.y + gradients.texcoords.step.x.y * x_prestep;
        let mut one_over_z = left.one_over_z.value + gradients.one_over_z.step.x * x_prestep;
        let mut depth = left.depth.value + gradients.depth.step.x * x_prestep;
        let mut light_amt = left.light_amp.value + gradients.light_amt.step.x * x_prestep;

        // @todo: remove this...
        // let something = Self::step(&left.depth, &gradients.depth.step, x_prestep);

        // draw min edge
        // if x_max - x_min > 0 {
        //     let z = 1.0 / one_over_z;
        //     let src_x = ((tex_coord_x * z) * (bitmap.width - 1) as f32 + 0.5) as u32;
        //     let src_y = ((tex_coord_y * z) * (bitmap.height - 1) as f32 + 0.5) as u32;

        //     // copy the pixel from the bitmap
        //     let texel = bitmap.get_pixel(src_x, src_y);
        //     self.color_buffer.set_pixel(x_min, y, &texel);
        // }

        for x in x_min..x_max {
            // get the flat index to find the pixel in the depth buffer
            let index = (x + y * self.width) as usize;

            // make sure the pixel is closer to the screen than whatever is currently in the depth buffer
            if depth < self.depth_buffer[index] {
                // set the z buffer value
                self.depth_buffer[index] = depth;

                // we undo perspective texture mapping and get the correct uv from the texture for the current pixel
                let z = 1.0 / one_over_z;
                let src_x = ((tex_coord_x * z) * (material.bitmap.width - 1) as f32 + 0.5) as u32;
                let src_y = ((tex_coord_y * z) * (material.bitmap.height - 1) as f32 + 0.5) as u32;

                // copy the pixel from the bitmap
                let mut tex_pixel = material.bitmap.get_pixel(src_x, src_y);

                // light it up
                if material.light {
                    tex_pixel.r = (tex_pixel.r as f32 * light_amt) as u8;
                    tex_pixel.g = (tex_pixel.g as f32 * light_amt) as u8;
                    tex_pixel.b = (tex_pixel.b as f32 * light_amt) as u8;
                }

                // finally set pixel in the color buffer
                self.color_buffer.set_pixel(x, y, &tex_pixel);
            } else {
                // # debug: we can draw a blue pixel when the depth test fails what it means is that
                // we tried to draw something in a screen position where the z-buffer already has a lower value
                if self.debug.depth_miss {
                    if (x as u32 + y) % 2 == 0 {
                        self.color_buffer.set_pixel(x as u32, y, &Color::BLUE);
                    }
                }
            }

            // # debug: draw the depth buffer
            if self.debug.depth {
                self.color_buffer.set_pixel(x as u32, y, &Color::newf(depth, depth, 1.0 - depth, 0.5));
            }

            // # debug: draw dithered fill over the shape
            if self.debug.solid {
                if (x as u32) % 4 == 0 && (y as u32) % 4 == 0 {
                    self.color_buffer.set_pixel(x as u32, y, &Color::GREEN);
                }
            }

            // step all gradient values for this scan line
            tex_coord_x += gradients.texcoords.step.x.x;
            tex_coord_y += gradients.texcoords.step.x.y;
            one_over_z += gradients.one_over_z.step.x;
            depth += gradients.depth.step.x;
            light_amt += gradients.light_amt.step.x;
        }

        // # debug: draw wireframe
        // @todo: fix the pixel bleed on opposite edge
        // if self.debug.wireframe {
        //     let color = Color::newf(1.0, 1.0, 1.0, 0.3);
        //     self.color_buffer.set_pixel(x_min, y, &color);
        //     self.color_buffer.set_pixel(x_max, y, &color);
        // }
    }

    // todo: remove: instead have the renderer return the color buffer
    pub fn present(&mut self) {
        // copy pixels from bitmap to the `pixels` frame
        let frame = self.pixels.get_frame_mut();

        for (i, pixel) in frame.chunks_mut(4).enumerate() {
            let byte_index = i * 4;
            // take a slice of 4 bytes from the color_buffer and move them into the frame
            // color_buffer:[RGBA] -> frame:[RGBA]
            pixel.copy_from_slice(&self.color_buffer[byte_index..byte_index + 4]);
        }

        self.pixels.render().unwrap();
    }
}
