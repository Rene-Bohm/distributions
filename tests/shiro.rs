use distributions::pseudo::{Shiro, Splitmix64};


#[test]
fn shiro_f64() {
    let mut generator = Splitmix64 { seed: 668941 };

    let mut rand = Shiro::new(generator.call_256());

    let mut data: Vec<f64> = Vec::with_capacity(10);

    for _i in 0..10 {
        data.push(rand.call_f64());
    }

    println!("{:?}", data);
}
