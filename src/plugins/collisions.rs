use bevy::{prelude::*, utils::HashMap};

use crate::components::Collider;

struct QueryItem {
    entity: Entity,
    translation: Vec3,
    radius: f32,
}

impl QueryItem {
    fn from_tuple(
        (entity, transform, collider): (Entity, &GlobalTransform, &Collider),
    ) -> QueryItem {
        QueryItem {
            entity,
            translation: transform.translation(),
            radius: collider.radius,
        }
    }

    fn overlaps(&self, item: &QueryItem) -> bool {
        self.entity != item.entity
            && self.translation.distance(item.translation) < self.radius + item.radius
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let query_items = || query.iter().map(QueryItem::from_tuple);

    // Detect collisions.  O(N*N)
    let mut collisions: HashMap<Entity, Vec<Entity>> = HashMap::new();
    for a in query_items() {
        for b in query_items() {
            if a.overlaps(&b) {
                collisions.entry(a.entity).or_default().push(b.entity);
            }
        }
    }

    // Update colliders.  O(N)
    for (entity, _, mut collider) in query.iter_mut() {
        collider.collisions = collisions.remove(&entity).take().unwrap_or_default();
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}
