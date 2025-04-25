use text_io::read;
fn main(){
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    let mut temp: u128;
    print!("Enter n:");
    let n: u128 = read!("{}");
    for i in 0..n+1 {
        temp = a+b;
        a = b;
        b = temp;
        println!("i:{} i!:{} ",i, b);
    }
}