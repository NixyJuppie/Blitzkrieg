use blitzkrieg::camera::{FirstPersonCamera, FirstPersonCameraTarget};
use blitzkrieg::character::{EquippedWeapons, WeaponSlot};
use blitzkrieg::player::Player;
use blitzkrieg::prelude::*;
use blitzkrieg::weapon::gun::{
    Ammunition, AmmunitionStorage, AttachedAmmunitionStorage, FiringMechanism, GunState,
    LoadingMechanism, ProjectileDefinition,
};
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

    let weapons = spawn_weapons(&mut commands, &mut meshes, &mut materials);
    commands
        .spawn((
            Player,
            FirstPersonCameraTarget::new(1.5),
            Mesh3d(capsule.clone()),
            MeshMaterial3d(gold.clone()),
            EquippedWeapons::new(&weapons),
        ))
        .add_children(&weapons.iter().filter_map(|w| *w).collect::<Vec<Entity>>()[..]);

    for x in -1..=1 {
        commands.spawn((
            Mesh3d(capsule.clone()),
            MeshMaterial3d(red.clone()),
            Transform::from_xyz(x as f32 * 2.5, 0.0, -10.0),
        ));
    }
}

fn spawn_weapons(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> [WeaponSlot; 3] {
    let ammunition = Ammunition {
        bullet: ProjectileDefinition {
            mesh: meshes.add(Capsule3d::new(0.01, 0.01)),
            material: materials.add(Color::linear_rgb(1.0, 0.9, 0.0)),
        },
        casing: Some(ProjectileDefinition {
            mesh: meshes.add(Capsule3d::new(0.02, 0.02)),
            material: materials.add(Color::linear_rgb(0.7, 0.7, 0.7)),
        }),
    };

    [
        // Manual loading and firing - Mosin-Nagant
        Some(
            commands
                .spawn((
                    GunState::Empty,
                    AttachedAmmunitionStorage(Some(AmmunitionStorage {
                        amount: 10,
                        ammunition: ammunition.clone(),
                    })),
                    LoadingMechanism {
                        automatic: false,
                        duration: 0.1,
                    },
                    FiringMechanism {
                        automatic: false,
                        duration: 0.1,
                    },
                ))
                .id(),
        ),
        // Automatic loading with manual firing - M1 Garand
        Some(
            commands
                .spawn((
                    GunState::Empty,
                    AttachedAmmunitionStorage(Some(AmmunitionStorage {
                        amount: 10,
                        ammunition: ammunition.clone(),
                    })),
                    LoadingMechanism {
                        automatic: true,
                        duration: 0.1,
                    },
                    FiringMechanism {
                        automatic: false,
                        duration: 0.1,
                    },
                ))
                .id(),
        ),
        // Automatic loading and firing - MP40
        Some(
            commands
                .spawn((
                    GunState::Empty,
                    AttachedAmmunitionStorage(Some(AmmunitionStorage {
                        amount: 20,
                        ammunition: ammunition.clone(),
                    })),
                    LoadingMechanism {
                        automatic: true,
                        duration: 0.1,
                    },
                    FiringMechanism {
                        automatic: true,
                        duration: 0.1,
                    },
                ))
                .id(),
        ),
    ]
}
