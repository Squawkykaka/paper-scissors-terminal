use rand::distr::{Distribution, StandardUniform};

#[derive(PartialEq, Debug)]
pub enum GameEnd {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Debug)]
pub enum Move {
    Scissors,
    Paper,
    Rock,
}

impl Distribution<Move> for StandardUniform {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Move {
        match rng.random_range(0..=2) {
            0 => Move::Paper,
            1 => Move::Scissors,
            _ => Move::Rock,
        }
    }
}

impl Move {
    pub fn compare_move(a: Move, b: Move) -> GameEnd {
        let winner: GameEnd = match (a, b) {
            (Move::Paper, Move::Scissors)
            | (Move::Scissors, Move::Rock)
            | (Move::Rock, Move::Paper) => GameEnd::Loss,
            (Move::Paper, Move::Rock)
            | (Move::Scissors, Move::Paper)
            | (Move::Rock, Move::Scissors) => GameEnd::Win,
            (Move::Paper, Move::Paper)
            | (Move::Scissors, Move::Scissors)
            | (Move::Rock, Move::Rock) => GameEnd::Draw,
        };

        winner
    }
}

pub fn parse_move(input: &str) -> Result<Move, &'static str> {
    match input {
        "p" | "P" => Ok(Move::Paper),
        "s" | "S" => Ok(Move::Scissors),
        "r" | "R" => Ok(Move::Rock),
        _ => Err("Invalid move"),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_move() {
        assert_eq!(parse_move("s"), Ok(Move::Scissors));
        assert_eq!(parse_move("S"), Ok(Move::Scissors));
        assert_eq!(parse_move("p"), Ok(Move::Paper));
        assert_eq!(parse_move("P"), Ok(Move::Paper));
        assert_eq!(parse_move("r"), Ok(Move::Rock));
        assert_eq!(parse_move("R"), Ok(Move::Rock));
    }

    #[test]
    fn test_parse_invalid_move() {
        assert!(parse_move("X").is_err());
        assert!(parse_move("").is_err());
    }
}
