use std::ops::Range;

use bevy::prelude::*;
use rand::Rng;

use crate::{
    bundles::MovingObjectBundle,
    components::{collider::Collider, Acceleration, Velocity},
};

use super::assets::SceneAssets;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0; // m
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0; // m
const SPEED: f32 = 5.0; // m/s
const ACCELERATION_MAGNITUDE: f32 = 1.0; // m/s/s
const SPAWN_TIME: f32 = 1.0; // s
const RADIUS: f32 = 2.0; // m
const ROTATION_SPEED: f32 = std::f32::consts::PI; // rad/s

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer(Timer);

fn rand_dir<R: Rng>(rng: &mut R) -> Vec3 {
    rand_xy(rng, -1.0..1.0, -1.0..1.0).normalize_or_zero()
}

fn rand_xy<R: Rng>(rng: &mut R, xs: Range<f32>, zs: Range<f32>) -> Vec3 {
    Vec3::new(rng.gen_range(xs), 0.0, rng.gen_range(zs))
}

fn handle_collision(mut commands: Commands, query: Query<(Entity, &Collider), With<Asteroid>>) {
    'asteroids: for (entity, collider) in query.iter() {
        let Some(asteroid) = commands.get_entity(entity) else {
            // This asteroid was already despawned.
            continue;
        };
        for other in collider
            .collisions
            .iter()
            .cloned()
            // Ignore collisions with other asteroids.
            .filter(|&other| !query.contains(other))
        {
            asteroid.despawn_recursive();
            continue 'asteroids;
        }
    }
}

fn rotate_asteroids(
    mut query: Query<(&Velocity, &GlobalTransform, &mut Transform), With<Asteroid>>,
    time: Res<Time>,
) {
    for (velocity, global_transform, mut transform) in query.iter_mut() {
        transform.rotate_axis(
            velocity.0.normalize(),
            ROTATION_SPEED * time.delta_seconds(),
        );
        // transform.rotate_local_z(ROTATION_SPEED * time.delta_seconds());
        debug_assert_eq!(transform.scale, Vec3::ONE);
        debug_assert_eq!(transform.translation.y, 0.0);
        debug_assert_eq!(global_transform.translation().y, 0.0);
    }
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    assets: Res<SceneAssets>,
) {
    spawn_timer.0.tick(time.delta());
    if !spawn_timer.0.just_finished() {
        return;
    }

    let rng = &mut rand::thread_rng();
    let translation = rand_xy(rng, SPAWN_RANGE_X, SPAWN_RANGE_Z);
    let velocity = rand_dir(rng) * SPEED;
    let acceleration = rand_dir(rng) * ACCELERATION_MAGNITUDE;

    commands.spawn((
        MovingObjectBundle {
            velocity: Velocity(velocity),
            acceleration: Acceleration(acceleration),
            model: SceneBundle {
                scene: assets.asteroid.clone(),
                transform: Transform::from_translation(translation),
                ..default()
            },
            collider: Collider::from_radius(RADIUS),
        },
        Asteroid,
    ));
}

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_TIME,
            TimerMode::Repeating,
        )))
        .add_systems(Update, (spawn_asteroid, handle_collision, rotate_asteroids));
    }
}
