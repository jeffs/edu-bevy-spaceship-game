pub mod assets;
pub mod asteroids;
pub mod camera;
pub mod collisions;
pub mod debug;
pub mod movement;
pub mod spaceship;

pub use assets::AssetLoaderPlugin;
pub use camera::CameraPlugin;
pub use collisions::CollisionDetectionPlugin;
pub use debug::DebugPlugin;
pub use movement::MovementPlugin;
pub use spaceship::SpaceshipPlugin;
