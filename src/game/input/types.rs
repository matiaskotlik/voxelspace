//! An abstract input state object that gets fed user
//! events and updates itself based on a set of key
//! bindings.
//!
//! The goals are:
//!
//! * Have a layer of abstract key bindings rather than
//! looking at concrete event types
//! * Use this to be able to abstract away differences
//! between keyboards, joysticks and game controllers
//! (rather based on Unity3D),
//! * Do some tweening of input axes and stuff just for
//! fun.
//! * Take ggez's event-based input API, and present event- or
//! state-based API so you can do whichever you want.

// https://github.com/ggez/ggez-goodies

use std::collections::HashMap;
use std::hash::Hash;

use ggez::event::KeyCode;
use ggez::event::MouseButton;

// Okay, but how does it actually work?
// Basically we have to bind input events to buttons and axes.
// Input events can be keys, mouse buttons/motion, or eventually
// joystick/controller inputs.  Mouse delta can be mapped to axes too.
//
// https://docs.unity3d.com/Manual/ConventionalGameInput.html has useful
// descriptions of the exact behavior of axes.
//
// So to think about this more clearly, here are the default bindings:
//
// W, ↑: +Y axis
// A, ←: -X axis
// S, ↓: -Y axis
// D, →: +X axis
// Enter, z, LMB: Button 1
// Shift, x, MMB: Button 2
// Ctrl,  c, RMB: Button 3
//
// Easy way?  Hash map of event -> axis/button bindings.

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
enum InputType {
    KeyEvent(KeyCode),
    MouseButtonEvent(MouseButton),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InputEffect<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    Axis(Axes, bool),
    Button(Buttons),
    Pointer(f32, f32, f32, f32),
}

#[derive(Debug, Copy, Clone)]
struct AxisState {
    // Where the axis is moving towards.  Possible
    // values are -1, 0, +1, or a continuous range
    // for analog devices
    direction: f32,
}

impl Default for AxisState {
    fn default() -> Self {
        AxisState { direction: 0.0 }
    }
}

#[derive(Debug, Copy, Clone, Default)]
struct ButtonState {
    pressed: bool,
    pressed_last_frame: bool,
}

/// A struct that contains a mapping from physical input events
/// (currently just `KeyCode`s) to whatever your logical Axis/Button
/// types are.
pub struct InputBinding<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    // Once EnumSet is stable it should be used for these
    // instead of BTreeMap. ♥?
    // Binding of keys to input values.
    bindings: HashMap<InputType, InputEffect<Axes, Buttons>>,
}

impl<Axes, Buttons> Default for InputBinding<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    fn default() -> Self {
        InputBinding { bindings: HashMap::new() }
    }
}

impl<Axes, Buttons> InputBinding<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    /// Adds a key binding connecting the given keycode to the given
    /// logical axis.
    pub fn bind_key_to_axis(mut self, keycode: KeyCode, axis: Axes, positive: bool) -> Self {
        self.bindings.insert(InputType::KeyEvent(keycode), InputEffect::Axis(axis, positive));
        self
    }

    /// Adds a key binding connecting the given keycode to the given
    /// logical button.
    pub fn bind_key_to_button(mut self, keycode: KeyCode, button: Buttons) -> Self {
        self.bindings.insert(InputType::KeyEvent(keycode), InputEffect::Button(button));
        self
    }

    pub fn bind_mouse_to_button(mut self, mouse_button: MouseButton, button: Buttons) -> Self {
        self.bindings
            .insert(InputType::MouseButtonEvent(mouse_button), InputEffect::Button(button));
        self
    }

    /// Takes an physical input type and turns it into a logical input type (keycode -> axis/button).
    pub fn resolve_keycode(&self, keycode: KeyCode) -> Option<InputEffect<Axes, Buttons>> {
        self.bindings.get(&InputType::KeyEvent(keycode)).cloned()
    }

    /// Takes an physical input type and turns it into a logical input type (keycode -> axis/button).
    pub fn resolve_mousebutton(&self, button: MouseButton) -> Option<InputEffect<Axes, Buttons>> {
        self.bindings.get(&InputType::MouseButtonEvent(button)).cloned()
    }

    pub fn resolve_mousemotion(
        &self,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
    ) -> InputEffect<Axes, Buttons> {
        InputEffect::Pointer(x, y, dx, dy)
    }
}

