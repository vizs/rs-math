extern crate rand;

use rand::{thread_rng, Rng};

const RANDOM_UPLIMIT: u64 = 1000000;
const RANDOM_DNLIMIT: u64 = 1;
const PROB_LIMIT: u64 = 1000000;

fn gcd(a: u64, b: u64) -> u64 {
    match b {
        0 => return a,
        _ => gcd(b, a % b),
    }
}

fn find_prob() -> f64 {
    // number of times, two random nums
    // were coprime
    let mut num_coprime: f64 = 0.0;

    for _ in 0..PROB_LIMIT {
        match gcd(
                  thread_rng().gen_range(RANDOM_DNLIMIT, RANDOM_UPLIMIT),
                  thread_rng().gen_range(RANDOM_DNLIMIT, RANDOM_UPLIMIT)
                 ) {
            1 => num_coprime += 1.0,
            _ => (),
        }
    }

    num_coprime / PROB_LIMIT as f64
}

fn calc_pi(rand_prob: f64) -> f64 {
    (6.0/rand_prob).sqrt()
}

fn main() {
    for _ in 0..10 {
        println!("{}", calc_pi(find_prob()));
    }
}
