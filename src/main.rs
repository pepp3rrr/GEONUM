use plotters::prelude::*;

mod bezier;
mod types;

use bezier::Bezier;
use types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let control = vec![Point::new(0., 0.), Point::new(1., 2.), Point::new(2., 0.)];
    let control_str = format!(
        "Control: {:?}",
        control.iter().map(|p| (p.x, p.y)).collect::<Vec<_>>()
    );
    let bezier = Bezier::new(control);

    let root = BitMapBackend::new("/tmp/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..2f32, 0f32..1f32)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(
            (-50..=50).map(|x| x as f32 / 50.0).map(|x| {
                let result = bezier.compute(x);
                (result.x, result.y)
            }),
            &RED,
        ))?
        .label(control_str)
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
