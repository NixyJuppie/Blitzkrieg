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

/// General state of weapon/tool, the thing in hands.
/// Maybe ToolState would be a better name?
#[derive(Component, Default, Debug)]
pub enum WeaponState {
    #[default]
    Idle,
    Active,
}
