use bevy::prelude::*;

use crate::bundles::MovingObjectBundle;
use crate::components::{Acceleration, Velocity};

use super::assets::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

fn spawn_spaceship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn(MovingObjectBundle {
        velocity: Velocity(STARTING_VELOCITY),
        acceleration: Acceleration(Vec3::ZERO),
        model: SceneBundle {
            scene: assets.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}
