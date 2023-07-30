use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Debug, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_generates_random_operations() {
        let random: Op = rand::random();
        assert!([Op::Add, Op::Sub, Op::Mul, Op::Div].contains(&random));
    }
}
