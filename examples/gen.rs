use distributions::pseudo::*;
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "img/bar.png";
fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut buffer:[u64; 4] = [0,0,0,0]; 
    let mut gen = splitmix64_state{ seed: 668941 };

    for i in 0..4{

        buffer[i] = gen.call();

    }

    println!("{:?}", buffer);

    let mut rand = State::new(buffer);

    let mut data: Vec<u32> = Vec::with_capacity(1000); 

    for _i in 0..1000{
        data.push((rand.call()%10) as u32);
    }

    println!("{:?}",&data);

    let root = BitMapBackend::new(OUT_FILE_NAME, (640, 480)).into_drawing_area();

    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Histogram Test", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..10u32).into_segmented(), 0u32..150u32)?;

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
            .data(data.iter().map(|x: &u32| (*x, 1))),
    )?;

    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", OUT_FILE_NAME);

    Ok(())

}