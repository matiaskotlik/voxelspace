use ggez::event;

pub mod types;

use types::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Button {
    Next,
    Prev,
    Reload,
    Quit,
    ToggleDebug,
    Grab,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Axis {
    Throttle,
    Strafe,
    HeightScale,
    ViewDistance,
    Height,
    Fov,
}

pub type Binding = InputBinding<Axis, Button>;
pub type Event = InputEffect<Axis, Button>;
pub type State = InputState<Axis, Button>;

/// Create the default keybindings for our input state.
pub fn create_input_binding() -> InputBinding<Axis, Button> {
    use event::KeyCode;
    use event::MouseButton;
    InputBinding::new()
        .bind_key_to_axis(KeyCode::D, Axis::Strafe, true)
        .bind_key_to_axis(KeyCode::A, Axis::Strafe, false)
        .bind_key_to_axis(KeyCode::W, Axis::Throttle, true)
        .bind_key_to_axis(KeyCode::S, Axis::Throttle, false)
        .bind_key_to_axis(KeyCode::Up, Axis::HeightScale, true)
        .bind_key_to_axis(KeyCode::Down, Axis::HeightScale, false)
        .bind_key_to_axis(KeyCode::Right, Axis::ViewDistance, true)
        .bind_key_to_axis(KeyCode::Left, Axis::ViewDistance, false)
        .bind_key_to_axis(KeyCode::Space, Axis::Height, true)
        .bind_key_to_axis(KeyCode::LShift, Axis::Height, false)
        .bind_key_to_axis(KeyCode::RBracket, Axis::Fov, true)
        .bind_key_to_axis(KeyCode::LBracket, Axis::Fov, false)
        .bind_mouse_to_button(MouseButton::Left, Button::Grab)
        .bind_key_to_button(KeyCode::N, Button::Next)
        .bind_key_to_button(KeyCode::P, Button::Prev)
        .bind_key_to_button(KeyCode::R, Button::Reload)
        .bind_key_to_button(KeyCode::L, Button::ToggleDebug)
        .bind_key_to_button(KeyCode::Escape, Button::Quit)
}
