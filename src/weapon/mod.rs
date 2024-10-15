use crate::input::GameplayInput;
use crate::prelude::*;

pub mod gun;

use gun::GunPlugin;

pub struct WeaponPlugin;
impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<GunPlugin>() {
            app.add_plugins(GunPlugin);
        }

        app.add_systems(Update, update_weapon_state);
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

fn update_weapon_state(mut weapons: Query<&mut WeaponState>, input: Res<GameplayInput>) {
    for mut state in weapons.iter_mut() {
        *state = match (state.clone(), input.use_weapon) {
            (WeaponState::Idle, true) => WeaponState::JustActivated,
            (_, true) => WeaponState::Active,
            (_, false) => WeaponState::Idle,
        };

        println!("NEW STATE: {state:?}");
    }
}
