use std::time::Instant;

use shared::{solve_puzzle, Puzzle};
use text_io::read;

fn main() {
    let puzzle = Puzzle::new();
    let solution = solve_puzzle(&puzzle);
    println!("  Decazzle\nStart with the seed number and apply each operator to the result of the previous step.\nRemember to drop any remainders if the answer is not an integer.");
    println!("{}", puzzle);

    let start_time = Instant::now();
    let answer: usize = read!();
    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;

    if answer == solution {
        println!("correct");
    } else {
        println!("Incorrect.\nThe correct answer is: {}", solution);
    }
    println!("Time elapsed: {}s", elapsed_time.as_secs());
}
