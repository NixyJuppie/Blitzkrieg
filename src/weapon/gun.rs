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
    pub bullet: (), // TODO
    pub casing: (), // TODO
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
    )>,
    time: Res<Time>,
) {
    for (mut gun, mut ammunition_storage, loading, firing, weapon) in weapons.iter_mut() {
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
                    *gun = GunState::Empty;
                }
            }
        };
    }
}
