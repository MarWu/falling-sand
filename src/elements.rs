use bevy_inspector_egui::Inspectable;

#[derive(Default, Inspectable, Clone)]
pub enum Element {
    #[default] Air,
    Sand,
    Stone,
}

impl Element {
    pub fn get_color(&self) -> &[u8; 4] {
        match *self {
            Element::Air => &[255, 255, 0, 255],
            Element::Sand => &[255, 0, 255, 255],
            Element::Stone => &[0, 0, 0, 255],
        }
    }
}
