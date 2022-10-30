use distributions::distribution::{Exponential, Set};
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "img/exp_f64.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut uni = Exponential::<f64>::new(1337, 0.75, 0.0, 1.0);

    let data = uni.set(10000);

    let mut count = 0;

    for i in 0..data.len() {
        if data[i] < 1.0 {
            count += 1;
        }
    }

    println!("--{}", count);

    let mut avg = 0.0;
    for i in 0..10000{

        avg += data[i] as f64;

    }

    println!("--{}", avg/10000.0);


    let root = BitMapBackend::new(OUT_FILE_NAME, (640 * 3, 480 * 3)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d(
            (0.0f64..10.1f64).step(0.2).use_floor().into_segmented(),
            0i32..1250i32,
        )?;

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
            .data(data.iter().map(|x: &f64| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())
}
