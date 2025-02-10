use clap::Parser;
use geonum_common::{FromCSV as _, Plot as _};
use plotters::{element::DashedPathElement, prelude::*};

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

    /// Wether to draw intermediate control polygons
    #[arg(short, long)]
    draw_control: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let spline = BezierSpline::from_csv(args.bcv_path);
    let bb = spline.bounding_box();

    let root = SVGBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier spline", ("sans-serif", 50).into_font())
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
                .unwrap();
        });

    if args.draw_control {
        spline.curves.iter().for_each(|bezier| {
            root.draw(&DashedPathElement::new(
                bezier
                    .control
                    .iter()
                    .map(|p| chart.backend_coord(&(p.x, p.y))),
                4,
                2,
                &full_palette::GREY_A700,
            ))
            .unwrap();

            bezier
                .control
                .iter()
                .enumerate()
                .for_each(|(index, point)| {
                    root.draw(&Circle::new(
                        chart.backend_coord(&(point.x, point.y)),
                        if index == 0 || index == (bezier.control.len() - 1) {
                            5
                        } else {
                            3
                        },
                        Into::<ShapeStyle>::into(&BLACK).filled(),
                    ))
                    .unwrap();
                });
        });
    }

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
