use ggez::graphics;
use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Color;
use ggez::graphics::DrawParam;
use ggez::graphics::Image;
use ggez::Context;
use ggez::GameResult;
use glam::*;

const RENDER_DETAIL: f32 = 150.0;

use super::MapDrawParam;
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Map {
    id: i32,
    #[derivative(Debug = "ignore")]
    colors: Vec<Color>,
    #[derivative(Debug = "ignore")]
    height_map: Vec<u8>,
    size: i32,
    #[derivative(Debug = "ignore")]
    period: i32,
    #[derivative(Debug = "ignore")]
    shift: i32,
    #[derivative(Debug = "ignore")]
    batch: SpriteBatch,
}

impl Map {
    pub fn new(ctx: &mut Context, id: i32) -> GameResult<Self> {
        let color_image_name = format!("/maps/C{}.png", id);
        let height_image_name = format!("/maps/D{}.png", id);
        let color_image = graphics::Image::new(ctx, color_image_name)?;
        let height_image = graphics::Image::new(ctx, height_image_name)?;
        assert_eq!(color_image.dimensions(), height_image.dimensions());

        let color_image_pixels = color_image.to_rgba8(ctx)?;
        let height_image_pixels = height_image.to_rgba8(ctx)?;
        let size = color_image.width() as i32;

        let colors =
            color_image_pixels.chunks_exact(4).map(|p| Color::from_rgb(p[0], p[1], p[2])).collect();

        let height_map = height_image_pixels.chunks_exact(4).map(|p| p[0]).collect();

        let shift = (size as f64).log2() as i32;
        assert_eq!(1 << shift, size);

        let mut batch = SpriteBatch::new(Image::solid(ctx, 1, Color::WHITE)?);
        batch.set_filter(graphics::FilterMode::Nearest);

        Ok(Map { id, colors, height_map, size, period: size - 1, shift, batch })
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    #[inline]
    pub fn get_point(&self, x: i32, y: i32) -> (Color, u8) {
        let x_offset = x as i32 & self.period;
        let y_offset = (y as i32 & self.period) << self.shift;
        let idx = (x_offset + y_offset) as usize;
        let color = self.colors[idx];
        let map_height = self.height_map[idx];
        (color, map_height)
    }

    pub fn draw(&mut self, ctx: &mut Context, param: &MapDrawParam) -> GameResult {
        let (width, height) = ggez::graphics::drawable_size(ctx);
        // visibility array
        let mut visibility = vec![height; width as usize];
        let mut z = 1.0;
        while z < param.view_distance {
            // projection
            let invz = 1.0 / z * param.height_scale;

            // find line on map. this corresponds to 90 degree FOV
            let left = param.rotation + param.fov / 2.0;
            let right = param.rotation - param.fov / 2.0;

            // draw left to right
            let r_delta = (right - left) / width;
            for (screen_x, visible_y) in visibility.iter_mut().enumerate() {
                // get position on map
                let phi = left + r_delta * screen_x as f32;
                let map_x = param.camera.x() + phi.cos() * z;
                let map_y = param.camera.z() + phi.sin() * z;

                // get color and height from map at the point
                let (color, map_height) = self.get_point(map_x as i32, map_y as i32);

                // perspective projection for height
                // aka voodoo magic
                let mut y = param.camera.y() - map_height as f32;
                y = (y * invz + param.horizon).clamp(0.0, height);

                if y < *visible_y {
                    self.draw_vertical_line(screen_x as f32, y, *visible_y, color);
                    *visible_y = y
                }
            }

            z += (z / RENDER_DETAIL).max(1.0);
        }

        graphics::draw(ctx, &self.batch, DrawParam::new())?;
        self.batch.clear();

        Ok(())
    }

    fn draw_vertical_line(&mut self, x: f32, y1: f32, y2: f32, color: Color) {
        assert!(y1 <= y2);
        self.batch.add(
            DrawParam::new().dest(Vec2::new(x, y1)).color(color).scale(Vec2::new(1.0, y2 - y1)),
        );
    }
}
