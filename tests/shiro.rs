use distributions::{
    helper::*,
    pseudo::{Shiro, Splitmix64},
};

#[test]
fn lcg() {
    let g = distributions::pseudo::Lcg::instantiate(16807, 123456789, 100);
    let u = g.call();

    println!("{:?}", u);

    let mut start = 0.0;
    let mut end = 0.05;
    let mut bounds: Vec<Bound> = Vec::with_capacity(20);

    for _ in 0..20 {
        bounds.push(Bound::instantiate(start, end));
        start += 0.05;
        end += 0.05;
    }

    for i in 0..u.len() {
        for j in 0..bounds.len() {
            bounds[j].call(u[i]);
        }
    }

    for i in 0..bounds.len() {
        println!("{}", bounds[i].get());
    }
}

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
