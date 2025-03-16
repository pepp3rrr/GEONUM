use bezier_surface::PiecewiseBezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    wgpu, ObjectSettings, ShaderSettings,
};
use clap::Parser;
use geonum_common::FromCSV as _;
use render::IntoMesh;

mod bezier_surface;
mod render;

const COLORS: [(f32, f32, f32, f32); 6] = [
    (1.0, 0.0, 0.0, 1.0),
    (0.0, 1.0, 0.0, 1.0),
    (1.0, 1.0, 0.0, 1.0),
    (0.0, 0.0, 1.0, 1.0),
    (1.0, 0.0, 1.0, 1.0),
    (0.0, 1.0, 1.0, 1.0),
];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the BCV file to plot
    #[arg()]
    bpt_path: String,

    /// Number of datapoints to sample (per axis)
    #[arg(short, long, default_value_t = 8)]
    samples: u16,
}

fn main() {
    let args = Args::parse();

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let surface = PiecewiseBezierSurface::from_csv(args.bpt_path);

    for (n, patch) in surface.patches.into_iter().enumerate() {
        let mut samples = Vec::new();
        for i in 0..(args.samples + 1) {
            let mut row = Vec::new();
            for j in 0..(args.samples + 1) {
                let (u, v) = (
                    i as f32 / args.samples as f32,
                    j as f32 / args.samples as f32,
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
