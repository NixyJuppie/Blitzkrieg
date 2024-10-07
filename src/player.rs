use crate::{input::GameplayInput, prelude::*};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (rotate_player, move_player));
    }
}

#[derive(Component, Default)]
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
    mut player: Query<(&mut LinearVelocity, &Transform), With<Player>>,
    input: Res<GameplayInput>,
) {
    let Ok((mut velocity, transform)) = player.get_single_mut() else {
        return;
    };

    let movement =
        transform.rotation * Vec3::new(input.movement.x, 0.0, -input.movement.y) * MOVEMENT_SPEED;
    velocity.x = velocity.x.lerp(movement.x, 0.1);
    velocity.z = velocity.z.lerp(movement.z, 0.1);

    // TODO: use actual character controller, maybe bevy_tnua?
}
