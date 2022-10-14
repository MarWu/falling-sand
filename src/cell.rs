use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::elements::Element;

#[derive(Default, Component, Inspectable, Clone)]
pub struct Cell {
    pub element: Element,
}

impl Cell {
    pub fn new(element: Element) -> Self {
        Self { element }
    }
}

#[derive(Debug, Component)]
pub struct CellPos {
    x: u32,
    y: u32,
}

impl CellPos {
    pub fn to_index(&self, size: crate::chunk::ChunkSize) -> usize {
        ((self.y * size.x as u32) + self.x) as usize
    }
}

impl From<IVec2> for CellPos {
    fn from(vec: IVec2) -> Self {
        CellPos {
            x: vec.x as u32,
            y: vec.y as u32,
        }
    }
}

#[derive(Debug, Component)]
pub struct CellParent(pub Entity);

#[derive(Bundle)]
pub struct CellBundle {
    pos: CellPos,
    element: Element,
    parent_chunk: CellParent,
}
