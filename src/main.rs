use cpufreq;
use std::{thread, time};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let mut interval = time::Duration::from_millis(1000);

    match args.len(){
        2=>{
            interval = time::Duration::from_millis(args[1].parse().unwrap());
        },
        _=>{
            eprintln!("Error, usage: 'cpufreq 500'. Defaulting to 1000ms refresh.");
            thread::sleep(interval)
        }
    }
   
    
    loop {
        let data = cpufreq::get_data();

        let results = cpufreq::search(&data);

        let formatted_results = cpufreq::format(results);

        println!("{:?}", formatted_results);

        thread::sleep(interval);
    }
}
