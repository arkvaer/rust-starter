fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
    let num = 5u64;
    println!("{}",num);
    test(x);
}

fn test(par: (i32, f64, u8) ) {
    println!("{:?}", par)
}