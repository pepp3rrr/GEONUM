use bezier_surface::BezierSurface;
use blue_engine::{
    header::{Engine, WindowDescriptor},
    primitive_shapes::triangle,
    KeyCode, ObjectSettings, Vector3,
};
use geonum_common::FromCSV as _;

mod bezier_surface;

const MOVE_SPEED: f32 = 10f32;

fn main() {
    let surface = BezierSurface::from_csv("tp6/data/teapot.bpt");
    println!("{:?}", surface.control);

    let mut engine =
        Engine::new_config(WindowDescriptor::default()).expect("Couldn't init the Engine");

    triangle(
        "Triangle",
        ObjectSettings::default(),
        &mut engine.renderer,
        &mut engine.objects,
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
