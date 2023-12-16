use bevy::prelude::*;

use crate::components::{Acceleration, Velocity};

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SceneBundle,
}
