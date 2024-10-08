use bevy::input::mouse::MouseMotion;

use crate::prelude::*;

pub struct GameInputPlugin;
impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameplayInput>();
        app.add_systems(Update, update_gameplay_input);
    }
}

#[derive(Resource, Default)]
pub struct GameplayInput {
    pub yaw: f32,
    pub pitch: f32,
    pub movement: Vec2,
    pub select_weapon: Option<u8>,
}

fn update_gameplay_input(
    mut input: ResMut<GameplayInput>,
    mut motion: EventReader<MouseMotion>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let motion = motion.read().fold(Vec2::ZERO, |a, e| a + e.delta.xy());
    input.yaw = motion.x;
    input.pitch = motion.y;

    input.movement = read_vec2(
        &keys,
        KeyCode::KeyW,
        KeyCode::KeyS,
        KeyCode::KeyA,
        KeyCode::KeyD,
    );

    input.select_weapon = if keys.pressed(KeyCode::Digit1) {
        Some(0)
    } else if keys.pressed(KeyCode::Digit2) {
        Some(1)
    } else if keys.pressed(KeyCode::Digit3) {
        Some(2)
    } else if keys.pressed(KeyCode::Digit4) {
        Some(3)
    } else if keys.pressed(KeyCode::Digit5) {
        Some(4)
    } else if keys.pressed(KeyCode::Digit6) {
        Some(5)
    } else if keys.pressed(KeyCode::Digit7) {
        Some(6)
    } else if keys.pressed(KeyCode::Digit8) {
        Some(7)
    } else if keys.pressed(KeyCode::Digit9) {
        Some(8)
    } else if keys.pressed(KeyCode::Digit0) {
        Some(9)
    } else {
        None
    }
}

fn read_vec2(
    keys: &ButtonInput<KeyCode>,
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
) -> Vec2 {
    let mut value = Vec2::ZERO;

    if keys.pressed(up) {
        value.y += 1.0;
    }
    if keys.pressed(down) {
        value.y -= 1.0;
    }
    if keys.pressed(left) {
        value.x -= 1.0;
    }
    if keys.pressed(right) {
        value.x += 1.0;
    }

    value
}
