use core::str;
use std::fs;
use std::{thread,time};

fn main() {
    let one_second = time::Duration::from_secs(1);
    loop{
        let data = get_data();

        let results = search(&data);

        println!("{:?}",results);
        thread::sleep(one_second);
    }

}

fn get_data()->String{
    let data:String = fs::read_to_string("/proc/cpuinfo").expect("Something went wrong when reading the file.");
    data
}

fn search(data:&String)->Vec<&str>{
    let mut results = Vec::new();

    for line in data.lines(){
        if line.contains("cpu MHz"){
            results.push(line);
        }
    }
    results
}