use bezier_surface::PiecewiseBezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    wgpu, KeyCode, ObjectSettings, RotateAmount, RotateAxis, ShaderSettings, Vector3,
};
use geonum_common::FromCSV as _;
use render::IntoMesh;

mod bezier_surface;
mod render;

const MOVE_SPEED: f32 = 10f32;
const SAMPLES_PER_AXIS: u32 = 12;

fn main() {
    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let surface = PiecewiseBezierSurface::from_csv("tp6/data/teapot.bpt");

    let mut figure_patch_meshes = Vec::new();
    for (n, patch) in surface.patches.into_iter().enumerate() {
        let mut samples = Vec::new();
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
            samples.push(row);
        }

        let (vertices, indices) = samples.into_mesh();
        let id = format!("figure-{}", n);

        engine.objects.new_object(
            id.clone(),
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

        figure_patch_meshes.push(id);
    }

    let mut time = std::time::Instant::now();

    engine
        .update_loop(
            move |_renderer, _window, _objects, events, camera, _plugins| {
                let delta = time.elapsed().as_secs_f32();

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
