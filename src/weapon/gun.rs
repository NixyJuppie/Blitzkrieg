use super::WeaponState;
use crate::prelude::*;

pub struct GunPlugin;
impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_state);
    }
}

#[derive(Component, Default, Debug)]
pub struct GunLoadingMechanism {
    pub automatic: bool,
    pub duration: f32,
}

#[derive(Component, Default, Debug)]
pub struct GunFiringMechanism {
    pub ready: bool,
    // pub automatic: bool,
    // pub duration: f32,
}

#[derive(Clone, Default, Debug)]
pub struct GunAmmunition {
    // TODO
}

#[derive(Default, Debug)]
pub struct GunAmmunitionStorage {
    pub ammunition: GunAmmunition,
    pub amount: u8,
}

#[derive(Component, Default, Debug)]
pub struct AttachedGunAmmunitionStorage(pub GunAmmunitionStorage);

#[derive(Component, Default, Debug)]
#[require(
    WeaponState,
    GunLoadingMechanism,
    GunFiringMechanism,
    AttachedGunAmmunitionStorage
)]
pub enum GunState {
    #[default]
    Empty,
    Loading {
        ammunition: GunAmmunition,
        timer: Timer,
    },
    Ready,
    Firing,
}

fn update_mechanism_state(
    mut weapons: Query<(
        &mut GunLoadingMechanism,
        &mut GunFiringMechanism,
        &WeaponState,
    )>,
) {
    // for (mut loading, mut firing, state) in weapons.iter_mut() {
    //     match state {
    //         WeaponState::Idle => {
    //             firing.ready = true;
    //         }
    //         WeaponState::Active => todo!(),
    //     }
    // }
}

fn update_state(
    mut weapons: Query<(
        &mut GunState,
        &mut AttachedGunAmmunitionStorage,
        &GunLoadingMechanism,
        &GunFiringMechanism,
        &WeaponState,
    )>,
    time: Res<Time>,
) {
    for (mut gun, mut ammunition_storage, loading, firing, weapon) in weapons.iter_mut() {
        match gun.as_mut() {
            GunState::Empty => {
                if ammunition_storage.0.amount == 0 {
                    continue;
                }
                println!("{gun:?} {weapon:?} {loading:?}");

                if *weapon == WeaponState::JustActivated
                    || (loading.automatic && *weapon == WeaponState::Active)
                {
                    ammunition_storage.0.amount -= 1;
                    *gun = GunState::Loading {
                        ammunition: ammunition_storage.0.ammunition.clone(),
                        timer: Timer::from_seconds(loading.duration, TimerMode::Once),
                    };
                }
            }
            GunState::Loading {
                ammunition,
                ref mut timer,
            } => {
                timer.tick(time.delta());
                if timer.finished() {
                    *gun = GunState::Ready
                }
            }
            GunState::Ready => *gun = GunState::Empty,
            GunState::Firing => *gun = GunState::Empty,
        };
    }
}
