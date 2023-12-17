mod assets;
mod asteroids;
mod collisions;
mod debug;
mod despawn;
mod movement;
mod spaceship;

pub use assets::AssetLoaderPlugin;
pub use asteroids::AsteroidPlugin;
pub use collisions::CollisionDetectionPlugin;
pub use debug::DebugPlugin;
pub use despawn::DespawnPlugin;
pub use movement::MovementPlugin;
pub use spaceship::SpaceshipPlugin;
