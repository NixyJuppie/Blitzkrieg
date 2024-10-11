use crate::character::EquippedWeapons;
use crate::input::GameplayInput;
use crate::prelude::*;
use crate::weapon::WeaponState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (rotate_player, move_player, use_weapon, select_weapon),
        );
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

fn use_weapon(
    player: Query<&EquippedWeapons, With<Player>>,
    mut weapons: Query<&mut WeaponState>,
    input: Res<GameplayInput>,
) {
    let Some(current_weapon) = player.get_single().ok().and_then(|w| w.current_slot()) else {
        return;
    };

    let Ok(mut state) = weapons.get_mut(current_weapon) else {
        unreachable!("Equipped weapon {current_weapon} is not a weapon or does not exist!");
    };

    *state = if input.use_weapon {
        WeaponState::Active
    } else {
        WeaponState::Idle
    };
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
