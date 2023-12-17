use bevy::prelude::*;

use crate::bundles::MovingObjectBundle;
use crate::components::{Acceleration, Velocity};

use super::assets::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPEED: f32 = 25.0;
const ROTATION_SPEED: f32 = 2.5;

#[derive(Component, Debug)]
struct Spaceship;

fn spawn_spaceship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(Vec3::ZERO),
            acceleration: Acceleration(Vec3::ZERO),
            model: SceneBundle {
                scene: assets.spaceship.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Spaceship,
    ));
}

fn get_speed(keys: &Res<Input<KeyCode>>) -> f32 {
    let forward = keys.pressed(KeyCode::K);
    let backward = keys.pressed(KeyCode::J);
    let direction = forward as i8 - backward as i8;
    direction as f32 * SPEED
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (transform, mut velocity) = query.single_mut();
    let (rotation, roll) = (0.0, 0.0);

    // `-` because the asset is backward.
    velocity.0 = -transform.forward() * get_speed(&keys);
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}
