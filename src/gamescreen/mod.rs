use std::f32::consts::PI;

use game::input::Axis;
use game::input::Button;
use ggez::graphics::Color;
use ggez::input::mouse::CursorIcon;
use ggez::{self};
use glam::*;

mod debugtext;
mod map;

use debugtext::DebugText;
use game::input::types::InputEffect;
use game::Scene;
use game::World;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use map::Map;

use crate::game;

const MAP_COUNT: i32 = 29;
const SKY: Color = Color { r: 53.0 / 255.0, g: 81.0 / 255.0, b: 92.0 / 255.0, a: 1.0 };
const TO_RADIANS: f32 = PI / 180.0;
const MIN_FOV: f32 = 30.0 * TO_RADIANS;
const MAX_FOV: f32 = 150.0 * TO_RADIANS;
const MIN_VIEW_DISTANCE: f32 = 10.0;
const DEFAULT_Y: f32 = 200.0;
const DEFAULT_HEIGHT_SCALE: f32 = 300.0;
const DEFAULT_VIEW_DISTANCE: f32 = 800.0;
const DEFAULT_HORIZON: f32 = 100.0;
const DEFAULT_FOV: f32 = 50.0 * TO_RADIANS;
const DEFAULT_SPEED: f32 = 75.50;
const DEFAULT_H_SPEED: f32 = 75.50;
const DEFAULT_HS_SENS: f32 = 20.0;
const DEFAULT_FOV_SPEED: f32 = 40.0 * TO_RADIANS;
const DEFAULT_V_SPEED: f32 = 150.0;

// First we make a structure to contain the game's state
#[derive(Debug)]
pub struct MapState {
    map_id: i32,
    pub map: Map,
    pub draw_param: MapDrawParam,
    pub draw_debug: bool,
    pub debug: DebugText,
    pub speed: f32,
    pub h_speed: f32,
    pub hs_sens: f32,
    pub fov_speed: f32,
    pub v_speed: f32,
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct MapDrawParam {
    #[derivative(Debug(format_with = "vec3_fmt"))]
    pub camera: Vec3,
    pub rotation: f32,
    pub height_scale: f32,
    pub view_distance: f32,
    pub horizon: f32,
    pub fov: f32,
}

impl Default for MapDrawParam {
    fn default() -> Self {
        MapDrawParam {
            camera: Vec3::new(0.0, DEFAULT_Y, 0.0),
            rotation: 0.0,
            height_scale: DEFAULT_HEIGHT_SCALE,
            view_distance: DEFAULT_VIEW_DISTANCE,
            horizon: DEFAULT_HORIZON,
            fov: DEFAULT_FOV,
        }
    }
}

impl MapState {
    pub fn new(ctx: &mut Context, map_id: i32) -> GameResult<MapState> {
        Ok(MapState {
            map_id,
            map: Map::new(ctx, map_id)?,
            draw_debug: true,
            debug: DebugText::new(ctx)?,
            draw_param: MapDrawParam::default(),
            speed: DEFAULT_SPEED,
            h_speed: DEFAULT_H_SPEED,
            hs_sens: DEFAULT_HS_SENS,
            fov_speed: DEFAULT_FOV_SPEED,
            v_speed: DEFAULT_V_SPEED,
        })
    }

    fn reset(&mut self) {
        // save fov
        self.draw_param = MapDrawParam { fov: self.draw_param.fov, ..Default::default() }
    }

    fn pointer_moved(
        &mut self,
        ctx: &mut Context,
        state: &mut World,
        _x: f32,
        _y: f32,
        dx: f32,
        dy: f32,
    ) {
        if state.input.get_button_down(Button::Grab) {
            let (_width, height) = ggez::graphics::drawable_size(ctx);
            self.draw_param.rotation += (dx / height) * self.draw_param.fov;

            let full = 2.0 * PI;
            self.draw_param.rotation = (self.draw_param.rotation % full + full) % full;

            self.draw_param.horizon += dy;
        }
    }

    fn handle_button(
        &mut self,
        ctx: &mut Context,
        _world: &mut World,
        button: Button,
        started: bool,
    ) -> GameResult {
        use Button::*;
        match (button, started) {
            (Quit, true) => event::quit(ctx),
            (Reload, true) => self.update_map(ctx, 0)?,
            (Next, true) => self.update_map(ctx, 1)?,
            (Prev, true) => self.update_map(ctx, -1)?,
            (ToggleDebug, true) => self.draw_debug = !self.draw_debug,
            (Grab, started) => {
                let cursor = if started { CursorIcon::Grabbing } else { CursorIcon::Default };
                ggez::input::mouse::set_cursor_type(ctx, cursor);
            }
            _ => (),
        }
        Ok(())
    }

    fn update_map(&mut self, ctx: &mut Context, change: i32) -> GameResult {
        self.map_id = ((self.map_id - 1 + change) % MAP_COUNT + MAP_COUNT) % MAP_COUNT + 1;
        self.map = Map::new(ctx, self.map_id)?;
        self.reset();
        Ok(())
    }
}

impl Scene for MapState {
    fn update(&mut self, state: &mut game::World, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();

        // calculate forward and sideways directions
        let direction =
            Vec3::new(self.draw_param.rotation.cos(), 0.0, self.draw_param.rotation.sin());
        let strafe_direction = Vec3::new(direction.z(), 0.0, -direction.x());

        // adjust draw parameters from control state
        self.draw_param.height_scale += state.input.get_axis(Axis::HeightScale) * self.hs_sens * dt;
        self.draw_param.view_distance +=
            state.input.get_axis(Axis::ViewDistance) * self.v_speed * dt;
        self.draw_param.camera +=
            direction * state.input.get_axis(Axis::Throttle) * self.speed * dt;
        self.draw_param.camera +=
            strafe_direction * state.input.get_axis(Axis::Strafe) * self.speed * dt;
        *self.draw_param.camera.y_mut() += state.input.get_axis(Axis::Height) * self.h_speed * dt;
        self.draw_param.fov += state.input.get_axis(Axis::Fov) * self.fov_speed * dt;

        self.draw_param.view_distance = self.draw_param.view_distance.max(MIN_VIEW_DISTANCE);
        self.draw_param.fov = self.draw_param.fov.clamp(MIN_FOV, MAX_FOV);

        Ok(())
    }

    fn draw(&mut self, _state: &mut game::World, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, SKY);

        self.map.draw(ctx, &self.draw_param)?;
        if self.draw_debug {
            self.debug.draw(&self.draw_param, self.map_id, ctx)?;
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn input(
        &mut self,
        state: &mut game::World,
        ctx: &mut Context,
        event: game::Event,
        started: bool,
    ) -> GameResult {
        match event {
            InputEffect::Button(button) => self.handle_button(ctx, state, button, started)?,
            InputEffect::Axis(_axis, _value) => (),
            InputEffect::Pointer(x, y, dx, dy) => self.pointer_moved(ctx, state, x, y, dx, dy),
        }
        Ok(())
    }
}

fn vec3_fmt(v: &Vec3, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    write!(fmt, "({:.2}, {:.2}, {:.2})", &v.x(), &v.y(), &v.z())
}
