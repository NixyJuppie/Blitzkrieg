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

fn rotate_player(mut player: Single<&mut Transform, With<Player>>, input: Res<GameplayInput>) {
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
    mut player: Single<&mut Transform, With<Player>>,
    input: Res<GameplayInput>,
    time: Res<Time>,
) {
    let direction = (input.movement.y * player.forward() + input.movement.x * player.right())
        .with_y(0.0)
        .normalize_or_zero();
    player.translation += direction * MOVEMENT_SPEED * time.delta_seconds();

    // TODO: use actual character controller, maybe bevy_tnua?
}

fn use_weapon(
    player: Single<&EquippedWeapons, With<Player>>,
    mut weapons: Query<&mut WeaponState>,
    input: Res<GameplayInput>,
) {
    let Some(current_weapon) = player.current_slot() else {
        return;
    };

    let Ok(mut state) = weapons.get_mut(current_weapon) else {
        unreachable!("Equipped weapon {current_weapon} is not a weapon or does not exist!");
    };

    *state = state.next(input.use_weapon);
}

fn select_weapon(
    mut player: Single<&mut EquippedWeapons, With<Player>>,
    input: Res<GameplayInput>,
) {
    let Some(index) = input.select_weapon else {
        return;
    };

    player.switch(index as usize);
}
