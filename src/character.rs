use crate::prelude::*;

const CAPACITY: usize = 10;
type WeaponSlot = Option<Entity>;

#[derive(Component, Debug)]
pub struct EquippedWeapons {
    slots: [WeaponSlot; CAPACITY],
    size: usize,
    current: usize,
}

impl EquippedWeapons {
    pub fn new(size: usize) -> Self {
        assert!(size > 0 && size <= CAPACITY);
        Self {
            slots: [None; CAPACITY],
            current: 0,
            size,
        }
    }

    /// Returns index of currently selected weapon slot.
    pub fn current_index(&self) -> usize {
        self.current
    }

    /// Returns currently selected weapon slot.
    pub fn current_slot(&self) -> WeaponSlot {
        self.slots[self.current]
    }

    /// Returns slot at given index or None if index is out of bounds.
    pub fn get_slot(&self, index: usize) -> Option<WeaponSlot> {
        if index >= self.size {
            None
        } else {
            self.slots.get(index).cloned()
        }
    }

    /// Switches current weapon slot to slot at given index. Returns new slot or None if index is out of bounds.
    pub fn switch(&mut self, index: usize) -> Option<WeaponSlot> {
        if index >= self.size {
            return None;
        };

        if self.current != index {
            self.current = index;
            info!("Switched current weapon slot to {index}");
        }
        Some(self.current_slot())
    }
}

impl Default for EquippedWeapons {
    fn default() -> Self {
        Self::new(CAPACITY)
    }
}
