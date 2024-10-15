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
#[derive(Component, Clone, PartialEq, Default, Debug)]
pub enum WeaponState {
    #[default]
    Idle,
    JustActivated,
    Active,
}

impl WeaponState {
    pub fn next(&self, input: bool) -> Self {
        match (self, input) {
            (_, false) => Self::Idle,
            (Self::Idle, true) => Self::JustActivated,
            (Self::JustActivated, true) => Self::Active,
            (Self::Active, true) => Self::Active,
        }
    }
}
