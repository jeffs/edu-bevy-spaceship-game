use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);
