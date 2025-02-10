use clap::Parser;
use geonum_common::{FromCSV as _, Plot as _};
use plotters::prelude::*;

mod spline;

use spline::BezierSpline;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the BCV file to plot
    #[arg()]
    bcv_path: String,

    /// The output path of the plotted image
    #[arg(short, long)]
    output: String,

    /// Number of datapoints to sample
    #[arg(short, long, default_value_t = 200)]
    samples: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let spline = BezierSpline::from_csv(args.bcv_path);
    let bb = spline.bounding_box();

    let root = BitMapBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x..bb.1.x, bb.0.y..bb.1.y)?;

    chart.configure_mesh().draw()?;

    spline
        .curves
        .iter()
        .enumerate()
        .for_each(|(index, bezier)| {
            let colors = [RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];

            chart
                .draw_series(LineSeries::new(
                    (0..=args.samples)
                        .map(|x| x as f32 / (args.samples as f32))
                        .map(|x| {
                            let result = bezier.sample(x);
                            (result.x, result.y)
                        }),
                    &colors.get(index % colors.len()).unwrap(),
                ))
                .expect("Failed to plot sub bezier");
        });

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
