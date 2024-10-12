use bevy::window::{CursorGrabMode, CursorOptions, PrimaryWindow};

use crate::input::GameplayInput;
use crate::prelude::*;

pub struct FirstPersonCameraPlugin;
impl Plugin for FirstPersonCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_cursor);
        app.add_systems(
            Update,
            (handle_cursor_grab_mode, rotate_camera, move_camera),
        );
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

fn setup_cursor(mut window: Single<&mut Window, With<PrimaryWindow>>) {
    lock_cursor(&mut window.cursor_options);
}

fn handle_cursor_grab_mode(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    input: Res<GameplayInput>,
) {
    if !input.switch_cursor_mode {
        return;
    }

    if is_cursor_unlocked(&window.cursor_options) {
        lock_cursor(&mut window.cursor_options);
    } else {
        unlock_cursor(&mut window.cursor_options);
    }
}

fn is_cursor_unlocked(cursor: &CursorOptions) -> bool {
    cursor.grab_mode == CursorGrabMode::None
}

fn lock_cursor(cursor: &mut CursorOptions) {
    cursor.visible = false;
    cursor.grab_mode = CursorGrabMode::Confined;
}

fn unlock_cursor(cursor: &mut CursorOptions) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}

const PITCH_SENSITIVITY: f32 = 0.2;
const MIN_PITCH_DEGREES: f32 = -45.0;
const MAX_PITCH_DEGREES: f32 = 45.0;

fn rotate_camera(
    mut camera: Single<&mut Transform, (With<FirstPersonCamera>, Without<FirstPersonCameraTarget>)>,
    target: Single<&Transform, (With<FirstPersonCameraTarget>, Without<FirstPersonCamera>)>,
    window: Single<&Window, With<PrimaryWindow>>,
    input: Res<GameplayInput>,
) {
    if is_cursor_unlocked(&window.cursor_options) {
        return;
    }

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
    mut camera: Single<&mut Transform, (With<FirstPersonCamera>, Without<FirstPersonCameraTarget>)>,
    window: Single<&Window, With<PrimaryWindow>>,
    target: Single<(&Transform, &FirstPersonCameraTarget), Without<FirstPersonCamera>>,
) {
    if is_cursor_unlocked(&window.cursor_options) {
        return;
    }

    let (target, FirstPersonCameraTarget { height }) = *target;
    camera.translation = target.translation + Vec3::new(0.0, *height, 0.0);
}
