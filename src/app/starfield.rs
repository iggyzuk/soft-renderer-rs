// class StarsField {

//     class Star {
//     public:
//         Star(uint id, float x, float y, float z, Color color) : color(color) {
//             this->id = id;
//             this->x  = x;
//             this->y  = y;
//             this->z  = z;
//         }
//         uint id;
//         float x,y,z;
//         Color color;
//     };

// public:
//     StarsField(int numStars, float spread, float speed) {
//         this->spread = spread;
//         this->speed = speed;

//         stars.resize(numStars);
//         for(int i = 0; i < numStars; ++i) {
//             initStar(i);
//         }
//     }
//     ~StarsField() {
//         for(uint i = 0; i < stars.size(); ++i) {
//             delete stars[i];
//         }
//         stars.empty();
//     }
//     void initStar(int index) {
//         float x = 2 * (random() - 0.5f) * spread;
//         float y = 2 * (random() - 0.5f) * spread;
//         float z = (random() + 0.001f) * spread;
//         stars[index] = new Star(index, x, y, z, Color::Random());
//     }
//     void render(Bitmap& target, const float dt) {

//         float halfFOV = tan((130.0f / 2.0f) * (PI / 180.0f));

//         uint halfWidth = target.width / 2;
//         uint halfHeight = target.height / 2;

//         for(auto& star : stars) {
//             star->z -= speed * dt;
//             if(star->z <= 0.0f) initStar(star->id);

//             int x = (star->x / (star->z * halfFOV)) * halfWidth + halfWidth;
//             int y = (star->y / (star->z * halfFOV)) * halfHeight + halfHeight;

//             if(x <= 0 || x > target.width || y <= 0 || y > target.height) {
//                 initStar(star->id);
//             } else {
//                 target.setPixel(x, y, star->color);
//             }

//         }
//     }
// private:
//     float speed;
//     float spread;

//     std::vector<Star*> stars;
// };

use rand::Rng;

use crate::{
    graphics::{bitmap::Bitmap, color::Color},
    math::PI,
};

pub struct Starfield {
    stars: Vec<Star>,
    speed: f32,
    spread: f32,
}

pub struct Star {
    id: usize,
    x: f32,
    y: f32,
    z: f32,
    alive: bool,
    color: Color,
}

impl Starfield {
    pub fn new(star_count: usize, speed: f32, spread: f32) -> Self {
        let mut starfield = Self {
            speed,
            spread,
            stars: Vec::with_capacity(star_count),
        };

        for i in 0..star_count {
            starfield.add_star(i);
        }

        starfield
    }

    pub fn add_star(&mut self, index: usize) {
        let (x, y, z) = Star::get_init_vars(self.spread);
        self.stars
            .push(Star::new(index, x, y, z, Color::from_random(0x00, 0xFF)));
    }

    pub fn render(&mut self, bitmap: &mut Bitmap, dt: f32) {
        let half_fov = ((130.0 / 2.0) * (PI / 180.0)).tan();

        let half_width = bitmap.width as f32 / 2 as f32;
        let half_height = bitmap.height as f32 / 2 as f32;

        for star in self.stars.iter_mut() {
            star.z -= self.speed * dt;
            if star.z <= 0.0 {
                star.alive = false;
            }

            // going from world-space to screen-space
            // -1w to 1w -> 0vp to 1vp -> 0px to 800px

            // divide by z makes it "3D"
            // move everything to the center
            // how much is the "d" (x,y)/d
            // it's a triangle: O/A = tan(ø)=d/z => tan(ø)z=d
            // tan(0)=1

            let x = ((star.x / (star.z * half_fov)) * half_width + half_width) as u32;
            let y = ((star.y / (star.z * half_fov)) * half_height + half_height) as u32;

            if x <= 0 || x > bitmap.width || y <= 0 || y > bitmap.height {
                star.alive = false;
            } else {
                bitmap.set_pixel(x, y, &star.color);
            }
        }

        let spread = self.spread;

        for star in self.stars.iter_mut() {
            if !star.alive {
                let (x, y, z) = Star::get_init_vars(spread);
                star.x = x;
                star.y = y;
                star.z = z;
                star.alive = true;
            }
        }
    }
}

impl Star {
    pub fn new(id: usize, x: f32, y: f32, z: f32, color: Color) -> Self {
        Self {
            id,
            x,
            y,
            z,
            alive: true,
            color,
        }
    }

    pub fn get_init_vars(spread: f32) -> (f32, f32, f32) {
        let x = 2.0 * (rand::thread_rng().gen_range(0.0..1.0) - 0.5) * spread;
        let y = 2.0 * (rand::thread_rng().gen_range(0.0..1.0) - 0.5) * spread;
        let z = (rand::thread_rng().gen_range(0.0..1.0) + 0.001) * spread;
        (x, y, z)
    }
}
