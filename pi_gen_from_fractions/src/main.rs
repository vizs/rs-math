const UPLIMIT: u64 = 1000;

fn calc_inf_series() -> f64 {
    let mut sum: f64 = 0.0;

    for i in 0..UPLIMIT {
        sum += 1.0/i.pow(2_u32) as f64;
    }

    sum
}

fn main() {
    println!(
             "{}",
             (6.0*calc_inf_series()).sqrt()
            );
}
