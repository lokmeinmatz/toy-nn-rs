use crate::game_maths::Vec2;
use ggez::{Context, GameResult};
use ggez::graphics::{Mesh, DrawParam, MeshBuilder, DrawMode, Color};

pub struct Border {
    inner: Vec<Vec2>,
    outer: Vec<Vec2>,
    mesh: Mesh
}

impl Border {
    pub fn new(ctx: &mut Context, resolution: usize, radius: f32, offset: f32, rand_jitter: f32) -> GameResult<Border> {

        let mut inner = Vec::new();
        let mut outer = Vec::new();

        for i in 0..(resolution) {
            let angle = (i as f32) / (resolution as f32) * std::f32::consts::PI * 2.0;
            let jitter: f32 = rand::random::<f32>() * rand_jitter;
            inner.push(Vec2::unit_from_angle(angle) * (radius + jitter));
            outer.push(Vec2::unit_from_angle(angle) * (radius + offset + jitter));
        }

        inner.push(inner[0].clone());
        outer.push(outer[0].clone());

        let mesh = MeshBuilder::new().polyline(DrawMode::stroke(0.3), &inner, Color::from_rgb(0, 255, 0))?.polyline(DrawMode::stroke(0.3), &outer, Color::from_rgb(0, 255, 0))?.build(ctx)?;

        Ok(Border {
            inner,
            outer,
            mesh
        })
    }

    pub fn draw(&self, ctx: &mut Context, offset: Vec2, scale: f32) -> GameResult<()> {

        ggez::graphics::draw(ctx, &self.mesh, DrawParam::new().dest(offset).scale([scale, scale]))?;

        Ok(())
    }

    pub fn check_collision(&self, lines: &[(Vec2, Vec2)]) -> bool {
        for l in lines {
            for e in self.inner.windows(2) {
                if let [p1, p2] = e {
                    if crate::game_maths::line_line_intersection((*p1, *p2), *l).is_some() {
                        return true;
                    }
                }
            }
        }

        false
    }
}