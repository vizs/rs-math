fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let uplimit: u64 = argv[1].trim()
                              .parse()
                              .unwrap();

    let mut den: f64 = 0.0;
    for n in 1..=uplimit {
        let d = (2*n) * (2*n + 1) * (2*n + 2);

        if n % 2 == 0 {
            den -= 4.0 / (d as f64);
        } else {
            den += 4.0 / (d as f64);
        }
    }

    println!("{}", 3.0 + den);
}
