use cpufreq;
use std::thread;

fn main() {
    let interval = cpufreq::get_interval();

    loop {
        let data = cpufreq::get_data();

        let results = cpufreq::search(&data);

        let formatted_results = cpufreq::format(results);

        println!("{:?}", formatted_results);

        thread::sleep(interval);
    }
}
