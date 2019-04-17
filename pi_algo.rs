fn fac(i: &i64) -> i64 {
    let mut factorial: i64 = 1;

    for n in 1..=*i {
        factorial *= n;
    }

    factorial
}

fn chudnovsky(k: i64) -> f64 {
    let num: f64 = 426880.0 * 10005.0_f64.sqrt();
    let mut den: f64 = 0.0;

    for i in 0..=k {
        let m = fac(&(6*i)) / (fac(&(3*i)) * fac(&i).pow(3_u32));
        let l = 545140134 * i + 13591409;
        let x = (-262537412640768000_i64).pow(i as u32);

        den += (m*l) as f64 / x as f64;
    }

    num / den
}

fn ramanujan_sato(k: i64) -> f64 {
    let num: f64 = (99.0 * 99.0) / (8.0_f64.sqrt());
    let mut den: f64 = 0.0;
    let a: f64 = 26390.0;
    let b: f64 = 1103.0;
    let c: f64 = 396.0_f64.powi(4_i32);

    let s = |k| {
        (fac(&(4*k)) / fac(&k).pow(4_u32)) as f64
    };

    for i in 0..=k {
        den += s(i) * ((a*(i as f64) + b) / c.powi(i as i32));
    }

    num / den
}

fn main() {
    for i in 0..=10 {
        println!("{}", ramanujan_sato(i));
    }
}
