use super::WeaponState;
use crate::prelude::*;

pub struct GunPlugin;
impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_state);
    }
}

#[derive(Component, Default, Debug)]
pub struct LoadingMechanism {
    pub automatic: bool,
    pub duration: f32,
}

#[derive(Component, Default, Debug)]
pub struct FiringMechanism {
    pub automatic: bool,
    pub duration: f32,
}

#[derive(Clone, Default, Debug)]
pub struct Ammunition {
    pub bullet: ProjectileDefinition,
    pub casing: Option<ProjectileDefinition>,
}

#[derive(Clone, Default, Debug)]
pub struct ProjectileDefinition {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

#[derive(Default, Debug)]
pub struct AmmunitionStorage {
    pub ammunition: Ammunition,
    pub amount: u8,
}

#[derive(Component, Default, Debug)]
pub struct AttachedAmmunitionStorage(pub Option<AmmunitionStorage>);

#[derive(Component, Default, Debug)]
#[require(
    WeaponState,
    LoadingMechanism,
    FiringMechanism,
    AttachedAmmunitionStorage
)]
pub enum GunState {
    #[default]
    Empty,
    Loading {
        ammunition: Ammunition,
        timer: Timer,
    },
    Ready(Ammunition),
    Firing {
        ammunition: Ammunition,
        timer: Timer,
    },
}

fn update_state(
    mut weapons: Query<(
        &mut GunState,
        &mut AttachedAmmunitionStorage,
        &LoadingMechanism,
        &FiringMechanism,
        &WeaponState,
        &GlobalTransform,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (mut gun, mut ammunition_storage, loading, firing, weapon, transform) in weapons.iter_mut()
    {
        match gun.as_mut() {
            GunState::Empty => {
                let Some(ref mut ammunition_storage) = ammunition_storage.0 else {
                    continue;
                };
                if ammunition_storage.amount == 0 {
                    todo!("No ammo");
                }

                if *weapon == WeaponState::JustActivated
                    || (loading.automatic && *weapon == WeaponState::Active)
                {
                    ammunition_storage.amount -= 1;
                    info!("Loading {:?}", ammunition_storage.ammunition);
                    *gun = GunState::Loading {
                        ammunition: ammunition_storage.ammunition.clone(),
                        timer: Timer::from_seconds(loading.duration, TimerMode::Once),
                    };
                }
            }
            GunState::Loading { ammunition, timer } => {
                timer.tick(time.delta());
                if timer.finished() {
                    info!("Ready {ammunition:?}");
                    *gun = GunState::Ready(ammunition.clone());
                }
            }
            GunState::Ready(ammunition) => {
                if *weapon == WeaponState::JustActivated
                    || (firing.automatic && *weapon == WeaponState::Active)
                {
                    info!("Firing {ammunition:?}");
                    *gun = GunState::Firing {
                        ammunition: ammunition.clone(),
                        timer: Timer::from_seconds(firing.duration, TimerMode::Once),
                    };
                }
            }
            GunState::Firing { ammunition, timer } => {
                timer.tick(time.delta());
                if timer.finished() {
                    info!("Fired {ammunition:?}");

                    let mut bullet_transform = transform.compute_transform();
                    bullet_transform.translation += *bullet_transform.forward();
                    commands.spawn((
                        bullet_transform,
                        Mesh3d(ammunition.bullet.mesh.clone()),
                        MeshMaterial3d(ammunition.bullet.material.clone()),
                    ));

                    if let Some(ref casing) = ammunition.casing {
                        let mut casing_transform = transform.compute_transform();
                        casing_transform.translation += *casing_transform.up();

                        commands.spawn((
                            casing_transform,
                            Mesh3d(casing.mesh.clone()),
                            MeshMaterial3d(casing.material.clone()),
                        ));
                    }

                    // TODO: Projectile physics
                    // Blocked because of bleeding-edge bevy xD

                    *gun = GunState::Empty;
                }
            }
        };
    }
}
