use shared::{solve_puzzle, Puzzle};
use text_io::read;

fn main() {
    let puzzle = Puzzle::new();
    let solution = solve_puzzle(&puzzle);
    println!("{}", puzzle);
    let answer: usize = read!();

    if answer == solution {
        println!("correct");
    } else {
        println!("Incorrect.\nThe correct answer is: {}", solution);
    }
}
