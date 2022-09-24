use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Component, Inspectable, Clone, PartialEq)]
pub enum Element {
    #[default] Air,
    Sand,
    Stone,
}

impl Element {
    pub fn get_color(&self) -> &[u8; 4] {
        match *self {
            Element::Air => &[0, 204, 255, 120],
            Element::Sand => &[255, 255, 0, 255],
            Element::Stone => &[128, 128, 128, 255],
        }
    }
}
