use core::str;
use std::fs;
use std::env;
use std::time::Duration;
use std::{thread, time};


pub fn get_interval()->Duration{
    let args: Vec<String> = env::args().collect();

    let mut interval = time::Duration::from_millis(1000);

    match args.len() {
        2 => {
            interval = time::Duration::from_millis(args[1].parse().unwrap());
        }
        _ => {
            eprintln!("Error, usage: 'cpufreq 500'. Defaulting to 1000ms refresh.");
            thread::sleep(interval)
        }
    }
    interval
}


pub fn get_data() -> String {
    let data: String =
        fs::read_to_string("/proc/cpuinfo").expect("Something went wrong when reading the file.");
    data
}

pub fn search(data: &String) -> Vec<&str> {
    let mut results = Vec::new();

    for line in data.lines() {
        if line.contains("cpu MHz") {
            results.push(line);
        }
    }
    results
}

pub fn format(vec: Vec<&str>) -> Vec<f32> {
    let mut results: Vec<f32> = Vec::new();
    for thread in vec {
        let result = thread.split(" ").last().unwrap();
        results.push(result.parse::<f32>().unwrap());
    }
    results.sort_by(|a, b| b.partial_cmp(a).unwrap());
    results
}
