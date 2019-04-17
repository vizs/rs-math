const LIMIT: u64 = 100;

fn main() {
    let mut sum: f64 = 0.0;

    for i in 0..LIMIT {
        sum += 0.5_f64.powi(i as i32);
        println!("{} {}", i+1, sum);
    }
}
