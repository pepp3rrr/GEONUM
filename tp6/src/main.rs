use bezier_surface::PiecewiseBezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    wgpu, ObjectSettings, ShaderSettings,
};
use geonum_common::FromCSV as _;
use render::IntoMesh;

mod bezier_surface;
mod render;

const SAMPLES_PER_AXIS: u32 = 12;
const COLORS: [(f32, f32, f32, f32); 6] = [
    (1.0, 0.0, 0.0, 1.0),
    (0.0, 1.0, 0.0, 1.0),
    (1.0, 1.0, 0.0, 1.0),
    (0.0, 0.0, 1.0, 1.0),
    (1.0, 0.0, 1.0, 1.0),
    (0.0, 1.0, 1.0, 1.0),
];

fn main() {
    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let surface = PiecewiseBezierSurface::from_csv("tp6/data/teapot.bpt");

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

        let id = format!("figure-{}", n);
        let (vertices, indices) = samples.into_mesh();
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

        let c = COLORS[n % COLORS.len()];
        engine
            .objects
            .get_mut(&id)
            .unwrap()
            .set_color(c.0, c.1, c.2, c.3);
    }

    let radius = 10.0;
    let start = std::time::SystemTime::now();

    engine
        .update_loop(
            move |_renderer, _window, _objects, _events, camera, _plugins| {
                let camx = start.elapsed().unwrap().as_secs_f32().sin() * radius;
                let camy = start.elapsed().unwrap().as_secs_f32().sin() * radius;
                let camz = start.elapsed().unwrap().as_secs_f32().cos() * radius;

                camera.set_position([camx, camy, camz]);
            },
        )
        .expect("Error during update loop");
}
