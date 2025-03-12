use clap::Parser;
use geonum_common::{BoundingBox as _, FromCSV as _};
use plotters::{element::DashedPathElement, prelude::*};

mod uniform_spline;

use uniform_spline::UniformBezierSpline;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The path of the CSV file to plot
    #[arg()]
    data_path: String,

    /// The output path of the plotted image
    #[arg(short, long)]
    output: String,

    /// Number of subdivision steps
    #[arg(short, long, default_value_t = 8)]
    steps: u16,

    /// Degree of the uniform spline
    #[arg(short, long, default_value_t = 3)]
    degree: u16,

    /// Wether to draw intermediate control polygons
    #[arg(short, long)]
    control: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let spline = UniformBezierSpline::from_csv(&args.data_path);
    let points = spline.clone().compute_two_point(args.steps, args.degree);
    let bb = points.bounding_box();

    let root = SVGBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sub-division curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x()..bb.1.x(), bb.0.y()..bb.1.y())?;

    chart.configure_mesh().draw()?;

    root.draw(&PathElement::new(
        points
            .iter()
            .map(|p| chart.backend_coord(&(p.x(), p.y())))
            .collect::<Vec<_>>(),
        &RED,
    ))?;

    if args.control {
        root.draw(&DashedPathElement::new(
            spline
                .control
                .iter()
                .map(|p| chart.backend_coord(&(p.x(), p.y()))),
            4,
            2,
            &BLACK,
        ))?;

        spline.control.iter().for_each(|point| {
            root.draw(&Circle::new(
                chart.backend_coord(&(point.x(), point.y())),
                3,
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
