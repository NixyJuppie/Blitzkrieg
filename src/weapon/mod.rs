use crate::prelude::*;

pub mod gun;

use gun::GunPlugin;

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<GunPlugin>() {
            app.add_plugins(GunPlugin);
        }
    }
}

#[derive(Component, Default, Debug)]
pub enum WeaponState {
    #[default]
    Idle,
    Active,
}
