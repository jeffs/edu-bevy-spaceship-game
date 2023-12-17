use bevy::prelude::*;

use crate::components::{Acceleration, Collider, Velocity};

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
    pub collider: Collider,
}
