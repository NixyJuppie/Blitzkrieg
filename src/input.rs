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
