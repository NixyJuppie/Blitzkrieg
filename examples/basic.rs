use blitzkrieg::{
    camera::{FirstPersonCamera, FirstPersonCameraTarget},
    create_app,
    player::Player,
    prelude::*,
    GameInfo,
};

fn main() {
    let mut app = create_app(GameInfo {
        name: "Basic",
        version: None,
    });

    app.add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        FirstPersonCameraTarget { height: 0.75 },
        TransformBundle::default(),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        LinearVelocity::default(),
        LockedAxes::ROTATION_LOCKED,
    ));
    commands.spawn((Camera3dBundle::default(), FirstPersonCamera));
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.5, 1.0, -0.25).looking_at(Vec3::ZERO, Vec3::Y),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(0.0, -5.0, 0.0),
            mesh: meshes.add(Cuboid::new(100.0, 0.1, 100.0)),
            material: materials.add(Color::srgb_u8(0, 64, 0)),
            ..default()
        },
        Collider::cuboid(100.0, 0.1, 100.0),
        RigidBody::Static,
    ));

    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(10.0, 2.5, -20.0),
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
    commands.spawn((
        PbrBundle {
            transform: Transform::from_xyz(-10.0, 2.5, -30.0),
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::srgb_u8(255, 0, 0)),
            ..default()
        },
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
    ));
}
