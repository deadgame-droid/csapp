#[test]
fn _22_a() {
    let mut left = i32::MAX;
    let mut n = 2;
    while left > n {
        left /= n;
        n += 1;
    }
    println!("\ni32 n! max n: {}", n - 1);
}

#[test]
fn _22_b() {
    let mut left = i64::MAX;
    let mut n = 2;
    while left > n {
        left /= n;
        n += 1;
    }
    println!("\ni32 n! max n: {}", n - 1);
}
