pub mod chapter2;

#[allow(arithmetic_overflow)]
fn main() {
    // i8: -128 -> 127
    let x: i8 = 127 + 1;
    let a = x - 1;
    let b = x - 127;
    println!("{} + {} = {}",a,b, x);
}
