use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;
const CAMERA_SPEED: f32 = 80.0; // m/s

#[derive(Component)]
struct Camera;

fn camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, CAMERA_DISTANCE, 0.0)
                .looking_at(Vec3::ZERO, Vec3::Z),
            ..default()
        },
        Camera,
    ));
}

fn camera_movement(
    mut query: Query<&mut Transform, With<Camera>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let to_speed = |key1, key2| {
        (keys.pressed(key1) as i8 - keys.pressed(key2) as i8) as f32
            * CAMERA_SPEED
            * time.delta_seconds()
    };
    let mut transform = query.single_mut();
    transform.translation.x += to_speed(KeyCode::A, KeyCode::D);
    transform.translation.y += to_speed(KeyCode::Q, KeyCode::E);
    transform.translation.z += to_speed(KeyCode::W, KeyCode::S);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
            .insert_resource(AmbientLight {
                color: Color::default(),
                brightness: 0.75,
            })
            .add_systems(Startup, camera)
            .add_systems(Update, camera_movement);
    }
}
