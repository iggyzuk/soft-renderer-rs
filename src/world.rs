use std::rc::Rc;

use crate::app::camera::Camera;
use crate::app::instance::Instance;
use crate::app::mesh_loader::MeshLoader;
use crate::app::renderer::Renderer;
use crate::app::starfield::Starfield;
use crate::graphics::mesh::Mesh;
use crate::graphics::{bitmap::Bitmap, color::Color};
use crate::math::lerp;
use crate::math::linear_algebra::{matrix::Matrix4, vector::Vector4};
use image::EncodableLayout;
use pixels::Pixels;
use rand::Rng;

#[derive(Debug)]
pub enum WorldInputEvent {
    MoveForward,
    MoveBack,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    LookUp,
    LookDown,
    LookLeft,
    LookRight,
}

/// Representation of the application state. In this example, a box will bounce around the screen.
pub struct World {
    width: u32,
    height: u32,
    starfield: Starfield,
    renderer: Renderer,
    camera: Camera,
    projection: Matrix4,
    instances: Vec<Instance>,
    time: f32,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: u32, height: u32, pixels: Pixels) -> Self {
        let aspect_ratio = width as f32 / height as f32;

        let mut world = Self {
            width,
            height,
            starfield: Starfield::new(0, 0.1, 2.0),
            renderer: Renderer::new(width, height, pixels),
            camera: Camera::new(Vector4::new(0.0, -2.0, -2.0, 1.0), Vector4::new(0.0, 0.0, -1.0, 0.0)),
            projection: Matrix4::perspective(100.0, aspect_ratio, 0.1, 10.0),
            instances: Vec::new(),
            time: 0.0,
        };

        // create a checker-board bitmap
        let mut bitmap = Bitmap::new(64, 64);
        for x in 0..bitmap.width {
            for y in 0..bitmap.height {
                let color = match (x + y) % 2 == 0 {
                    true => Color::from_hex(0x555555FF),
                    false => Color::from_hex(0x888888FF),
                };
                bitmap.set_pixel(x, y, &color);
            }
        }

        let bitmap_resource = Rc::new(Box::new(bitmap));

        let mario_mesh = MeshLoader::load("./assets/mario.obj");
        let mario_mesh_resource = Rc::new(Box::new(mario_mesh));

        let mario_image = image::open("./assets/mario.png").unwrap();
        let mut mario_bitmap = Bitmap::new(mario_image.width(), mario_image.height());
        mario_bitmap.pixels = mario_image.as_bytes().into();

        let mario_bitmap_resource = Rc::new(Box::new(mario_bitmap));

        let mario = Instance::new(Rc::clone(&mario_mesh_resource), Rc::clone(&mario_bitmap_resource), false);

        world.instances.push(mario);

        let some_mesh = Self::make_mesh_res("./assets/box.obj");

        let mut instance = world.make_instance(&some_mesh, &bitmap_resource, true);
        instance.transform.translate(0.0, -0.5, 0.0);
        instance.transform.scale(40.0, 0.5, 40.0);
        world.instances.push(instance);

        let mut instance = world.make_instance(&some_mesh, &bitmap_resource, true);
        instance.transform.translate(0.0, 1.0, 3.0);
        world.instances.push(instance);

        // let grid = 50;

        // for x in -grid..grid {
        //     for z in -grid..grid {
        //         let mut instance = world.make_instance(&some_mesh, &bitmap_resource);
        //         instance.transform.scale(0.02, 0.02, 0.02);
        //         instance
        //             .transform
        //             .translate(x as f32 * 4.0, -0.5, z as f32 * 4.0);
        //         world.instances.push(instance);
        //     }
        // }

        // spawn a bunch of marios
        // for x in 0..8 {
        //     let mut instance = world.make_instance(&mario_mesh_resource, &mario_bitmap_resource);
        //     instance.transform.scale(1.0, 1.0, 1.0);

        //     let v = Vector4::new(
        //         rand::thread_rng().gen_range(-1.0..1.0),
        //         rand::thread_rng().gen_range(-1.0..1.0),
        //         rand::thread_rng().gen_range(-1.0..1.0),
        //         0.0,
        //     );

        //     let v = v.normalize() * 4.0;

        //     instance.transform.translate(v.x, v.y, v.z);
        //     world.instances.push(instance);
        // }

        // world.spawn_instance("./assets/turtle.obj", "./assets/turtle.png", 1.0);
        // world.spawn_instance("./assets/car.obj", "./assets/car.png", Vector4::new(3.5, 0.0, 0.0, 0.0), -60.0, 1.0, true);

        // world.spawn_instance("./assets/ghoul.obj", "./assets/ghoul.png", Vector4::new(-3.5, 0.0, 1.0, 0.0), 60.0, 1.0, true);

        // world.spawn_instance(
        //     "./assets/house.obj",
        //     "./assets/house.png",
        //     Vector4::new(-10.0, 0.0, -10.0, 0.0),
        //     30.0,
        //     1.0,
        //     true,
        // );

        // world.spawn_instance(
        //     "./assets/plane.obj",
        //     "./assets/plane.png",
        //     Vector4::new(10.0, 6.0, -10.0, 0.0),
        //     -130.0,
        //     1.0,
        //     true,
        // );

        // create a sky bitmap
        let mut bitmap = Bitmap::new(1, 128);
        for y in 0..bitmap.height {
            let l = lerp(2.0, 0.2, y as f32 / bitmap.height as f32);
            bitmap.set_pixel(0, y, &Color::newf(l * 0.1, l * 0.7, l, 1.0));
        }

        let bitmap_resource = Rc::new(Box::new(bitmap));

        let sky = Self::make_mesh_res("./assets/skydome.obj");
        let instance = world.make_instance(&sky, &bitmap_resource, false);
        world.instances.push(instance);

        // world.spawn_instance(
        //     "./assets/land.obj",
        //     "./assets/war.png",
        //     Vector4::new(0.0, -1.0, 0.0, 0.0),
        //     50.0,
        // );

        // world.spawn_instance("./assets/human.obj", "./assets/mario.png", 10.0);

        // let turtle_mesh = MeshLoader::load("./assets/female.obj");
        // let turtle_mesh_resource = Rc::new(Box::new(turtle_mesh));

        // let turtle_image = image::open("./assets/turtle.png").unwrap();
        // let mut turtle_bitmap = Bitmap::new(turtle_image.width(), turtle_image.height());
        // turtle_bitmap.pixels = turtle_image.as_bytes().into();

        // let turtle_bitmap_resource = Rc::new(Box::new(turtle_bitmap));

        // let mut turtle = Instance::new(
        //     Rc::clone(&turtle_mesh_resource),
        //     Rc::clone(&turtle_bitmap_resource),
        // );

        // turtle.transform.translate(
        //     rand::thread_rng().gen_range(-20.0..20.0),
        //     rand::thread_rng().gen_range(-20.0..20.0),
        //     rand::thread_rng().gen_range(-20.0..20.0),
        // );

        // world.instances.push(turtle);

        // for _ in 0..30 {
        //     // create a basic triangle mesh
        //     // let mesh = Mesh::new(
        //     //     vec![
        //     //         Vertex::new(
        //     //             Vector4::from_random(-5.0, 5.0),
        //     //             Vector4::from_random(-5.0, 5.0),
        //     //             Vector4::from_random(-5.0, 5.0),
        //     //         ),
        //     //         Vertex::new(
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //         ),
        //     //         Vertex::new(
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //             Vector4::from_random(-1.0, 1.0),
        //     //         ),
        //     //     ],
        //     //     vec![0, 1, 2],
        //     // );

        //     let mesh = Mesh::new(
        //         vec![
        //             Vertex::new(
        //                 Vector4::from_xyz(-5.0, 0.0, 5.0),
        //                 Vector4::from_xy(0.0, 0.0),
        //                 Vector4::ONE,
        //             ),
        //             Vertex::new(
        //                 Vector4::from_xy(5.0, 0.0),
        //                 Vector4::from_xy(1.0, 0.0),
        //                 Vector4::ONE,
        //             ),
        //             Vertex::new(
        //                 Vector4::from_xyz(0.5, -5.0, -5.0),
        //                 Vector4::from_xy(1.0, 1.0),
        //                 Vector4::ONE,
        //             ),
        //         ],
        //         vec![0, 2, 1],
        //     );

        //     let mesh_resource = Rc::new(Box::new(mesh));

        //     let mut instance =
        //         Instance::new(Rc::clone(&mesh_resource), Rc::clone(&bitmap_resource));

        //     instance.transform.translate(
        //         rand::thread_rng().gen_range(-20.0..20.0),
        //         rand::thread_rng().gen_range(-20.0..20.0),
        //         rand::thread_rng().gen_range(-20.0..20.0),
        //     );

        //     instance
        //         .transform
        //         .rotate_x(rand::thread_rng().gen_range(-180.0..180.0));
        //     instance
        //         .transform
        //         .rotate_y(rand::thread_rng().gen_range(-180.0..180.0));
        //     instance
        //         .transform
        //         .rotate_z(rand::thread_rng().gen_range(-180.0..180.0));

        //     world.instances.push(instance);
        // }

        world
    }

    /// Update the `World` internal state; bounce the box around the screen.
    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        self.camera.update(dt);
        // for instance in self.instances.iter_mut() {
        //     // instance
        //     //     .transform
        //     //     .translate(self.time.cos() * 0.01, 0.0, self.time.sin() * 0.01);

        //     instance.transform.rotate_y(360.0 / 8.0 * dt);
        // }
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    pub fn draw(&mut self, dt: f32) {
        self.renderer.color_buffer.fill(&Color::newf(0.1, 0.1, 0.1, 1.0));
        self.renderer.clear_depth_buffer();
        // self.starfield.render(&mut self.renderer.color_buffer, dt);

        // calculate view projection matrix
        // let mut m1 = Matrix4::new_identity();
        // m1.translate(1.0, -2.0, -100.0);

        let view_projection = Matrix4::multiply(&self.projection, &self.camera.transform());
        // let view_projection = &self.projection;

        // log::info!("{:?}", self.instances.len());

        let ss_mat = Matrix4::screenspace(self.width as f32, self.height as f32);

        // draw all instances
        for instance in self.instances.iter() {
            instance.draw(&mut self.renderer, &view_projection);
        }

        // let identity = Matrix4::new_identity();
        // dbg!(&self.projection);

        dbg!(&view_projection);

        // # debug: draw all vertices
        for instance in self.instances.iter() {
            let mvp = Matrix4::multiply(&view_projection, &instance.transform);
            let identity = Matrix4::new_identity();
            dbg!(&mvp);
            for v in instance.mesh.vertices.iter() {
                let new_vertex = v.transform(&mvp, &identity);

                // @todo: fix here
                // all vertices say that they are outside of the frustum!
                // if !new_vertex.is_inside_view_frustum() {
                //     continue;
                // }

                // it's not the perspective divide! it happens later... ->
                let ss_pos = new_vertex.transform(&ss_mat, &identity).perspective_divide();

                self.renderer
                    .color_buffer
                    .set_pixel(ss_pos.position.x as u32, ss_pos.position.y as u32, &Color::from_hex(0xFFFFFF55));

                // let color = Color::from_hex(0xFFFFFF55);
                // for i in -1..=1 {
                //     self.renderer.color_buffer.set_pixel(
                //         (ss_pos.position.x as i32 + i) as u32,
                //         (ss_pos.position.y as i32 + i) as u32,
                //         &color,
                //     );

                //     self.renderer.color_buffer.set_pixel(
                //         (ss_pos.position.x as i32 - i) as u32,
                //         (ss_pos.position.y as i32 + i) as u32,
                //         &color,
                //     );
                // }

                // self.renderer.color_buffer.set_pixel(
                //     ss_pos.position.x as u32,
                //     ss_pos.position.y as u32,
                //     &Color::from_hex(0xFFFFFFAA),
                // );

                // for i in 1..=5 {
                //     self.renderer.color_buffer.set_pixel(
                //         ss_pos.position.x as u32 + i as u32,
                //         ss_pos.position.y as u32 as u32,
                //         &Color::from_hex(0xFF0000AA),
                //     );
                //     self.renderer.color_buffer.set_pixel(
                //         ss_pos.position.x as u32 as u32,
                //         ss_pos.position.y as u32 - i as u32,
                //         &Color::from_hex(0x00FF00AA),
                //     );
                //     self.renderer.color_buffer.set_pixel(
                //         ss_pos.position.x as u32 + i as u32,
                //         ss_pos.position.y as u32 - i as u32,
                //         &Color::from_hex(0x0000FFAA),
                //     );
                // }
            }
        }

        // example: manual scan buffer triangle
        let mut sb = crate::graphics::scan_buffer::ScanBuffer::new();
        for x in 0..100 {
            sb.push(100 - x, x + 100);
        }
        sb.draw(&mut self.renderer.color_buffer);

        self.renderer.present();
    }

    pub fn input(&mut self, event: WorldInputEvent, dt: f32) {
        println!("{:?}", event);

        let move_speed = 75.0;
        let look_speed = 3.0;

        match event {
            WorldInputEvent::MoveForward => self.camera.speed -= move_speed * dt,
            WorldInputEvent::MoveBack => self.camera.speed += move_speed * dt,
            WorldInputEvent::MoveUp => self.camera.v_speed -= move_speed * dt,
            WorldInputEvent::MoveDown => self.camera.v_speed += move_speed * dt,
            WorldInputEvent::MoveLeft => self.camera.h_speed -= move_speed * dt,
            WorldInputEvent::MoveRight => self.camera.h_speed += move_speed * dt,

            WorldInputEvent::LookUp => self.camera.v_angle += look_speed * dt,
            WorldInputEvent::LookDown => self.camera.v_angle -= look_speed * dt,
            WorldInputEvent::LookLeft => self.camera.h_angle -= look_speed * dt,
            WorldInputEvent::LookRight => self.camera.h_angle += look_speed * dt,
        }
    }

    pub fn spawn_instance_rand(&mut self, mesh_path: &str, bitmap_path: &str, scale: f32) {
        let mesh_res = Self::make_mesh_res(mesh_path);
        let bitmap_res = Self::make_bitmap_res(bitmap_path);

        let mut instance = Instance::new(Rc::clone(&mesh_res), Rc::clone(&bitmap_res), true);

        instance.transform.translate(
            rand::thread_rng().gen_range(-20.0..20.0),
            rand::thread_rng().gen_range(-20.0..20.0),
            rand::thread_rng().gen_range(-20.0..20.0),
        );

        instance.transform.scale(scale, scale, scale);

        self.instances.push(instance);
    }

    pub fn spawn_instance(&mut self, mesh_path: &str, bitmap_path: &str, pos: Vector4, y_angle: f32, scale: f32, light: bool) {
        let mesh_res = Self::make_mesh_res(mesh_path);
        let bitmap_res = Self::make_bitmap_res(bitmap_path);

        let mut instance = Instance::new(Rc::clone(&mesh_res), Rc::clone(&bitmap_res), light);

        instance.transform.translate(pos.x, pos.y, pos.z);
        instance.transform.rotate_y(y_angle);
        instance.transform.scale(scale, scale, scale);

        self.instances.push(instance);
    }

    pub fn make_instance(&mut self, mesh_res: &Rc<Box<Mesh>>, bitmap_res: &Rc<Box<Bitmap>>, light: bool) -> Instance {
        Instance::new(Rc::clone(&mesh_res), Rc::clone(&bitmap_res), light)
    }

    pub fn make_mesh_res(path: &str) -> Rc<Box<Mesh>> {
        let mesh = MeshLoader::load(path);
        Rc::new(Box::new(mesh))
    }

    pub fn make_bitmap_res(path: &str) -> Rc<Box<Bitmap>> {
        let image = image::open(path).unwrap();
        let mut bitmap = Bitmap::new(image.width(), image.height());
        bitmap.pixels = image.to_rgba8().as_bytes().into();
        Rc::new(Box::new(bitmap))
    }
}

