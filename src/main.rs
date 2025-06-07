use bevy::prelude::*;
use rand::random;

const N_BODIES: usize = 10000;
const WORLD_RADIUS: f32 = 100.0;

// Const below should not be changed, rather change WORLD_RADIUS and N_BODIES.
const CAMERA_DISTANCE: f32 = WORLD_RADIUS * 2.0;
const CAMERA_SPEED: f32 = 50.0 * CAMERA_DISTANCE / 100.0;

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
    // -------------------------------------------------------------------
    // 1.  Camera
    // -------------------------------------------------------------------
    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        AmbientLight {
            color: Color::WHITE,
            brightness: 10_000.0,
            affects_lightmapped_meshes: false,
        },
        Transform::from_xyz(0.0, CAMERA_DISTANCE, -CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // -------------------------------------------------------------------
    // 2.  Sample points *inside* the sphere  (rejection sampling)
    // -------------------------------------------------------------------
    let r2 = WORLD_RADIUS * WORLD_RADIUS; // distance² threshold
    let mut spawned = 0;

    while spawned < N_BODIES {
        // candidate in the enclosing cube
        let x = random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;
        let y = random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;
        let z = random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;

        // keep it only if it lies inside the sphere
        if x * x + y * y + z * z > r2 {
            continue; // reject, try again
        }

        // accept
        let pos = Vec3::new(x, y, z);
        commands.spawn((
            Name::new(format!("Body {}", spawned)),
            Mesh3d(meshes.add(Sphere::default().mesh())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.8),
                perceptual_roughness: 0.5,
                metallic: 0.0,
                ..default()
            })),
            Transform::from_translation(pos),
        ));

        spawned += 1;
    }
}

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn((
//         Name::new("Camera"),
//         Camera3d::default(),
//         AmbientLight {
//             color: Color::WHITE,
//             brightness: 10_000.0,
//             affects_lightmapped_meshes: false,
//         },
//         Transform::from_xyz(0.0, CAMERA_DISTANCE, -CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
//     ));

//     // Spawn a number of random bodies in the world.
//     for i in 0..N_BODIES {
//         let x = rand::random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;
//         let y = rand::random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;
//         let z = rand::random::<f32>() * WORLD_RADIUS * 2.0 - WORLD_RADIUS;

//         commands.spawn((
//             Name::new(format!("Body {i}")),
//             Mesh3d(meshes.add(Sphere::default().mesh())),
//             MeshMaterial3d(materials.add(StandardMaterial {
//                 base_color: Color::srgb(0.1, 0.1, 0.8),
//                 perceptual_roughness: 0.5,
//                 metallic: 0.0,
//                 ..default()
//             })),
//             Transform::from_xyz(x, y, z),
//         ));
//     }
// }

fn move_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera: Single<&mut Transform, With<Camera3d>>,
) {
    // --------------------------------------------------------------------
    // 0.  Parameters
    // --------------------------------------------------------------------
    let mut lin_speed = CAMERA_SPEED; // m · s⁻¹ for W / S
    let mut ang_speed = CAMERA_SPEED; // “surface” speed for A/D/Q/E

    if keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight) {
        lin_speed *= 3.0;
        ang_speed *= 3.0;
    }

    let dt = time.delta_secs();
    let radius = camera.translation.length().max(1e-6); // avoid /0

    // Convert the desired tangential speed into an angle step
    //     v = ω·r  ⇒  ω = v / r
    let dtheta = (ang_speed / radius) * dt; // radians per frame

    // --------------------------------------------------------------------
    // 1.  Azimuth (A / D) – rotate around the global Y axis
    // --------------------------------------------------------------------
    let mut yaw = 0.0;
    if keyboard.pressed(KeyCode::KeyA) {
        yaw += dtheta;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        yaw -= dtheta;
    }
    if yaw != 0.0 {
        camera.rotate_around(Vec3::ZERO, Quat::from_rotation_y(yaw));
    }

    // --------------------------------------------------------------------
    // 2.  Elevation (Q / E) – rotate around the *current* right axis
    // --------------------------------------------------------------------
    let mut pitch = 0.0;
    if keyboard.pressed(KeyCode::KeyQ) {
        pitch += dtheta;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        pitch -= dtheta;
    }
    if pitch != 0.0 {
        // `camera.right()` must be taken *after* the yaw above, otherwise the
        // axes are one frame out of date.
        let right_axis = *camera.right();
        camera.rotate_around(Vec3::ZERO, Quat::from_axis_angle(right_axis, pitch));
    }

    // --------------------------------------------------------------------
    // 3.  Zoom (W / S) – move along the forward/backward axis
    // --------------------------------------------------------------------
    let mut move_vec = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) {
        move_vec += *camera.forward();
    }
    if keyboard.pressed(KeyCode::KeyS) {
        move_vec -= *camera.forward();
    }
    if move_vec != Vec3::ZERO {
        camera.translation += move_vec.normalize() * lin_speed * dt;
    }

    // --------------------------------------------------------------------
    // 4.  Re-orient so the view always points at the origin.
    // --------------------------------------------------------------------
    camera.look_at(Vec3::ZERO, Vec3::Y);
}
