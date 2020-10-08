mod algorithms;
mod generate;
mod measure;

use algorithms::single_pattern::naive::naive_all;
use generate::gen_rand_bytes;
use measure::{calculate_avg_duration, measure_multiple};

fn main() {
    let text = gen_rand_bytes(1_000_000);
    let pattern = &text[20..25];

    let durations = measure_multiple(pattern, &text, naive_all, 10);

    println!("Duration: {:?}", durations);
    println!("Average: {:?}", calculate_avg_duration(durations));
}
