use bevy::prelude::*;

const N_BODIES: usize = 300;
const WORLD_RADIUS: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NBodiesPlugin)
        .run();
}

struct NBodiesPlugin;
impl Plugin for NBodiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_camera);
        app.insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        AmbientLight {
            color: Color::WHITE,
            brightness: 10000.0,
            affects_lightmapped_meshes: false,
        },
        Transform::from_xyz(0.0, 30.0, -50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Name::new("Sphere"),
        Mesh3d(meshes.add(Sphere::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn move_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    let mut speed = 50.0;
    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        speed *= 3.0;
    }

    let forward = *camera.forward(); // points where the camera looks :contentReference[oaicite:0]{index=0}
    let right = *camera.right(); // 90° to the camera’s right
    let top = *camera.up(); // 90° to the camera’s top

    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        movement += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        movement -= forward;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        movement += right;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        movement -= right;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        movement += top;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        movement -= top;
    }

    if movement != Vec3::ZERO {
        camera.translation += movement.normalize() * speed * time.delta_secs();
        camera.look_at(Vec3::ZERO, Vec3::Y);
    }
}
