use distributions::shift::csl_8;
use distributions::int::*;
#[test]
fn shiro(){

    let mut u = csl_8::instantiate(5); 

    println!("{:?}", &u);

    for i in 0..9{
    println!("{}", &u.shl());    
    }
    
}

#[test]
fn lcg(){

    let g = distributions::pseudo::Lcg::instantiate(16807, 123456789,100);
    let u = g.call();

    println!("{:?}", u);

    let mut start = 0.0;
    let mut end= 0.05;
    let mut bounds: Vec<bound> = Vec::with_capacity(20);

    for i in 0.. 20{

        bounds.push(bound::instantiate(start, end));
        start += 0.05;
        end += 0.05;

    }

    for i in 0..u.len(){
        for j in 0..bounds.len(){
            bounds[j].call(u[i]);
        }
    }

    for i in 0..bounds.len(){

        println!("{}", bounds[i].get());

    }

}