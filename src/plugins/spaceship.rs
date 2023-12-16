use bevy::prelude::*;

use crate::components::velocity::Velocity;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const STARTING_VELOCITY: Vec3 = Vec3::new(0.0, 0.0, 1.0);

#[derive(Bundle)]
struct SpaceshipBundle {
    velocity: Velocity,
    model: SceneBundle,
}

// fn spawn_spaceship(mut commands: Commands) {
//     commands.spawn((
//         SpatialBundle::default(),
//         Velocity(Vec3 {
//             x: 1.0,
//             y: 1.0,
//             z: 0.0,
//         }),
//     ));
// }

fn spawn_spaceship(mut commands: Commands, asset_sever: Res<AssetServer>) {
    commands.spawn(SpaceshipBundle {
        velocity: Velocity(STARTING_VELOCITY),
        model: SceneBundle {
            scene: asset_sever.load("Spaceship.glb#Scene0"),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    });
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}
