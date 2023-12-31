mod bundles;
mod components;
mod plugins;

#[cfg(feature = "debug")]
use crate::plugins::DebugPlugin;
use crate::plugins::{
    AssetLoaderPlugin, AsteroidPlugin, CollisionDetectionPlugin, DespawnPlugin, MovementPlugin,
    SpaceshipPlugin,
};

use game_camera::CameraPlugin;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    let app = app.add_plugins(DefaultPlugins).add_plugins((
        AssetLoaderPlugin,
        AsteroidPlugin,
        CameraPlugin,
        CollisionDetectionPlugin,
        DespawnPlugin,
        MovementPlugin,
        SpaceshipPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
