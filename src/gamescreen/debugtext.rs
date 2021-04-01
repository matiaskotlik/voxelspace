use ggez::graphics::Align;
use ggez::graphics::Font;
use ggez::graphics::Text;
use ggez::Context;
use ggez::GameResult;
use glam::*;

use super::MapDrawParam;

#[derive(Derivative)]
#[derivative(Debug)]
pub struct DebugText {
    #[derivative(Debug = "ignore")]
    font: Font,
}

impl DebugText {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(DebugText { font: Font::new(ctx, "/LiberationMono-Regular.ttf")? })
    }

    pub fn draw(&mut self, param: &MapDrawParam, map_id: i32, ctx: &mut Context) -> GameResult {
        let left = format!(
            "FPS: {fps:.0}
Position: ({x:.0}, {y:.0}, {z:.0}) {rotation:.0}°
Render Distance: {view_distance:.0} FOV: {fov:.0}°
Height Scale: {height_scale:.0} Horizon: {horizon:.0}
Map: {map_id}",
            fps = ggez::timer::fps(ctx),
            x = param.camera.x(),
            y = param.camera.y(),
            z = param.camera.z(),
            rotation = param.rotation.to_degrees(),
            view_distance = param.view_distance,
            fov = param.fov.to_degrees(),
            height_scale = param.height_scale,
            horizon = param.horizon,
            map_id = map_id,
        );

        let right = "Controls:
WASD: Move Camera
→←: Render Distance
↑↓: Height Scale
Space/LShift: Up/Down
N/P: Next/Previous Map
L: Toggle Debug Text";

        let (width, height) = ggez::graphics::drawable_size(ctx);
        let bounds = Vec2::new(width, height);

        let mut left_text = Text::new((left, self.font, 16.0));
        left_text.set_bounds(bounds, Align::Left);
        ggez::graphics::draw(ctx, &left_text, (Vec2::new(0.0, 0.0),))?;

        let mut right_text = Text::new((right, self.font, 16.0));
        right_text.set_bounds(bounds, Align::Right);
        ggez::graphics::draw(ctx, &right_text, (Vec2::new(0.0, 0.0),))?;
        Ok(())
    }
}
