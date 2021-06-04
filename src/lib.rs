use core::str;
use std::fs;

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

pub fn format(vec:Vec<&str>)->Vec<&str>{
    let mut results = Vec::new();
    for thread in vec{
        let result = thread.split(" ").last().unwrap();
        results.push(result);
    }
    results
}