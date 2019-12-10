use std::f32::consts::FRAC_PI_2;
use crate::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{self, Mesh, MeshBuilder, DrawMode, Color, DrawParam};
use crate::game_maths::Mat3;
use crate::borders::Border;

static CAR_BASE_SHAPE : [Vec2; 4] = [
    Vec2::new(1.0, 2.0),
    Vec2::new(1.0, -2.0),
    Vec2::new(-1.0, -2.0),
    Vec2::new(-1.0, 2.0)
];


pub struct Car {
    polygon: [Vec2; 4],
    mesh: Mesh,
    pos: Vec2,
    dir: f32,
    speed: f32,
    steering: f32,
    crashed: bool
}

impl Car {
    pub fn new(pos: Vec2, dir: f32, ctx: &mut Context) -> Car {
        let mut c = Car {
            polygon: CAR_BASE_SHAPE.clone(),
            mesh: MeshBuilder::new().polygon(DrawMode::stroke(0.1), &CAR_BASE_SHAPE, Color::new(1.0, 0.0, 0.5, 1.0)).unwrap().build(ctx).unwrap(),
            pos: pos.clone(),
            dir,
            speed: 0.0,
            steering: 0.0,
            crashed: false
        };

        c.update_polygon();

        c
    }

    fn update_polygon(&mut self) {
        for (i, vec) in CAR_BASE_SHAPE.iter().enumerate() {
            self.polygon[i] = Mat3::affine(self.pos, self.dir, (1.0, 1.0).into()) * *vec;
        }
    }

    pub fn update(&mut self, speed: f32, steering: f32, border: &Border) {

        if self.crashed { return; }
        let speed = speed * 0.5;
        self.speed = (self.speed * 50.0 + speed) / 51.0;
        self.steering = (self.steering * 10.0 + steering) / 11.0;

        let old_polygon: [Vec2; 4] = self.polygon.clone();

        self.dir += self.steering * self.speed;

        self.pos += Vec2::unit_from_angle(self.dir + FRAC_PI_2) * self.speed;

        self.update_polygon();

        // generate movement paths
        let mut movement = [(Vec2::zero(), Vec2::zero()); 4];
        for (idx, (old, new)) in self.polygon.iter().zip(old_polygon.iter()).enumerate() {
            movement[idx] = (*old, *new);
        }

        if border.check_collision(&movement) {
            // crashed
            println!("Crashed!!!");
            self.crashed = true;
        }

    }

    fn gen_rays(&self) -> Vec<Vec2> {
        let mut res = vec![Vec2::unit_from_angle(self.dir + FRAC_PI_2)];

        for angle in &[15f32, 30f32, 45f32, 60f32] {
            res.push(Vec2::unit_from_angle(self.dir + FRAC_PI_2 + angle.to_radians()));
            res.push(Vec2::unit_from_angle(self.dir + FRAC_PI_2 - angle.to_radians()));
        }

        res
    }

    fn get_front(&self) -> Vec2 {
        Mat3::affine(self.pos, self.dir, (1.0, 1.0).into()) * Vec2::new(0.0, 2.0)
    }

    fn get_front_local(&self) -> Vec2 {
        Mat3::rotate(self.dir) * Vec2::new(0.0, 2.0)
    }

    pub fn draw(&mut self, ctx: &mut Context, offset: Vec2, scale: f32) -> GameResult<()> {
        let local_draw_param = DrawParam::new().dest(self.pos * scale + offset).rotation(self.dir).scale([scale, scale]);

        graphics::draw(ctx, &self.mesh, local_draw_param)?;

        let circle = Mesh::new_circle(ctx, DrawMode::fill(), Vec2::new(0f32, 2f32), 0.3, 0.05, Color::new(0.0, 0.4, 0.6, 1.0))?;
        graphics::draw(ctx, &circle, local_draw_param)?;

        let rays = self.gen_rays();

        let mut ray_mesh = MeshBuilder::new();
        let front_glob = self.pos +  self.get_front_local();
        for ray in &rays {

            ray_mesh.line(&[front_glob.clone(), front_glob + *ray], 0.04, Color::from_rgb(100, 150, 200))?;
        }

        let ray_mesh = ray_mesh.build(ctx)?;
        graphics::draw(ctx, &ray_mesh, DrawParam::new().scale([scale, scale]).dest(offset))?;


        Ok(())
    }
}