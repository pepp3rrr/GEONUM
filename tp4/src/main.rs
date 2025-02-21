use clap::{Parser, ValueEnum};
use geonum_common::{BoundingBox as _, FromCSV};
use plotters::{element::DashedPathElement, prelude::*};
use subdivision::{ComputeMethod, SubdivisionCurve};

mod subdivision;

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

    /// Wether to draw intermediate control polygons
    #[arg(short, long)]
    draw_control: bool,

    /// Method for computing the subdivision curve
    #[arg(short, long, default_value_t, value_enum)]
    method: Method,

    /// Alpha parameter for corner-cutting method
    #[arg(short, long, default_value_t = 0.1)]
    alpha: f32,

    /// Beta parameter for corner-cutting method
    #[arg(short, long, default_value_t = 0.6)]
    beta: f32,
}

#[derive(ValueEnum, Default, Debug, Clone, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
enum Method {
    #[default]
    Chaikin,
    CornerCutting,
    FourPoint,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let method = match args.method {
        Method::Chaikin => ComputeMethod::Chaikin,
        Method::CornerCutting => ComputeMethod::CornerCutting {
            a: args.alpha,
            b: args.beta,
        },
        Method::FourPoint => ComputeMethod::FourPoint,
    };
    let subdivision = SubdivisionCurve::from_csv(&args.data_path);
    let points = subdivision.clone().compute(method, args.steps);
    let bb = points.bounding_box();

    let root = SVGBackend::new(&args.output, (1080, 720)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Sub-division curve", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(bb.0.x..bb.1.x, bb.0.y..bb.1.y)?;

    chart.configure_mesh().draw()?;

    root.draw(&PathElement::new(
        points
            .iter()
            .map(|p| chart.backend_coord(&(p.x, p.y)))
            .collect::<Vec<_>>(),
        &RED,
    ))?;

    if args.draw_control {
        root.draw(&DashedPathElement::new(
            subdivision
                .control
                .iter()
                .map(|p| chart.backend_coord(&(p.x, p.y))),
            4,
            2,
            &BLACK,
        ))?;

        subdivision
            .control
            .iter()
            .enumerate()
            .for_each(|(index, point)| {
                root.draw(&Circle::new(
                    chart.backend_coord(&(point.x, point.y)),
                    if index == 0 || index == (subdivision.control.len() - 1) {
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
