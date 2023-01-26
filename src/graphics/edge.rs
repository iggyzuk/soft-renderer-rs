use std::ops::{Add, AddAssign, Mul};

use crate::math::linear_algebra::vector::Vector4;

use super::{
    gradients::{Gradient, Gradients},
    vertex::Vertex,
};

// move along the edge y and some x and increase steppable values as you go along
#[derive(Debug)]
pub struct Edge {
    pub x: f32,                       // current x of the edge
    pub x_step: f32,                  // how much to step on the x-axis everytime we step down on y-axis
    pub y_start: u32,                 // y-start of the edge
    pub y_end: u32,                   // y-end of the edge
    pub texcoords: Stepable<Vector4>, // texture-coordinate start and step values
    pub one_over_z: Stepable<f32>,    // ...
    pub depth: Stepable<f32>,         // ...
    pub light_amp: Stepable<f32>,     // ...
}

impl Edge {
    // `start_index` is also the `min_y_vertex`/`top_to_bot`
    pub fn new(gradients: &Gradients, start: &Vertex, end: &Vertex, start_index: usize) -> Self {
        // apply our fill convention to the start and end y screen positions
        let y_start = start.position.y.ceil() as u32;
        let y_end = end.position.y.ceil() as u32;

        // calculate the pixel distance between x start and y end, the same for y
        let y_dist = end.position.y - start.position.y;
        let x_dist = end.position.x - start.position.x;

        // x: 0---------------5
        //    |
        //    |
        //    |
        // y: 3
        //
        // edge ratio of x to y, based on vertex distances
        // example 5/3 = 1.666
        //
        // basically x_step is how much our x increases for every increase in the y
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
        let texcoords = Stepable::new(&gradients.texcoords, start_index, x_prestep, y_prestep, x_step);
        let one_over_z = Stepable::new(&gradients.one_over_z, start_index, x_prestep, y_prestep, x_step);
        let depth = Stepable::new(&gradients.depth, start_index, x_prestep, y_prestep, x_step);
        let light_amp = Stepable::new(&gradients.light_amt, start_index, x_prestep, y_prestep, x_step);

        // and finally return the newly constructed edge
        Self {
            x,
            x_step,
            y_start: (y_start as u32),
            y_end: (y_end as u32),
            texcoords,
            one_over_z,
            depth,
            light_amp,
        }
    }

    pub fn step(&mut self) {
        // move forward on the current scan line
        self.x += self.x_step;

        // move forward all gradients
        self.texcoords.step();
        self.one_over_z.step();
        self.depth.step();
        self.light_amp.step();
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
    pub fn new(gradient: &Gradient<T>, start_index: usize, x_prestep: f32, y_prestep: f32, x_step: f32) -> Self {
        Self {
            value: gradient.value[start_index] + (gradient.step.x * x_prestep) + (gradient.step.y * y_prestep),
            step: gradient.step.y + gradient.step.x * x_step,
        }
    }
}

impl<T: AddAssign + Copy + Clone> Stepable<T> {
    pub fn step(&mut self) {
        self.value += self.step;
    }
}
