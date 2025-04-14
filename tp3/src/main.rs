use clap::Parser;
use geonum_common::{BoundingBox, FromCSV, Point};
use plotters::{element::DashedPathElement, prelude::*};

mod bspline;

use bspline::BSpline;

const COLORS: [RGBColor; 6] = [RED, GREEN, BLUE, YELLOW, CYAN, MAGENTA];

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the BSpline or NURBS file to plot
    #[arg()]
    spline_path: String,

    /// The output path of the plotted image
    #[arg(short, long)]
    output: String,

    /// Number of datapoints to sample
    #[arg(short, long, default_value_t = 100)]
    samples: usize,

    /// Whether to draw intermediate control polygons
    #[arg(short, long)]
    draw_control: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let extension = std::path::Path::new(&args.spline_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .expect("Invalid file path");

    let (spline, control) = match extension {
        "bspline" => {
            let spline = BSpline::<2>::from_csv(&args.spline_path);
            (spline.evaluate(args.samples), spline.control)
        }
        "nurbs" => {
            let spline = BSpline::<3>::from_csv(&args.spline_path);
            let control_2d = spline
                .control
                .iter()
                .map(|c| Point::<2>::new(c.x(), c.y()))
                .collect::<Vec<_>>();
            let spline_2d = spline
                .evaluate(args.samples)
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|p| Point::<2>::new(p.x(), p.y()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            (spline_2d, control_2d)
        }
        _ => unimplemented!("Invalid file extention"),
    };

    let bb = control.bounding_box();

    let root = SVGBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Bezier curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x()..bb.1.x(), bb.0.y()..bb.1.y())?;

    chart.configure_mesh().draw()?;

    for (i, segment) in spline.into_iter().enumerate() {
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
            control.iter().map(|p| chart.backend_coord(&(p.x(), p.y()))),
            4,
            2,
            &BLACK,
        ))?;

        control.iter().enumerate().for_each(|(index, point)| {
            root.draw(&Circle::new(
                chart.backend_coord(&(point.x(), point.y())),
                if index == 0 || index == (control.len() - 1) {
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
