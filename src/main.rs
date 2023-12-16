// #![allow(dead_code, unused_variables)]

mod components;
mod plugins;

use bevy::prelude::*;
use plugins::{CameraPlugin, DebugPlugin, MovementPlugin, SpaceshipPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((CameraPlugin, DebugPlugin, MovementPlugin, SpaceshipPlugin))
        .run();
}
