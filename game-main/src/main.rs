#![allow(dead_code, unused_mut, unused_variables)]

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
use bevy::window::{PrimaryWindow, Window};

pub fn hide_cursor(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };
    window.cursor.visible = false;
}

fn main() {
    let mut app = App::new();
    let app = app
        .add_plugins(DefaultPlugins)
        .add_plugins((
            AssetLoaderPlugin,
            AsteroidPlugin,
            CameraPlugin,
            CollisionDetectionPlugin,
            DespawnPlugin,
            MovementPlugin,
            SpaceshipPlugin,
        ))
        .add_systems(PostStartup, hide_cursor);

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
