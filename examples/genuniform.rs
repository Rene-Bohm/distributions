
fn main(){

    let g = distributions::pseudo::Lcg::instantiate(16807, 123456789,10000);
    let u = g.call();

    

}