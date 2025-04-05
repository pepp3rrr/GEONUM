use blue_engine::header::*;
use clap::{Parser, ValueEnum};
use geonum_common::{FromCSV as _, IntoMesh};
use triangle_mesh::{TriangleMesh, WeightMethod};

mod triangle_mesh;

const MOVE_SPEED: f32 = 5.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the OFF file to plot
    #[arg()]
    off_path: String,

    /// Number of subdivision steps (per axis)
    #[arg(short, long, default_value_t = 2)]
    steps: u16,

    /// Weight compute method
    #[arg(short, long, default_value_t, value_enum)]
    method: Method,

    /// Draw in wireframe mode
    #[arg(short, long)]
    wireframe: bool,
}

#[derive(ValueEnum, Default, Debug, Clone, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
enum Method {
    #[default]
    Beta,
    Warren,
}

fn main() {
    let args = Args::parse();

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let mut mesh = TriangleMesh::from_csv(args.off_path);
    mesh.weight_method = match args.method {
        Method::Beta => WeightMethod::Beta,
        Method::Warren => WeightMethod::Warren,
    };

    let mesh = mesh.subdivide(args.steps);

    let (vertices, indices) = mesh.into_mesh();
    engine.objects.new_object(
        "figure",
        vertices,
        indices,
        ObjectSettings {
            shader_settings: ShaderSettings {
                polygon_mode: if args.wireframe {
                    wgpu::PolygonMode::Line
                } else {
                    Default::default()
                },
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
