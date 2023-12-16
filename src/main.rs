mod components;
mod plugins;

#[cfg(feature = "debug")]
use crate::plugins::DebugPlugin;
use crate::plugins::{CameraPlugin, MovementPlugin, SpaceshipPlugin};

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    let app = app.add_plugins(DefaultPlugins).add_plugins((
        CameraPlugin,
        MovementPlugin,
        SpaceshipPlugin,
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugin);

    app.run();
}
