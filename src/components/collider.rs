use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Collider {
    pub radius: f32,
    pub collisions: Vec<Entity>,
}

impl Collider {
    pub fn from_radius(radius: f32) -> Self {
        Collider {
            radius,
            collisions: Vec::new(),
        }
    }
}
