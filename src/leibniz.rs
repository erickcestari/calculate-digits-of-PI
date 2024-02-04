use rayon::prelude::*;

pub fn leibniz_pi() {
    let iterations: u64 = 1000000000;

    // Set the maximum number of threads to 4 (you can adjust this number) 
    // !Warning: It can overheat your CPU use a low value on num_threads
    rayon::ThreadPoolBuilder::new().num_threads(4).build_global().unwrap();

    let pi_estimate = (0..iterations)
        .into_par_iter()
        .map(|i| {
            let denominator = (2 * i + 1) as f64;
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            sign / denominator
        })
        .sum::<f64>() * 4.0;

    println!(
        "PI approximation after {} num: {}",
        iterations,
        pi_estimate
    );
}
