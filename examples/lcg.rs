use distributions::{helper::Bound, pseudo::*};
use plotters::prelude::*;

const OUT_FILE_NAME: &'static str = "img/bar.png";
fn main() {
    let g = distributions::pseudo::Lcg::instantiate(16807, 123456789, 10000);
    let u = g.call();

    println!("{:?}", u);

    let mut start = 0.0;
    let mut end = 0.05;
    let mut bounds: Vec<Bound> = Vec::with_capacity(20);

    for i in 0..20 {
        bounds.push(Bound::instantiate(start, end));
        start += 0.05;
        end += 0.05;
    }

    for i in 0..bounds.len() {
        for j in 0..u.len() {
            bounds[i].call(u[j]);
        }
    }

    let mut data = Vec::<i32>::new();

    for i in 0..bounds.len() {
        data.push(bounds[i].get() as i32);
    }

    let root_area = BitMapBackend::new("img/lcg.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Bar Demo", ("sans-serif", 40))
        .build_cartesian_2d(0..700, (0..20).into_segmented())
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series((0..).zip(data.iter()).map(|(y, x)| {
        let mut bar = Rectangle::new(
            [
                (0, SegmentValue::Exact(y)),
                (*x, SegmentValue::Exact(y + 1)),
            ],
            GREEN.filled(),
        );
        bar.set_margin(5, 5, 0, 0);
        bar
    }))
    .unwrap();
}
