use crate::converter::{convert_axis, convert_button, convert_gamepad_id};
use bevy_app::Events;
use bevy_ecs::{Resources, World};
use bevy_input::prelude::*;
use gilrs::{EventType, Gilrs};

pub fn gilrs_startup_system(_world: &mut World, resources: &mut Resources) {
    let gilrs = resources.get_thread_local::<Gilrs>().unwrap();
    let mut gamepad_event = resources.get_mut::<Events<GamepadEvent>>().unwrap();
    let mut inputs = resources.get_mut::<Input<GamepadButton>>().unwrap();
    let mut axes = resources.get_mut::<Axis<GamepadAxis>>().unwrap();
    gamepad_event.update();
    inputs.update();
    for (gilrs_id, _) in gilrs.gamepads() {
        connect_gamepad(
            convert_gamepad_id(gilrs_id),
            &mut gamepad_event,
            &mut inputs,
            &mut axes,
        );
    }
}

pub fn gilrs_update_system(_world: &mut World, resources: &mut Resources) {
    let mut gilrs = resources.get_thread_local_mut::<Gilrs>().unwrap();
    let mut gamepad_event = resources.get_mut::<Events<GamepadEvent>>().unwrap();
    let mut inputs = resources.get_mut::<Input<GamepadButton>>().unwrap();
    let mut axes = resources.get_mut::<Axis<GamepadAxis>>().unwrap();

    gamepad_event.update();
    inputs.update();
    while let Some(gilrs_event) = gilrs.next_event() {
        match gilrs_event.event {
            EventType::Connected => {
                connect_gamepad(
                    convert_gamepad_id(gilrs_event.id),
                    &mut gamepad_event,
                    &mut inputs,
                    &mut axes,
                );
            }
            EventType::Disconnected => {
                disconnect_gamepad(
                    convert_gamepad_id(gilrs_event.id),
                    &mut gamepad_event,
                    &mut inputs,
                    &mut axes,
                );
            }
            EventType::ButtonPressed(gilrs_button, _) => {
                if let Some(button_type) = convert_button(gilrs_button) {
                    inputs.press(GamepadButton(
                        convert_gamepad_id(gilrs_event.id),
                        button_type,
                    ));
                }
            }
            EventType::ButtonReleased(gilrs_button, _) => {
                if let Some(button_type) = convert_button(gilrs_button) {
                    inputs.release(GamepadButton(
                        convert_gamepad_id(gilrs_event.id),
                        button_type,
                    ));
                }
            }
            EventType::AxisChanged(gilrs_axis, value, _) => {
                if let Some(axis_type) = convert_axis(gilrs_axis) {
                    axes.set(
                        GamepadAxis(convert_gamepad_id(gilrs_event.id), axis_type),
                        value,
                    );
                }
            }
            _ => (),
        };
    }
    gilrs.inc();
}

const ALL_GAMEPAD_BUTTON_TYPES: [GamepadButtonType; 19] = [
    GamepadButtonType::South,
    GamepadButtonType::East,
    GamepadButtonType::North,
    GamepadButtonType::West,
    GamepadButtonType::C,
    GamepadButtonType::Z,
    GamepadButtonType::LeftTrigger,
    GamepadButtonType::LeftTrigger2,
    GamepadButtonType::RightTrigger,
    GamepadButtonType::RightTrigger2,
    GamepadButtonType::Select,
    GamepadButtonType::Start,
    GamepadButtonType::Mode,
    GamepadButtonType::LeftThumb,
    GamepadButtonType::RightThumb,
    GamepadButtonType::DPadUp,
    GamepadButtonType::DPadDown,
    GamepadButtonType::DPadLeft,
    GamepadButtonType::DPadRight,
];

const ALL_GAMEPAD_AXIS_TYPES: [GamepadAxisType; 8] = [
    GamepadAxisType::LeftStickX,
    GamepadAxisType::LeftStickY,
    GamepadAxisType::LeftZ,
    GamepadAxisType::RightStickX,
    GamepadAxisType::RightStickY,
    GamepadAxisType::RightZ,
    GamepadAxisType::DPadX,
    GamepadAxisType::DPadY,
];

fn connect_gamepad(
    gamepad: Gamepad,
    events: &mut Events<GamepadEvent>,
    inputs: &mut Input<GamepadButton>,
    axes: &mut Axis<GamepadAxis>,
) {
    for button_type in ALL_GAMEPAD_BUTTON_TYPES.iter() {
        inputs.reset(GamepadButton(gamepad, *button_type));
    }
    for axis_type in ALL_GAMEPAD_AXIS_TYPES.iter() {
        axes.set(GamepadAxis(gamepad, *axis_type), 0.0f32);
    }
    events.send(GamepadEvent(gamepad, GamepadEventType::Connected));
}

fn disconnect_gamepad(
    gamepad: Gamepad,
    events: &mut Events<GamepadEvent>,
    inputs: &mut Input<GamepadButton>,
    axes: &mut Axis<GamepadAxis>,
) {
    for button_type in ALL_GAMEPAD_BUTTON_TYPES.iter() {
        inputs.reset(GamepadButton(gamepad, *button_type));
    }
    for axis_type in ALL_GAMEPAD_AXIS_TYPES.iter() {
        axes.remove(&GamepadAxis(gamepad, *axis_type));
    }
    events.send(GamepadEvent(gamepad, GamepadEventType::Disconnected));
}
