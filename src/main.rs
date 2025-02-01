use plotters::prelude::*;

mod bezier;
mod types;

use bezier::Bezier;
use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bezier = Bezier::from_csv("data/simple.bcv");
    let bb = bezier.bounding_box();

    let root = BitMapBackend::new("/tmp/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x..bb.1.x, bb.0.y..bb.1.y)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-50..=50).map(|x| x as f32 / 50.0).map(|x| {
            let result = bezier.compute(x);
            (result.x, result.y)
        }),
        &RED,
    ))?;

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
