use bevy::prelude::*;

const DESPAWN_DISTANCE: f32 = 100.0; // m

fn despawn_far_away_entities(
    mut commands: Commands,
    query: Query<(Entity, &GlobalTransform)>,
    world: &World,
) {
    for (entity, transform) in query
        .iter()
        .filter(|(_, transform)| transform.translation().distance(Vec3::ZERO) > DESPAWN_DISTANCE)
    {
        let Some(mut thing) = commands.get_entity(entity) else {
            // The entity was already despawned.
            continue;
        };

        // If we use despawn_recursive, as in zymarku's tutorial, we get warnings about some (but
        // not all) of the entities already being gone. My wild guess is that since the parent and
        // child are both returned by our query, we sometimes delete the child first; so then when
        // we try to delete the parent, despawn_recursive freaks out because its children are dead.
        // What I don't get is:
        //
        // 1. How are you supposed to handle this situation?  Don't despawn_recursive?  Skip
        //    non-root entities, for some definition of root?
        // 2. Why is this happening in this silly little spaceship game, which has no explicit
        //    parent/child relationships in the entity hierarchy, AFAICS?
        thing.despawn();
    }
}

pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_far_away_entities);
    }
}
