use std::time;
use std::io::stdin;
use std::num::ParseIntError;

mod binary_tree;
mod vojsic_tree;

fn main() {

    println!("Input number of nodes:");

    let node_count = match read_input() {
        Ok(node_count) => node_count,
        Err(why) => {
            println!("Error parsing input: {}", why);
            std::process::exit(1);
        }
    };

    println!("Unoptimized solution:");
    time_solution(node_count, binary_tree::solution, vojsic_tree::solution);

    println!("Optimized solution:");
    time_solution(node_count, binary_tree::solution_optimized, vojsic_tree::solution_optimized);
}

fn read_input() -> Result<usize, ParseIntError> {
    let mut buffer: String = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Error reading stdin");

    buffer.trim().parse()
}

fn time_solution(node_count: usize, binary_solution: fn(usize) -> usize, vojsic_solution: fn(usize) -> usize) {
    let start_time = time::SystemTime::now();

    println!("Pocet uzlov: {}, pocet binarnych stromov: {}, pocet vojsicovych stromov: {}", node_count, binary_solution(node_count), vojsic_solution(node_count));

    let end_time = time::SystemTime::now();
    let duration = end_time.duration_since(start_time).expect("Time error");

    println!("Solution took: {} ms\n", duration.as_millis());
}



