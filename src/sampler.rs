use std::{thread, time};

fn main() {
    let start = time::Instant::now();
    let sleep_time = time::Duration::from_secs(5);
    println!("Sampling by sleeping thread every {:?}...", sleep_time);
    loop {
        let start_sample = start.elapsed();
        thread::sleep(sleep_time);
        let end_sample = start.elapsed();
        let sample = end_sample - start_sample;
        if sample.as_secs() > (sleep_time.as_secs() as f64 * 1.5) as u64 {
            println!("System appears to have slept! Expected ~{sleep_time:?}, calculated {sample:?}.", sleep_time=sleep_time, sample=sample);
            break;
        }
    }
}