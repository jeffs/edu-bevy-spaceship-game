use bevy::prelude::*;

use crate::bundles::MovingObjectBundle;
use crate::components::{Acceleration, Velocity};

use super::assets::SceneAssets;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0); // m
const SPEED: f32 = 25.0; // m/s
const ROTATION_SPEED: f32 = 2.5; // radians

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

fn get_rotation(keys: &Res<Input<KeyCode>>, time: &Res<Time>) -> f32 {
    let left = keys.pressed(KeyCode::H);
    let right = keys.pressed(KeyCode::L);
    let rotation = left as i8 - right as i8;
    rotation as f32 * ROTATION_SPEED * time.delta_seconds()
}

fn get_roll(keys: &Res<Input<KeyCode>>, time: &Res<Time>) -> f32 {
    let left = keys.pressed(KeyCode::ShiftLeft);
    let right = keys.pressed(KeyCode::ShiftRight);
    let roll = left as i8 - right as i8;
    roll as f32 * ROTATION_SPEED * time.delta_seconds()
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = query.single_mut();

    // `-` because the asset is backward.
    velocity.0 = -transform.forward() * get_speed(&keys);
    transform.rotate_y(get_rotation(&keys, &time));
    transform.rotate_local_z(-get_roll(&keys, &time));
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, spaceship_movement_controls);
    }
}
