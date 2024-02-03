use rand::Rng;

fn main() {
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

