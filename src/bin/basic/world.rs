use std::rc::Rc;

use core::app::camera::Camera;
use core::app::instance::Instance;
use core::app::mesh_loader::load_mesh;
use core::app::renderer::Renderer;
use core::graphics::light::Light;
use core::graphics::mesh::Mesh;
use core::graphics::vertex::Vertex;
use core::graphics::{bitmap::Bitmap, color::Color};
use core::math::lerp;
use core::math::{Matrix4, Vector4};
use image::EncodableLayout;
use rand::Rng;

pub struct World {
    width: u32,
    height: u32,
    renderer: Renderer,
    shadow_renderer: Renderer,
    camera: Camera,
    projection: Matrix4,
    instances: Vec<Instance>,
    time: f32,
}

impl World {
    /// Create a new `World` instance that can draw a moving box.
    pub fn new(width: u32, height: u32) -> Self {
        let aspect_ratio = width as f32 / height as f32;

        let mut world = Self {
            width,
            height,
            renderer: Renderer::new(width * 2, height * 2),
            shadow_renderer: Renderer::new(width * 2, height * 2),
            camera: Camera::new(
                Vector4::new(0.0, 2.0, 2.0, 1.0),
                Vector4::new(0.0, 0.0, -1.0, 0.0),
            ),
            projection: Matrix4::perspective(100.0, aspect_ratio, 0.1, 100.0),
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

        let mario_mesh = load_mesh("./assets/mario.obj");
        let mario_mesh_resource = Rc::new(Box::new(mario_mesh));

        let mario_image = image::open("./assets/mario.png").unwrap();
        let mut mario_bitmap = Bitmap::new(mario_image.width(), mario_image.height());
        mario_bitmap.pixels = mario_image.as_bytes().into();

        let mario_bitmap_resource = Rc::new(Box::new(mario_bitmap));

        let mario = Instance::new(
            Rc::clone(&mario_mesh_resource),
            Rc::clone(&mario_bitmap_resource),
            true,
        );

        world.instances.push(mario);

        let box_mesh_res = Self::make_mesh_res("./assets/box.obj");

        // ground
        let mut instance = world.make_instance(&box_mesh_res, &bitmap_resource, false);
        instance.transform.translate(0.0, -0.5, 0.0);
        instance.transform.scale(40.0, 0.5, 40.0);
        world.instances.push(instance);

        // simple box
        // create a checker-board bitmap
        let mut bitmap = Bitmap::new(8, 8);
        for x in 0..bitmap.width {
            for y in 0..bitmap.height {
                let color = match (x + y) % 2 == 0 {
                    true => Color::from_hex(0xFEDB00FF),
                    false => Color::from_hex(0xFF9536FF),
                };
                bitmap.set_pixel(x, y, &color);
            }
        }
        let bitmap_resource = Rc::new(Box::new(bitmap));
        let triangle_mesh = Mesh::new(
            vec![
                Vertex::new(
                    Vector4::new(-2.0, 0.0, 0.0, 1.0),
                    Vector4::new(0.0, 1.0, 0.0, 0.0),
                    Vector4::FORWARD,
                ),
                Vertex::new(
                    Vector4::new(2.0, 0.0, 0.0, 1.0),
                    Vector4::new(1.0, 1.0, 0.0, 0.0),
                    Vector4::FORWARD,
                ),
                Vertex::new(
                    Vector4::new(0.0, 2.0, 0.0, 1.0),
                    Vector4::new(0.5, 0.0, 0.0, 0.0),
                    Vector4::FORWARD,
                ),
            ],
            vec![0, 1, 2],
        );
        let triangle_resource = Rc::new(Box::new(triangle_mesh));
        let mut instance = world.make_instance(&triangle_resource, &bitmap_resource, true);
        instance.transform.translate(0.0, 1.0, 5.0);
        world.instances.push(instance);

        // spawn some models
        world.spawn_instance(
            "./assets/car.obj",
            "./assets/car.png",
            Vector4::new(3.5, 0.0, 0.0, 0.0),
            -60.0,
            1.0,
            true,
        );

        world.spawn_instance(
            "./assets/ghoul.obj",
            "./assets/ghoul.png",
            Vector4::new(-3.5, 0.0, 1.0, 0.0),
            60.0,
            1.0,
            true,
        );

        world.spawn_instance(
            "./assets/house.obj",
            "./assets/house.png",
            Vector4::new(-10.0, 0.0, -10.0, 0.0),
            30.0,
            1.0,
            true,
        );

        world.spawn_instance(
            "./assets/plane.obj",
            "./assets/plane.png",
            Vector4::new(10.0, 6.0, -10.0, 0.0),
            -130.0,
            1.0,
            true,
        );

        world.spawn_instance(
            "./assets/ship.obj",
            "./assets/pirates.png",
            Vector4::new(-10.0, 0.0, 10.0, 0.0),
            130.0,
            1.0,
            true,
        );

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

        return world;
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        self.camera.update(dt);

        // # example: set the ground bitmap to use the same pixels as what the renderer sees
        // let mut render_bitmap = Box::new(Bitmap::new(self.width, self.height));
        // render_bitmap.pixels = self.renderer.color_buffer.pixels.clone();
        // self.instances[1].bitmap = Rc::new(render_bitmap);

        // # example: motion
        // for instance in self.instances.iter_mut() {
        //     instance
        //         .transform
        //         .translate(self.time.cos() * 0.01, 0.0, self.time.sin() * 0.01);

        //     instance.transform.rotate_y(360.0 / 8.0 * dt);
        // }

        for instance in self.instances.iter_mut().take(1) {
            // instance
            //     .transform
            //     .translate(self.time.cos() * 0.01, 0.0, self.time.sin() * 0.01);

            instance.transform.rotate_y(360.0 / 4.0 * dt);
        }
    }

    pub fn draw(&mut self, frame: &mut [u8], dt: f32) {
        // fill and clear buffers
        #[rustfmt::skip]
        // self.renderer.color_buffer.fill(&Color::newf(0.1, 0.1, 0.1, 1.0));
        self.renderer.clear_depth_buffer();

        // # shadow mapping experiment
        // let shadow_projection = Matrix4::perspective(100.0, self.width as f32 / self.height as f32, 0.1, 100.0);
        let range = 10.0;
        let aspect = self.width as f32 / self.height as f32;
        let shadow_projection =
            Matrix4::orthographic(-range * aspect, range * aspect, -range, range, 25.0, -5.0);
        // let shadow_projection = Matrix4::perspective(130.0, self.width as f32 / self.height as f32, 0.1, 100.0);

        // let mut shadow_light_transform = Matrix4::new_identity();
        // // #[rustfmt::skip]
        // // shadow_light_transform.look_at(self.camera.position, self.camera.position + self.camera.direction, Vector4::UP);
        // let follow = Vector4::new(self.camera.position.x, 0.0, self.camera.position.z, 0.0);
        // shadow_light_transform.look_at(
        //     follow,
        //     follow + Vector4::new(-0.4, -0.6, -0.3, 0.0).normalized(),
        //     Vector4::UP,
        // );
        // // shadow_light_transform.look_at(Vector4::new(1.58, 4.52, 2.7, 0.0), Vector4::new(-0.5, -0.8, -0.6, 0.0), Vector4::UP);

        // // dbg!(self.camera.position);
        // // dbg!(self.camera.direction);

        // let shadow_view_projection = Matrix4::multiply(&shadow_projection, &shadow_light_transform);

        // // shadow-map: draw all instances
        // self.shadow_renderer.clear_depth_buffer();
        // for instance in self.instances.iter() {
        //     instance.draw(&mut self.shadow_renderer, &shadow_view_projection, None);
        // }

        // let shadow_depth = self.shadow_renderer.depth_buffer.clone();
        // let mut shadow_bitmap =
        //     Bitmap::new(self.shadow_renderer.width, self.shadow_renderer.height);
        // for (i, value) in shadow_bitmap.chunks_exact_mut(4).enumerate() {
        //     value[0] = (shadow_depth[i] * 255.0) as u8;
        //     value[1] = (shadow_depth[i] * 255.0) as u8;
        //     value[2] = (shadow_depth[i] * 255.0) as u8;
        //     value[3] = 255;
        // }

        // let light = Light::new(
        //     shadow_view_projection,
        //     shadow_light_transform,
        //     shadow_bitmap,
        // );

        let view_projection = Matrix4::multiply(&self.projection, &self.camera.transform());

        // draw all instances
        for instance in self.instances.iter() {
            instance.draw(&mut self.renderer, &view_projection, Option::None);
        }

        // # debug: draw all vertices
        // let screenspace = Matrix4::screenspace(self.width as f32, self.height as f32);
        // for instance in self.instances.iter() {
        //     let mvp = Matrix4::multiply(&view_projection, &instance.transform);
        //     // dbg!(&self.camera.transform());
        //     let identity = Matrix4::new_identity();
        //     // dbg!(&mvp);
        //     for v in instance.mesh.vertices.iter() {
        //         let new_vertex = v.transform(&mvp, &identity);

        //         // @todo: fix here
        //         // all vertices say that they are outside of the frustum!
        //         if !new_vertex.is_inside_view_frustum() {
        //             continue;
        //         }

        //         // it's not the perspective divide! it happens later... ->
        //         let ss_pos = new_vertex
        //             .transform(&ss_mat, &identity)
        //             .perspective_divide();

        //         self.renderer.color_buffer.set_pixel(
        //             ss_pos.position.x as u32,
        //             ss_pos.position.y as u32,
        //             &Color::from_hex(0xFFFFFF55),
        //         );

        //         // let color = Color::from_hex(0xFFFFFF55);
        //         // for i in -1..=1 {
        //         //     self.renderer.color_buffer.set_pixel(
        //         //         (ss_pos.position.x as i32 + i) as u32,
        //         //         (ss_pos.position.y as i32 + i) as u32,
        //         //         &color,
        //         //     );

        //         //     self.renderer.color_buffer.set_pixel(
        //         //         (ss_pos.position.x as i32 - i) as u32,
        //         //         (ss_pos.position.y as i32 + i) as u32,
        //         //         &color,
        //         //     );
        //         // }

        //         // self.renderer.color_buffer.set_pixel(
        //         //     ss_pos.position.x as u32,
        //         //     ss_pos.position.y as u32,
        //         //     &Color::from_hex(0xFFFFFFAA),
        //         // );

        //         // for i in 1..=5 {
        //         //     self.renderer.color_buffer.set_pixel(
        //         //         ss_pos.position.x as u32 + i as u32,
        //         //         ss_pos.position.y as u32 as u32,
        //         //         &Color::from_hex(0xFF0000AA),
        //         //     );
        //         //     self.renderer.color_buffer.set_pixel(
        //         //         ss_pos.position.x as u32 as u32,
        //         //         ss_pos.position.y as u32 - i as u32,
        //         //         &Color::from_hex(0x00FF00AA),
        //         //     );
        //         //     self.renderer.color_buffer.set_pixel(
        //         //         ss_pos.position.x as u32 + i as u32,
        //         //         ss_pos.position.y as u32 - i as u32,
        //         //         &Color::from_hex(0x0000FFAA),
        //         //     );
        //         // }
        //     }
        // }

        // # example: manual scan buffer triangle
        // let mut sb = crate::graphics::scan_buffer::ScanBuffer::new();
        // for x in 0..100 {
        //     sb.push(100 - x, x + 100);
        // }
        // sb.draw(&mut self.renderer.color_buffer);

        // let scale = 8;
        // for x in 0..self.shadow_renderer.width / scale {
        //     for y in 0..self.shadow_renderer.height / scale {
        //         let index = (x * 4 * 4 + y * 4 * self.width * scale) as usize;
        //         let d = self.shadow_renderer.depth_buffer[index];
        //         self.renderer
        //             .color_buffer
        //             .set_pixel(x, y, &Color::newf(d, d, d, 1.0));
        //     }
        // }

        // # debug: show depth buffer
        // for x in 0..self.width / 4 {
        //     for y in 0..self.height / 4 {
        //         let index = (x * 4 + y * 4 * self.width) as usize;
        //         let d = self.renderer.depth_buffer[index] as f32;
        //         self.renderer
        //             .color_buffer
        //             .set_pixel(x, y, &Color::newf(d, d, d, 1.0))
        //     }
        // }

        // let rgb_bytes = self
        //     .renderer
        //     .color_buffer
        //     .chunks(4)
        //     .map(|x| &x[0..4])
        //     .flatten()
        //     .map(|x| *x)
        //     .collect::<Vec<u8>>();

        // let block = xbr::x2(xbr::Block::new(
        //     rgb_bytes,
        //     self.renderer.color_buffer.width,
        //     self.renderer.color_buffer.height,
        // ));

        // let colors = block.bytes;
        // // let colors = rgb_bytes;
        // // let colors = block.colors();

        // for (i, pixel) in frame.chunks_mut(4).enumerate() {
        //     let byte_index = i * 4;
        //     // take a slice of 4 bytes from the color_buffer and move them into the frame
        //     // color_buffer:[RGBA] -> frame:[RGBA]
        //     let color = &[
        //         colors[byte_index],
        //         colors[byte_index + 1],
        //         colors[byte_index + 2],
        //         255,
        //     ];
        //     pixel.copy_from_slice(color);
        // }

        let rgb_bytes = self
            .renderer
            .color_buffer
            .chunks(4)
            .map(|x| &x[0..3])
            .flatten()
            .map(|x| *x)
            .collect::<Vec<u8>>();

        let block = xbr::x2(xbr::Block::new(
            rgb_bytes,
            self.renderer.color_buffer.width,
            self.renderer.color_buffer.height,
        ));

        let colors = block.bytes;

        let mut byte_offset = 0;
        for pixel in frame.chunks_mut(4) {
            // take a slice of 4 bytes from the color_buffer and move them into the frame
            // color_buffer:[RGBA] -> frame:[RGBA]
            let color = &[
                colors[byte_offset],
                colors[byte_offset + 1],
                colors[byte_offset + 2],
                255,
            ];
            byte_offset += 3;
            pixel.copy_from_slice(color);
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

    pub fn spawn_instance(
        &mut self,
        mesh_path: &str,
        bitmap_path: &str,
        pos: Vector4,
        y_angle: f32,
        scale: f32,
        light: bool,
    ) {
        let mesh_res = Self::make_mesh_res(mesh_path);
        let bitmap_res = Self::make_bitmap_res(bitmap_path);

        let mut instance = Instance::new(Rc::clone(&mesh_res), Rc::clone(&bitmap_res), light);

        instance.transform.translate(pos.x, pos.y, pos.z);
        instance.transform.rotate_y(y_angle);
        instance.transform.scale(scale, scale, scale);

        self.instances.push(instance);
    }

    pub fn make_instance(
        &mut self,
        mesh_res: &Rc<Box<Mesh>>,
        bitmap_res: &Rc<Box<Bitmap>>,
        light: bool,
    ) -> Instance {
        Instance::new(Rc::clone(&mesh_res), Rc::clone(&bitmap_res), light)
    }

    pub fn make_mesh_res(path: &str) -> Rc<Box<Mesh>> {
        let mesh = load_mesh(path);
        Rc::new(Box::new(mesh))
    }

    pub fn make_bitmap_res(path: &str) -> Rc<Box<Bitmap>> {
        let image = image::open(path).unwrap();
        let mut bitmap = Bitmap::new(image.width(), image.height());
        bitmap.pixels = image.to_rgba8().as_bytes().into();
        Rc::new(Box::new(bitmap))
    }

    pub fn handle_event(&mut self, event: &winit::event::Event<()>) {
        self.camera.handle_event(event);
    }
}
