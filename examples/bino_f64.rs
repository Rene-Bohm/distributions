use distributions::distribution::*;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "img/bino_f64.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut uni = Binomial::<u64>::new(3123485, 100, 0.75);

    let data = uni.set(10000);

    let root = BitMapBackend::new(OUT_FILE_NAME, (640 * 3, 480 * 3)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Binomial Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u64..120u64).into_segmented(), 0i32..1250i32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .margin(3)
            .data(data.iter().map(|x: &u64| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
