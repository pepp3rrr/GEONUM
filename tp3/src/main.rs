use clap::Parser;
use geonum_common::{BoundingBox, FromCSV};
use plotters::{element::DashedPathElement, prelude::*};

mod bspline;

use bspline::BSpline;

const COLORS: [RGBColor; 6] = [RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the BSpline file to plot
    #[arg()]
    bspline_path: String,

    /// The output path of the plotted image
    #[arg(short, long)]
    output: String,

    /// Number of datapoints to sample
    #[arg(short, long, default_value_t = 100)]
    samples: usize,

    /// Wether to draw intermediate control polygons
    #[arg(short, long)]
    draw_control: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let spline = BSpline::from_csv(&args.bspline_path);
    let result = spline.evaluate(args.samples);
    let bb = spline.control.bounding_box();

    let root = SVGBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x()..bb.1.x(), bb.0.y()..bb.1.y())?;

    chart.configure_mesh().draw()?;

    for (i, segment) in result.into_iter().enumerate() {
        chart.draw_series(LineSeries::new(
            segment
                .into_iter()
                .map(|p| (p.x(), p.y()))
                .collect::<Vec<_>>(),
            COLORS[i % COLORS.len()],
        ))?;
    }

    if args.draw_control {
        root.draw(&DashedPathElement::new(
            spline
                .control
                .iter()
                .map(|p| chart.backend_coord(&(p.x(), p.y()))),
            4,
            2,
            &BLACK,
        ))?;

        spline
            .control
            .iter()
            .enumerate()
            .for_each(|(index, point)| {
                root.draw(&Circle::new(
                    chart.backend_coord(&(point.x(), point.y())),
                    if index == 0 || index == (spline.control.len() - 1) {
                        5
                    } else {
                        3
                    },
                    Into::<ShapeStyle>::into(&BLACK).filled(),
                ))
                .unwrap();
            });
    }

    chart
        .configure_series_labels()
        .legend_area_size(0)
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}
