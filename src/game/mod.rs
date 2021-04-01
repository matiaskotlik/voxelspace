use ggez::event;
use ggez::graphics::Rect;
use ggez::Context;
use ggez::GameResult;

pub mod input;
pub mod scene;

pub use input::Event;
pub use scene::Scene;

use self::input::create_input_binding;
use self::input::types::InputState;

pub struct World {
    pub input: input::State,
}

pub struct Game {
    scenes: Vec<Box<dyn Scene>>,
    world: World,
    input_binding: input::Binding,
    prev_x: f32,
    prev_y: f32,
}

impl Game {
    pub fn new(scene_stack: Vec<Box<dyn Scene>>) -> Self {
        Game {
            scenes: scene_stack,
            world: World { input: InputState::default() },
            input_binding: create_input_binding(),
            prev_x: f32::NAN,
            prev_y: f32::NAN,
        }
    }

    fn input(&mut self, ctx: &mut Context, event: input::Event, started: bool) -> GameResult {
        if let Some(scene) = self.scenes.last_mut() {
            scene.as_mut().input(&mut self.world, ctx, event, started)?;
        } else {
            event::quit(ctx);
        }
        Ok(())
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.world.input.update(ggez::timer::delta(ctx).as_secs_f32());
        if let Some(scene) = self.scenes.last_mut() {
            scene.as_mut().update(&mut self.world, ctx)?;
        } else {
            event::quit(ctx);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if let Some(scene) = self.scenes.last_mut() {
            scene.as_mut().draw(&mut self.world, ctx)?;
        } else {
            event::quit(ctx);
        }
        Ok(())
    }

    fn mouse_motion_event(&mut self, ctx: &mut Context, x: f32, y: f32, _: f32, _: f32) {
        let dx = if self.prev_x.is_nan() { 0.0 } else { x - self.prev_x };
        let dy = if self.prev_y.is_nan() { 0.0 } else { y - self.prev_y };
        self.prev_x = x;
        self.prev_y = y;
        self.input(ctx, self.input_binding.resolve_mousemotion(x, y, dx, dy), true).unwrap();
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
        _repeat: bool,
    ) {
        if let Some(ev) = self.input_binding.resolve_keycode(keycode) {
            self.world.input.update_effect(ev, true);
            self.input(ctx, ev, true).unwrap();
        }
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: event::KeyMods,
    ) {
        if let Some(ev) = self.input_binding.resolve_keycode(keycode) {
            self.world.input.update_effect(ev, false);
            self.input(ctx, ev, false).unwrap();
        }
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Some(ev) = self.input_binding.resolve_mousebutton(button) {
            self.world.input.update_effect(ev, true);
            self.input(ctx, ev, true).unwrap();
        }
    }

    fn mouse_button_up_event(
        &mut self,
        ctx: &mut Context,
        button: event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if let Some(ev) = self.input_binding.resolve_mousebutton(button) {
            self.world.input.update_effect(ev, false);
            self.input(ctx, ev, false).unwrap();
        }
    }

    fn resize_event(&mut self, ctx: &mut Context, width: f32, height: f32) {
        ggez::graphics::set_screen_coordinates(ctx, Rect::new(0.0, 0.0, width, height)).unwrap();
    }
}
