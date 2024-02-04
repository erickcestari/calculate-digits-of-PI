
use rand::Rng;

pub fn monte_carlo(){
  let r: f64 = 100.0;

    let mut rng = rand::thread_rng();
    let mut points: usize = 0;
    let mut circle_points: usize = 0;

    loop {
        let x: f64 = rng.gen_range(-r..=r);
        let y: f64 = rng.gen_range(-r..=r);

        let d = x.powi(2) + y.powi(2);
        points += 1;

        if r.powi(2) > d {
            circle_points += 1;
        }

        if points % 10000000 == 0 {
            let pi = 4.0 * (circle_points as f64) / (points as f64);

            println!("PI approximation after {} points: {}", points, pi);
        }
    }
}

pub fn euclid() {
    let size = 10000;
    let mut rng = rand::thread_rng();
    
    let mut total: u64 = 0;
    let mut co_prime: u64 = 0;

    loop {
        let n1: u64 = rng.gen_range(1..=size);
        let n2: u64 = rng.gen_range(1..=size);

        let d = gcd(n1, n2);

        if d == 1 {
            co_prime += 1;
        }
        total += 1;

        if total % 10000000 == 0 {
            let pi_estimate = estimate_pi(co_prime, total);

            println!("PI approximation after {} num: {}", total, pi_estimate);
        }
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn estimate_pi(co_prime: u64, total: u64) -> f64 {
    (6.0 * total as f64 / co_prime as f64).sqrt()
}
