use blue_engine::{
    header::{Engine, WindowDescriptor},
    wgpu, KeyCode, ObjectSettings, ShaderSettings,
};
use clap::Parser;
use geonum_common::{FromCSV as _, IntoMesh as _};
use subdivision_surface::SubdivisionSurface;

mod subdivision_surface;

const MOVE_SPEED: f32 = 10.0;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the NET file to plot
    #[arg()]
    net_path: String,

    /// Number of subdivision steps (per axis)
    #[arg(short, long, default_value_t = 2)]
    steps: u16,

    /// Draw in wireframe mode
    #[arg(short, long)]
    wireframe: bool,

    /// Wether to draw intermediate control polygons
    #[arg(short, long)]
    draw_control: bool,
}

fn main() {
    let args = Args::parse();

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    let surface = SubdivisionSurface::from_csv(args.net_path);
    let subdivided_surface = surface.compute(args.steps);

    let (vertices, indices) = subdivided_surface.into_mesh();
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

    if args.draw_control {
        let (vertices, indices) = surface.control.into_mesh();
        engine.objects.new_object(
            "control",
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
        engine
            .objects
            .get_mut("control")
            .unwrap()
            .set_color(1.0, 0.0, 0.0, 0.6);
    }

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
