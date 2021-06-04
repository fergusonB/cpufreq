# cpufreq

Linux only.

cpufreq is a tool that lists the current cpu frequencies from /proc/cpuinfo. 

Resulting frequencies are sorted in descending order.

Written in rust.

## Usage

`./cpufreq` 

or

`./cpufreq 100`

If you move the binary to `/bin`, you can use `cpufreq`.

## Download
Binary available from the releases page on github.

## Building

### Requirements:
* [Rust](https://rustup.rs/)

### Commands
* Clone repo + enter cpufreq directory
* `cargo build --release`