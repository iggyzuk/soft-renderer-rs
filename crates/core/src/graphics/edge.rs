use std::ops::{Add, AddAssign, Mul};

use crate::math::linear_algebra::vector::Vector4;

use super::{
    gradients::{Gradient, Gradients},
    vertex::Vertex,
};

// move along the edge y and some x and increase steppable values as you go along
// contains varying variables (like OpenGL)
#[derive(Debug)]
#[rustfmt::skip]
pub struct Edge {
    pub y_start: u32,                         // y-start of the edge
    pub y_end: u32,                           // y-end of the edge
    pub x: f32,                               // current x on the edge 
    pub x_step: f32,                          // how much to step on the x-axis every time we step down on y-axis
    pub position: Stepable<Vector4>,          // position
    pub texcoords: Stepable<Vector4>,         // texture-coordinate start and step values
    pub one_over_z: Stepable<f32>,            // one-over-z start and step values
    pub depth: Stepable<f32>,                 // depth start and step values
    pub light_amp: Stepable<f32>,             // light-amp start and step values
    pub shadow_map_coords: Stepable<Vector4>, // interpolated shadow-map-coordinates
}

impl Edge {
    // `start_index` is also the `min_y_vertex`/`top_to_bot`
    pub fn new(gradients: &Gradients, start: &Vertex, end: &Vertex, start_index: usize) -> Self {
        // apply our fill convention to the start and end y screen positions
        let y_start = start.position.y.ceil() as u32;
        let y_end = end.position.y.ceil() as u32;

        // calculate the vertex distances (y2 - y1) and (x2 - x1)
        let y_dist = end.position.y - start.position.y;
        let x_dist = end.position.x - start.position.x;

        // find the slope from x and y start to end distances
        //
        // x: 0---------------5
        //    |
        //    |
        //    |
        // y: 3
        //
        // example 5/3 = 1.666
        //
        // x_step is the slope x over y (for every 1 unit of y how much we need to step on x)
        // from example: x: 1.666, y: 1.0
        let x_step = x_dist / y_dist;

        // y_prestep is the distance from the vertex to the first scan line y
        //
        //       x (off pixel by a bit)
        //     / | <---- this distance is y_prestep
        //  scan x <---- moves it down here accurately on the scan line
        let y_prestep = y_start as f32 - start.position.y;

        // now move the x right to the edge
        // y_prestep * x_step is a ratio so it aligns nicely
        let x = start.position.x + y_prestep * x_step;

        // now we can calculate what the x_prestep is so that gradients can utilize them later
        //
        //     /
        //    x === x   <--- where x was and where it moved
        //   /
        //  / x_prestep is the distance `===`
        let x_prestep = x - start.position.x;

        // construct steps with gradients with initial values
        #[rustfmt::skip]
        let position = Stepable::new(&gradients.position, start_index, x_prestep, y_prestep, x_step);
        #[rustfmt::skip]
        let texcoords = Stepable::new(&gradients.texcoords, start_index, x_prestep, y_prestep, x_step);
        #[rustfmt::skip]
        let one_over_z = Stepable::new(&gradients.one_over_z, start_index, x_prestep, y_prestep, x_step);
        #[rustfmt::skip]
        let depth = Stepable::new(&gradients.depth, start_index, x_prestep, y_prestep, x_step);
        #[rustfmt::skip]
        let light_amp = Stepable::new(&gradients.light_amt, start_index, x_prestep, y_prestep, x_step);
        #[rustfmt::skip]
        let shadow_map_coords = Stepable::new(&gradients.shadow_map_coords, start_index, x_prestep, y_prestep, x_step);

        // and finally return the newly constructed edge
        return Self {
            x,
            x_step,
            y_start: (y_start as u32),
            y_end: (y_end as u32),
            position,
            texcoords,
            one_over_z,
            depth,
            light_amp,
            shadow_map_coords,
        };
    }

    // called once we go down a scan line y+1
    pub fn step(&mut self) {
        // move forward on the current scan line (e.g. x+=1.666)
        self.x += self.x_step;

        // move forward all gradients
        self.position.step();
        self.texcoords.step();
        self.one_over_z.step();
        self.depth.step();
        self.light_amp.step();
        self.shadow_map_coords.step();
    }
}

#[derive(Debug)]
pub struct Stepable<T>
where
    T: AddAssign + Copy + Clone,
{
    pub value: T, // initial value starts with pre-stepped x and y this moves us to the center of the pixel
    pub step: T,  // how much to move by with each step
}

impl<T> Stepable<T>
where
    T: AddAssign + Copy + Clone + Mul<f32, Output = T> + Add<T, Output = T>,
{
    pub fn new(
        gradient: &Gradient<T>,
        start_index: usize,
        x_prestep: f32,
        y_prestep: f32,
        x_step: f32,
    ) -> Self {
        return Self {
            #[rustfmt::skip]
            // set the initial value and make sure to offset its gradient to the center with x and y pre_steps
            value: gradient.value[start_index] + (gradient.step.x * x_prestep) + (gradient.step.y * y_prestep),
            // set the edge step value: for every y it has a bit of x
            step: gradient.step.y + gradient.step.x * x_step,
        };
    }
}

impl<T: AddAssign + Copy + Clone> Stepable<T> {
    pub fn step(&mut self) {
        self.value += self.step;
    }
}
