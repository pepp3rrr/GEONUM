use bezier_surface::BezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    KeyCode, ObjectSettings, Vector2, Vector3, Vertex,
};
use geonum_common::FromCSV as _;

mod bezier_surface;

const MOVE_SPEED: f32 = 10f32;

fn main() {
    let surface = BezierSurface::from_csv("tp6/data/heart.bpt");

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut i = 0;
    for grid in surface.control {
        for x in 0..grid.len() {
            let row = grid[x].clone();
            for y in 0..row.len() {
                let b_x_y = row[y];
                if x + 1 < grid.len() && y + 1 < row.len() {
                    let b_xp1_y = grid[x + 1][y];
                    let b_x_yp1 = row[y + 1];
                    let b_xp1_yp1 = grid[x + 1][y + 1];

                    vertices.push(Vertex {
                        position: Vector3::new(b_x_y.x(), b_x_y.y(), b_x_y.z()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_x_yp1.x(), b_x_yp1.y(), b_x_yp1.z()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_y.x(), b_xp1_y.y(), b_xp1_y.z()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_yp1.x(), b_xp1_yp1.y(), b_xp1_yp1.z()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    indices.push(i);
                    indices.push(i + 1);
                    indices.push(i + 2);
                    indices.push(i + 2);
                    indices.push(i + 1);
                    indices.push(i + 3);
                    i += 4;
                }
            }
        }
    }

    engine.objects.new_object(
        "Figure",
        vertices,
        indices,
        ObjectSettings::default(),
        &mut engine.renderer,
    );

    let mut time = std::time::Instant::now();

    engine
        .update_loop(
            move |_renderer, _window, _objects, events, camera, _plugins| {
                let main_camera = camera.cameras.get("main").expect("No main camera");
                let camera_position = main_camera.position.clone();
                let delta = time.elapsed().as_secs_f32();

                let direction_vector = Vector3 {
                    x: if events.key_held(KeyCode::KeyA) {
                        -1.0
                    } else if events.key_held(KeyCode::KeyD) {
                        1.0
                    } else {
                        0.0
                    },
                    y: if events.key_held(KeyCode::ShiftLeft) {
                        -1.0
                    } else if events.key_held(KeyCode::Space) {
                        1.0
                    } else {
                        0.0
                    },
                    z: if events.key_held(KeyCode::KeyW) {
                        -1.0
                    } else if events.key_held(KeyCode::KeyS) {
                        1.0
                    } else {
                        0.0
                    },
                };

                camera.set_position(camera_position + direction_vector * MOVE_SPEED * delta);
                camera.set_target(camera_position - Vector3::UNIT_Z);

                time = std::time::Instant::now();
            },
        )
        .expect("Error during update loop");
}
