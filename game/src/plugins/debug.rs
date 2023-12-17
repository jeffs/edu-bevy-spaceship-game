use bevy::prelude::*;

fn print_transform(query: Query<(Entity, &Transform)>) {
    for (entity, position) in query.iter() {
        info!("Entity {entity:?} is at {position:?}");
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_transform);
    }
}
