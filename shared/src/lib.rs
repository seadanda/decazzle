use rand::distributions::{Distribution, Standard};
use rand::Rng;

/// Enum for the possible operations for a step
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

// type alias to make struct definitions more readable
type Number = i32;

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

/// Create a new puzzle with random seed and steps
impl Puzzle {
    pub fn new() -> Self {
        let seed = rand::random();
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
        let operand = rand::random();

        Step { operator, operand }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
