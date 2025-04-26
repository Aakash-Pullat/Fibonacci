use num_bigint::BigUint;
use num_traits::One;
use std::time::Instant;
fn fibonacci(){
    let start = Instant::now();
    let mut a: BigUint = BigUint::ZERO;
    let mut b: BigUint = BigUint::one();
    let mut i: u128 = 1;
    loop{
        i += 1;
        let temp = a+&b;
        a = b;
        b = temp;
        let runtime = start.elapsed().as_millis();
        if runtime > 1000 {
            println!("Fibonacci number {} is {}", i, b);
            break;
        }
    }
}
fn main(){
    fibonacci();
}