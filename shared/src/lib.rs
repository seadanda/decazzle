use core::fmt::Display;

use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

const STEPS_PER_PUZZLE: usize = 2;

/// Enum for the possible operations for a step
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// type alias to make struct definitions more readable
type Number = usize;

/// One step in the puzzle containing an operator and the second operand
#[derive(Debug, PartialEq, Clone, Copy)]
struct Step {
    operator: Op,
    operand: Number,
}

/// The puzzle containing the seed number (initial operand) and the ten operations
#[derive(Debug, PartialEq)]
pub struct Puzzle {
    seed: Number,
    steps: [Step; STEPS_PER_PUZZLE],
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut steps_formatted = String::from("");
        for step in self.steps {
            steps_formatted += &format!(" {} {}", step.operator, step.operand);
        }
        write!(f, "{}{}", self.seed, steps_formatted)
    }
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {} {}", self.operator, self.operand)
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = match self {
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/",
        };
        write!(f, "{}", operator)
    }
}

/// Create a new puzzle with random seed and steps
impl Puzzle {
    pub fn new() -> Self {
        let seed = thread_rng().gen_range(1..100);
        let steps: [Step; STEPS_PER_PUZZLE] = rand::random();

        Self { seed, steps }
    }
}

/// Create a default step which adds one
impl Default for Step {
    fn default() -> Self {
        let operator = Op::Add;
        let operand = 1;

        Self { operator, operand }
    }
}

/// Create default puzzle with a seed of 10 and adds one each time
impl Default for Puzzle {
    fn default() -> Self {
        let seed = 10;
        let step = Step::default();
        let steps: [Step; STEPS_PER_PUZZLE] = [step; STEPS_PER_PUZZLE];

        Self { seed, steps }
    }
}

/// Allow rand::random to randomly choose an operator
impl Distribution<Op> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Op {
        match rng.gen_range(0..4) {
            0 => Op::Add,
            1 => Op::Sub,
            2 => Op::Mul,
            _ => Op::Div,
        }
    }
}

/// Allow rand::random to randomly generate a step
impl Distribution<Step> for Standard {
    fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> Step {
        let operator = rand::random();
        let range = match operator {
            Op::Add => 1..100,
            Op::Sub => 1..100,
            Op::Mul => 1..10,
            Op::Div => 1..10,
        };

        let operand = thread_rng().gen_range(range);

        Step { operator, operand }
    }
}

pub fn solve_puzzle(puzzle: &Puzzle) -> usize {
    let mut solution = puzzle.seed;
    for step in puzzle.steps {
        solution = match step.operator {
            Op::Add => solution + step.operand,
            Op::Sub => solution - step.operand,
            Op::Mul => solution * step.operand,
            Op::Div => solution / step.operand,
        }
    }
    solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_solves_puzzles() {
        let puzzle = Puzzle::default();
        let solved_puzzle = 10 + STEPS_PER_PUZZLE;

        assert_eq!(solve_puzzle(&puzzle), solved_puzzle);
    }

    #[test]
    fn it_prints_puzzles() {
        let puzzle = Puzzle::default();
        let steps = " + 1".repeat(STEPS_PER_PUZZLE);
        let puzzle_output = format!("10{}", steps);

        assert_eq!(format!("{}", puzzle), puzzle_output);
    }

    #[test]
    fn it_generates_puzzles() {
        let seed = 10;
        let step = Step {
            operator: Op::Add,
            operand: 1,
        };
        let steps: [Step; STEPS_PER_PUZZLE] = [step; STEPS_PER_PUZZLE];

        let puzzle = Puzzle { seed, steps };
        assert_eq!(Puzzle::default(), puzzle);
    }

    #[test]
    fn it_generates_random_operations() {
        let random: Op = rand::random();
        assert!([Op::Add, Op::Sub, Op::Mul, Op::Div].contains(&random));
    }
}
