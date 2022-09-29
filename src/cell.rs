use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

use crate::elements::Element;

#[derive(Default, Component, Inspectable, Clone)]
pub struct Cell {
    pub element: Element,
}

impl Cell {
    pub fn new(element: Element) -> Self { Self { element } }
}
