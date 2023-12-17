#![allow(dead_code, unused_mut, unused_variables)]

mod bundles;
mod components;
mod plugins;

#[cfg(feature = "debug")]
use crate::plugins::DebugPlugin;
use crate::plugins::{CameraPlugin, MovementPlugin, SpaceshipPlugin};

use bevy::prelude::*;
use plugins::{asteroids::AsteroidPlugin, AssetLoaderPlugin, CollisionDetectionPlugin};

fn main() {
    let mut app = App::new();
    let app = app.add_plugins(DefaultPlugins).add_plugins((
        AssetLoaderPlugin,
        AsteroidPlugin,
        CameraPlugin,
        CollisionDetectionPlugin,
        MovementPlugin,
        SpaceshipPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
