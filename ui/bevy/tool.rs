use bevy::prelude::*;
use bevy::input::ButtonState;
use bevy::input::mouse::{MouseButtonInput, MouseMotion, MouseWheel};
use bevy::input::keyboard::KeyboardInput;

#[derive(PartialEq, Debug, Clone, Default)]
pub enum Operation {
    #[default] Select,
    Pan,
    Orbit,
    Zoom
}

#[derive(Resource, PartialEq, Debug, Clone, Default)]
pub struct Tool {
    operation: Operation,
    position: Vec2,
    velocity: Vec2
}

pub fn update_tool(
    mut tool: ResMut<Tool>,
    windows: Query<&Window>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut keyboard_events: EventReader<KeyboardInput>,
) {
    match windows.single().cursor_position() {
        Some(position) => {
            tool.position = position;
        }
        None => {}
    }
    tool.velocity = mouse_motion_events.read().map(|event| event.delta).sum();
    for event in mouse_button_events.read() {
        match event.state {
            ButtonState::Pressed => {
                match event.button {
                    MouseButton::Left => {
                        tool.operation = Operation::Orbit;
                    }
                    MouseButton::Right => {
                        tool.operation = Operation::Pan;
                    }
                    _ => {}
                }
            }
            ButtonState::Released => {
                tool.operation = Operation::Select;
            }
        };
    }
}