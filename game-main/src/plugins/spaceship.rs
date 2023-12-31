use bevy::prelude::*;

use crate::bundles::MovingObjectBundle;
use crate::components::{Acceleration, Collider, Velocity};

use super::assets::SceneAssets;

const SHIP_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0); // m
const SHIP_SPEED: f32 = 25.0; // m/s
const SHIP_ROTATION_SPEED: f32 = 2.5; // radians

const MISSILE_OFFSET: f32 = 7.5; // m
const MISSILE_SPEED: f32 = 50.0; // m/s

// For collision detection.
const SHIP_RADIUS: f32 = 4.0; // m
const MISSILE_RADIUS: f32 = 1.0; // m

#[derive(Component, Debug)]
struct Missile;

#[derive(Component, Debug)]
struct Spaceship;

fn spawn_spaceship(mut commands: Commands, assets: Res<SceneAssets>) {
    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(Vec3::ZERO),
            acceleration: Acceleration(Vec3::ZERO),
            model: SceneBundle {
                scene: assets.spaceship.clone(),
                transform: Transform::from_translation(SHIP_TRANSLATION),
                ..default()
            },
            collider: Collider::from_radius(SHIP_RADIUS),
        },
        Spaceship,
    ));
}

fn handle_collision(mut commands: Commands, query: Query<(Entity, &Collider), With<Spaceship>>) {
    let Ok((entity, collider)) = query.get_single() else {
        return;
    };
    let Some(ship) = commands.get_entity(entity) else {
        return;
    };
    if collider.collisions.is_empty() {
        return;
    }
    ship.despawn_recursive();
}

fn get_speed(keys: &Res<Input<KeyCode>>) -> f32 {
    let forward = keys.pressed(KeyCode::K);
    let backward = keys.pressed(KeyCode::J);
    let direction = forward as i8 - backward as i8;
    direction as f32 * SHIP_SPEED
}

fn get_rotation(keys: &Res<Input<KeyCode>>, time: &Res<Time>) -> f32 {
    let left = keys.pressed(KeyCode::H);
    let right = keys.pressed(KeyCode::L);
    let rotation = left as i8 - right as i8;
    rotation as f32 * SHIP_ROTATION_SPEED * time.delta_seconds()
}

fn get_roll(keys: &Res<Input<KeyCode>>, time: &Res<Time>) -> f32 {
    let left = keys.pressed(KeyCode::ShiftLeft) as i8;
    let right = keys.pressed(KeyCode::ShiftRight) as i8;
    (left - right) as f32 * SHIP_ROTATION_SPEED * time.delta_seconds()
}

fn spaceship_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Spaceship>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    // `-` because the asset is backward.
    velocity.0 = -transform.forward() * get_speed(&keys);
    transform.rotate_y(get_rotation(&keys, &time));
    transform.rotate_local_z(-get_roll(&keys, &time));
}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Spaceship>>,
    keys: Res<Input<KeyCode>>,
    assets: Res<SceneAssets>,
) {
    let Ok(ship_transform) = query.get_single() else {
        return;
    };
    let ship_translation = ship_transform.translation;
    let forward = -ship_transform.forward(); // direction of local Z axis
    if keys.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity(forward * MISSILE_SPEED),
                acceleration: Acceleration(Vec3::ZERO),
                collider: Collider::from_radius(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: assets.missile.clone(),
                    transform: Transform::from_translation(
                        ship_translation + forward * MISSILE_OFFSET,
                    ),
                    ..default()
                },
            },
            Missile,
        ));
    }
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship).add_systems(
            Update,
            (
                handle_collision,
                spaceship_movement_controls,
                spaceship_weapon_controls,
            ),
        );
    }
}
