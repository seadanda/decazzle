use core::fmt::Display;

use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};

/// Enum for the possible operations for a step
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// type alias to make struct definitions more readable
type Number = u32;

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
    steps: [Step; 10],
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
        let seed = thread_rng().gen_range(0..100);
        let steps: [Step; 10] = rand::random();

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
        let steps: [Step; 10] = [step; 10];

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
            Op::Add => 0..100,
            Op::Sub => 0..100,
            Op::Mul => 0..10,
            Op::Div => 0..10,
        };

        let operand = thread_rng().gen_range(range);

        Step { operator, operand }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_prints_puzzles() {
        let puzzle = Puzzle::default();
        let puzzle_output = String::from("10 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1");

        assert_eq!(format!("{}", puzzle), puzzle_output);
    }

    #[test]
    fn it_generates_puzzles() {
        let seed = 10;
        let step = Step {
            operator: Op::Add,
            operand: 1,
        };
        let steps: [Step; 10] = [step; 10];

        let puzzle = Puzzle { seed, steps };
        assert_eq!(Puzzle::default(), puzzle);
    }

    #[test]
    fn it_generates_random_operations() {
        let random: Op = rand::random();
        assert!([Op::Add, Op::Sub, Op::Mul, Op::Div].contains(&random));
    }
}