#[derive(Debug)]
pub struct InputState<Axes, Buttons>
where
    Axes: Hash + Eq + Clone,
    Buttons: Hash + Eq + Clone,
{
    // Input state for axes
    axes: HashMap<Axes, AxisState>,
    // Input states for buttons
    buttons: HashMap<Buttons, ButtonState>,
}

impl<Axes, Buttons> Default for InputState<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    fn default() -> Self {
        InputState { axes: HashMap::new(), buttons: HashMap::new() }
    }
}

impl<Axes, Buttons> InputState<Axes, Buttons>
where
    Axes: Eq + Hash + Clone,
    Buttons: Eq + Hash + Clone,
{
    pub fn new() -> Self {
        Self::default()
    }
    /// Updates the logical input state based on the actual
    /// physical input state.  Should be called in your update()
    /// handler.
    pub fn update(&mut self, _dt: f32) {
        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed_last_frame = button_status.pressed;
        }
    }

    /// Takes an InputEffect and actually applies it.
    pub fn update_effect(&mut self, effect: InputEffect<Axes, Buttons>, started: bool) {
        match effect {
            InputEffect::Axis(axis, positive) => {
                let f = || AxisState::default();
                let axis_status = self.axes.entry(axis).or_insert_with(f);
                if started {
                    let direction_float = if positive { 1.0 } else { -1.0 };
                    axis_status.direction = direction_float;
                } else if (positive && axis_status.direction > 0.0)
                    || (!positive && axis_status.direction < 0.0)
                {
                    axis_status.direction = 0.0;
                }
            }
            InputEffect::Button(button) => {
                let f = || ButtonState::default();
                let button_status = self.buttons.entry(button).or_insert_with(f);
                button_status.pressed = started;
            }
            _ => (),
        }
    }

    pub fn get_axis(&self, axis: Axes) -> f32 {
        let d = AxisState::default();
        let axis_status = self.axes.get(&axis).unwrap_or(&d);
        axis_status.direction
    }

    fn get_button(&self, button: Buttons) -> ButtonState {
        let d = ButtonState::default();
        let button_status = self.buttons.get(&button).unwrap_or(&d);
        *button_status
    }

    pub fn get_button_down(&self, axis: Buttons) -> bool {
        self.get_button(axis).pressed
    }

    pub fn get_button_up(&self, axis: Buttons) -> bool {
        !self.get_button(axis).pressed
    }

    /// Returns whether or not the button was pressed this frame,
    /// only returning true if the press happened this frame.
    ///
    /// Basically, `get_button_down()` and `get_button_up()` are level
    /// triggers, this and `get_button_released()` are edge triggered.
    pub fn get_button_pressed(&self, axis: Buttons) -> bool {
        let b = self.get_button(axis);
        b.pressed && !b.pressed_last_frame
    }

    pub fn get_button_released(&self, axis: Buttons) -> bool {
        let b = self.get_button(axis);
        !b.pressed && b.pressed_last_frame
    }

    pub fn mouse_scroll_delta(&self) {
        unimplemented!()
    }

    pub fn get_mouse_button(&self) {
        unimplemented!()
    }

    pub fn get_mouse_button_down(&self) {
        unimplemented!()
    }

    pub fn get_mouse_button_up(&self) {
        unimplemented!()
    }

    pub fn reset_input_state(&mut self) {
        for (_axis, axis_status) in self.axes.iter_mut() {
            axis_status.direction = 0.0;
        }

        for (_button, button_status) in self.buttons.iter_mut() {
            button_status.pressed = false;
            button_status.pressed_last_frame = false;
        }
    }
}
