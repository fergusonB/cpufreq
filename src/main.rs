use cpufreq;
use std::{thread, time};

fn main() {
    let one_second = time::Duration::from_secs(1);
    loop {
        let data = cpufreq::get_data();

        let results = cpufreq::search(&data);

        println!("{:?}", results);
        thread::sleep(one_second);
    }
}
