use blitzkrieg::camera::{FirstPersonCamera, FirstPersonCameraTarget};
use blitzkrieg::character::EquippedWeapons;
use blitzkrieg::player::Player;
use blitzkrieg::prelude::*;
use blitzkrieg::weapon::gun::{AttachedGunAmmunitionStorage, GunAmmunitionStorage, GunState};
use blitzkrieg::{create_default_app, GameInfo};

fn main() {
    let mut app = create_default_app(GameInfo {
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
    commands.spawn(FirstPersonCamera);
    commands.spawn((
        DirectionalLight::default(),
        Transform::default().looking_at(Vec3::new(0.3, -1.0, -0.5), Vec3::Y),
    ));

    let capsule = meshes.add(Capsule3d::new(1.0, 1.75));
    let gold = materials.add(Color::linear_rgb(1.0, 0.8, 0.0));
    let red = materials.add(Color::linear_rgb(1.0, 0.0, 0.0));

    let weapon = commands
        .spawn((
            GunState::Empty,
            AttachedGunAmmunitionStorage(GunAmmunitionStorage {
                amount: 10,
                ..default()
            }),
        ))
        .id();
    commands.spawn((
        Player,
        FirstPersonCameraTarget::new(1.5),
        Mesh3d(capsule.clone()),
        MeshMaterial3d(gold.clone()),
        EquippedWeapons::new(&[Some(weapon), None]),
    ));

    for x in -1..=1 {
        commands.spawn((
            Mesh3d(capsule.clone()),
            MeshMaterial3d(red.clone()),
            Transform::from_xyz(x as f32 * 2.5, 0.0, -10.0),
        ));
    }
}
