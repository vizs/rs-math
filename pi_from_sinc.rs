use std::f64;

const LOWER_LIMIT: i64 = -1000000000000;
const UPPER_LIMIT: i64 = -LOWER_LIMIT;

fn main() {
    let sinc = |x: f64| {
        if x == 0.0 {
            1.0
        } else {
            x.sin() / x
        }
    };

    let mut pi = 0.0;

    for i in LOWER_LIMIT..=UPPER_LIMIT {
        pi += sinc(i as f64);
    }
    println!("{}", pi);
    println!("{}", f64::consts::PI - pi);
}
