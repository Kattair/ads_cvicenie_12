use std::io::stdin;
use std::time;

mod binary_tree;
mod vojsic_tree;

fn main() {
    print_menu();
    let mode = read_input();

    println!("Input number of nodes:");
    let node_count = read_input();

    if mode == 1 {
        time_comparison(node_count);
        return;
    }

    if mode == 2 || mode == 4 {
        println!("Unoptimized solution:");
        time_solution(node_count, binary_tree::solution, vojsic_tree::solution);
    }

    if mode == 3 || mode == 4 {
        println!("Optimized solution:");
        time_solution(node_count, binary_tree::solution_optimized, vojsic_tree::solution_optimized);
    }
}

fn print_menu() {
    println!("Select mode:");
    println!("1. Comparison");
    println!("2. Unoptimized solution");
    println!("3. Optimized solution");
    println!("4. Both solutions");
}

fn read_input() -> usize {
    let mut buffer: String = String::new();

    stdin()
        .read_line(&mut buffer)
        .expect("Error reading stdin");

    match buffer.trim().parse() {
        Ok(num) => num,
        Err(why) => {
            println!("Error parsing input: {}", why);
            std::process::exit(1);
        }
    }
}

fn time_solution(node_count: usize, binary_solution: fn(usize) -> usize, vojsic_solution: fn(usize) -> usize) {
    let start_time = time::SystemTime::now();

    println!("Pocet uzlov: {}, pocet binarnych stromov: {}, pocet vojsicovych stromov: {}", node_count, binary_solution(node_count), vojsic_solution(node_count));

    let end_time = time::SystemTime::now();
    let duration = end_time.duration_since(start_time).expect("Time error");

    println!("Solution took: {} ms\n", duration.as_millis());
}

fn time_comparison(node_count: usize) {
    let start_time = time::SystemTime::now();

    for i in 1..(node_count + 1) {
        println!("Pocet uzlov: {}, pocet binarnych stromov: {}, pocet vojsicovych stromov: {}",
                 node_count, binary_tree::solution_optimized(i), vojsic_tree::solution_optimized(i));
    }

    let end_time = time::SystemTime::now();
    let duration = end_time.duration_since(start_time).expect("Time error");

    println!("Solution took: {} ms\n", duration.as_millis());
}



