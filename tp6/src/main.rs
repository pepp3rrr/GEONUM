use bezier_surface::PiecewiseBezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    wgpu, KeyCode, ObjectSettings, RotateAmount, RotateAxis, ShaderSettings, Vector2, Vector3,
    Vertex,
};
use geonum_common::FromCSV as _;

mod bezier_surface;

const MOVE_SPEED: f32 = 10f32;
const SAMPLES_PER_AXIS: u32 = 12;

fn main() {
    let surface = PiecewiseBezierSurface::from_csv("tp6/data/teapot.bpt");

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut i = 0;
    for patch in surface.patches {
        let mut grid = Vec::new();
        for i in 0..(SAMPLES_PER_AXIS + 1) {
            let mut row = Vec::new();
            for j in 0..(SAMPLES_PER_AXIS + 1) {
                let (u, v) = (
                    i as f32 / SAMPLES_PER_AXIS as f32,
                    j as f32 / SAMPLES_PER_AXIS as f32,
                );
                let sample = patch.evalutate(u, v);
                row.push(sample);
            }
            grid.push(row);
        }
        // grid = patch.control;

        for x in 0..grid.len() {
            let row = grid[x].clone();
            for y in 0..row.len() {
                let b_x_y = row[y];
                if x + 1 < grid.len() && y + 1 < row.len() {
                    let b_xp1_y = grid[x + 1][y];
                    let b_x_yp1 = row[y + 1];
                    let b_xp1_yp1 = grid[x + 1][y + 1];

                    vertices.push(Vertex {
                        position: Vector3::new(b_x_y.x(), b_x_y.z(), b_x_y.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_x_yp1.x(), b_x_yp1.z(), b_x_yp1.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_y.x(), b_xp1_y.z(), b_xp1_y.y()),
                        uv: Vector2::ZERO,
                        normal: Vector3::ZERO,
                    });
                    vertices.push(Vertex {
                        position: Vector3::new(b_xp1_yp1.x(), b_xp1_yp1.z(), b_xp1_yp1.y()),
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
        ObjectSettings {
            shader_settings: ShaderSettings {
                polygon_mode: wgpu::PolygonMode::Line,
                cull_mode: None,
                ..Default::default()
            },
            ..Default::default()
        },
        &mut engine.renderer,
    );

    let mut time = std::time::Instant::now();

    engine
        .update_loop(
            move |_renderer, _window, objects, events, camera, _plugins| {
                let delta = time.elapsed().as_secs_f32();

                let figure = objects.get_mut("Figure").unwrap();

                // figure.rotate(RotateAmount::Radians(delta / 3.0), RotateAxis::X);
                figure.rotate(RotateAmount::Radians(delta), RotateAxis::Y);
                // figure.rotate(RotateAmount::Radians(delta / 2.0), RotateAxis::Z);

                let main_camera = camera.cameras.get("main").expect("No main camera");
                let camera_position = main_camera.position.clone();

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
