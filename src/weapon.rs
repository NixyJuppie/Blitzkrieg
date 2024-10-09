use crate::prelude::*;

#[derive(Component, Default, Debug)]
pub enum WeaponState {
    #[default]
    Idle,
    Active,
}
