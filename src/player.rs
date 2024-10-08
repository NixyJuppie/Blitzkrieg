use crate::{character::EquippedWeapons, input::GameplayInput, prelude::*};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rotate_player, move_player, select_weapon));
    }
}

#[derive(Component, Default)]
#[require(Transform, EquippedWeapons)]
pub struct Player;

const YAW_SENSITIVITY: f32 = 0.2;

fn rotate_player(mut player: Query<&mut Transform, With<Player>>, input: Res<GameplayInput>) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let (yaw, pitch, roll) = player.rotation.to_euler(EulerRot::YXZ);
    player.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw - input.yaw.to_radians() * YAW_SENSITIVITY,
        pitch,
        roll,
    )
}

const MOVEMENT_SPEED: f32 = 15.0;

fn move_player(
    mut player: Query<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let Ok(mut player) = player.get_single_mut() else {
        return;
    };

    let direction = (input.movement.y * player.forward() + input.movement.x * player.right())
        .with_y(0.0)
        .normalize_or_zero();
    player.translation += direction * MOVEMENT_SPEED * time.delta_seconds();

    // TODO: use actual character controller, maybe bevy_tnua?
}

fn select_weapon(mut player: Query<&mut EquippedWeapons, With<Player>>, input: Res<GameplayInput>) {
    let Some(index) = input.select_weapon else {
        return;
    };

    let Ok(mut weapons) = player.get_single_mut() else {
        return;
    };

    weapons.switch(index as usize);
}
