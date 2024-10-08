use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::{input::GameplayInput, prelude::*};

pub struct FirstPersonCameraPlugin;
impl Plugin for FirstPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor);
        app.add_systems(Update, (rotate_camera, move_camera));
    }
}

#[derive(Component, Default, Debug)]
#[require(Transform, Camera3d)]
pub struct FirstPersonCamera;

#[derive(Component, Default, Debug, Constructor)]
#[require(Transform)]
pub struct FirstPersonCameraTarget {
    pub height: f32,
}

fn setup_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window.single_mut();
    window.cursor_options.visible = false;
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
}

const PITCH_SENSITIVITY: f32 = 0.2;
const MIN_PITCH_DEGREES: f32 = -45.0;
const MAX_PITCH_DEGREES: f32 = 45.0;

fn rotate_camera(
    mut camera: Query<&mut Transform, (With<FirstPersonCamera>, Without<FirstPersonCameraTarget>)>,
    target: Query<&Transform, (With<FirstPersonCameraTarget>, Without<FirstPersonCamera>)>,
    input: Res<GameplayInput>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok(target) = target.get_single() else {
        return;
    };

    let (_, pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);
    let (yaw, _, roll) = target.rotation.to_euler(EulerRot::YXZ);

    camera.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw,
        (pitch.to_degrees() - input.pitch * PITCH_SENSITIVITY)
            .clamp(MIN_PITCH_DEGREES, MAX_PITCH_DEGREES)
            .to_radians(),
        roll,
    )
}

fn move_camera(
    mut camera: Query<&mut Transform, (With<FirstPersonCamera>, Without<FirstPersonCameraTarget>)>,
    target: Query<(&Transform, &FirstPersonCameraTarget), Without<FirstPersonCamera>>,
) {
    let Ok(mut camera) = camera.get_single_mut() else {
        return;
    };
    let Ok((target, FirstPersonCameraTarget { height })) = target.get_single() else {
        return;
    };

    camera.translation = target.translation + Vec3::new(0.0, *height, 0.0);
}
