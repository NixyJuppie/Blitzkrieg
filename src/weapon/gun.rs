use super::WeaponState;
use crate::prelude::*;

pub struct GunPlugin;
impl Plugin for GunPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_state);
    }
}

#[derive(Component, Default, Debug)]
#[require(WeaponState)]
pub enum GunState {
    #[default]
    Idle,
}

fn update_state(mut weapons: Query<(&mut GunState, &WeaponState)>) {
    for weapon in weapons.iter_mut() {
        println!("{weapon:?}");
    }
}