// const WIDTH: u32 = 640;
// const HEIGHT: u32 = 480;
// const BOX_SIZE: i16 = 64;

// /// Representation of the application state. In this example, a box will bounce around the screen.
// pub struct World {
//     pub box_x: i16,
//     pub box_y: i16,
//     pub velocity_x: i16,
//     pub velocity_y: i16,
// }

// /// Update the `World` internal state; bounce the box around the screen.
// pub fn update(&mut self) {
//     if self.box_x <= 0 || self.box_x + BOX_SIZE > WIDTH as i16 {
//         self.velocity_x *= -1;
//     }
//     if self.box_y <= 0 || self.box_y + BOX_SIZE > HEIGHT as i16 {
//         self.velocity_y *= -1;
//     }

//     self.box_x += self.velocity_x;
//     self.box_y += self.velocity_y;
// }

// /// Draw the `World` state to the frame buffer.
// ///
// /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
// pub fn draw(&self, frame: &mut [u8]) {
//     for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
//         let x = (i % WIDTH as usize) as i16;
//         let y = (i / WIDTH as usize) as i16;

//         let inside_the_box = x >= self.box_x
//             && x < self.box_x + BOX_SIZE
//             && y >= self.box_y
//             && y < self.box_y + BOX_SIZE;

//         let rgba = if inside_the_box {
//             [0x5e, 0x48, 0xe8, 0xff]
//         } else {
//             [0x48, 0xb2, 0xe8, 0xff]
//         };

//         pixel.copy_from_slice(&rgba);
//     }
// }
