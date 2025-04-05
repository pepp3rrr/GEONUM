use blue_engine::header::*;
use geonum_common::{FromCSV as _, IntoMesh};
use triangle_mesh::TriangleMesh;

mod triangle_mesh;

const MOVE_SPEED: f32 = 10.0;

fn main() {
    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let mesh = TriangleMesh::from_csv("tp9/data/cube.off").subdivide();

    let (vertices, indices) = mesh.into_mesh();
    engine.objects.new_object(
        "figure",
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

    let mut radius = 10.0;

    let start = std::time::SystemTime::now();
    let mut t0 = start.elapsed().unwrap();

    engine
        .update_loop(
            move |_renderer, _window, _objects, events, camera, _plugins| {
                let delta = (start.elapsed().unwrap() - t0).as_secs_f32();
                t0 = start.elapsed().unwrap();

                if events.key_held(KeyCode::KeyW) {
                    radius -= MOVE_SPEED * delta;
                }
                if events.key_held(KeyCode::KeyS) {
                    radius += MOVE_SPEED * delta;
                }

                let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
                let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
                let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;

                camera.set_position([camx, camy, camz]);
            },
        )
        .expect("Error during update loop");
}
