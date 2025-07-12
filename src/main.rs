use std::{
    io::{self, Write},
    process::{Command, Stdio},
};

use clap::{Parser, command};
use rand::{Rng, distr::StandardUniform};

use crate::game::{GameEnd, Move, parse_move};

mod game;

#[derive(Parser, Debug)]
#[command(
    author = "Squawkykaka",
    version = "1.0",
    about = "
Play paper scissors rock to be able to use the terminal.
Choose [P, S, R] to guess.
    "
)]
struct Args {
    #[arg(required = true)]
    command: Vec<String>,
}

fn main() -> color_eyre::eyre::Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let mut guess = String::new();

    print!("Please enter your move: ");
    io::stdout().flush()?;

    io::stdin().read_line(&mut guess)?;

    let enemy_move: Move = rand::rng().sample(StandardUniform);

    let player_move = match parse_move(guess.trim_end()) {
        Ok(current_move) => current_move,
        Err(_) => {
            println!(
                "{} is not a valid move, run with --help to get help.",
                guess.trim_end()
            );
            std::process::exit(1);
        }
    };

    let game_end = Move::compare_move(player_move, enemy_move);

    match game_end {
        GameEnd::Win => {
            println!("You won!");
            run_command(args.command);
            std::process::exit(0);
        }
        GameEnd::Draw => {
            println!("Its a draw");
            std::process::exit(0);
        }
        GameEnd::Loss => {
            println!("You lost!");
            std::process::exit(0);
        }
    }
}

fn run_command(string_command: Vec<String>) {
    let mut binding = Command::new(string_command.first().unwrap());
    let new_command = binding.args(&string_command[1..]);

    new_command
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Command failed to run")
        .wait()
        .expect("Zombie process spawned");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compare_moves() {
        assert_eq!(Move::compare_move(Move::Paper, Move::Rock), GameEnd::Win);
        assert_eq!(Move::compare_move(Move::Rock, Move::Scissors), GameEnd::Win);
        assert_eq!(
            Move::compare_move(Move::Scissors, Move::Paper),
            GameEnd::Win
        );

        assert_eq!(Move::compare_move(Move::Paper, Move::Paper), GameEnd::Draw);
        assert_eq!(Move::compare_move(Move::Rock, Move::Rock), GameEnd::Draw);
        assert_eq!(
            Move::compare_move(Move::Scissors, Move::Scissors),
            GameEnd::Draw
        );

        assert_eq!(Move::compare_move(Move::Rock, Move::Paper), GameEnd::Loss);
        assert_eq!(
            Move::compare_move(Move::Scissors, Move::Rock),
            GameEnd::Loss
        );
        assert_eq!(
            Move::compare_move(Move::Paper, Move::Scissors),
            GameEnd::Loss
        );
    }

    #[test]
    fn test_run_command() {
        // Test a simple command like `echo`
        let command = vec!["echo".to_string(), "Hello, World!".to_string()];
        run_command(command);
        // Note: This test assumes the command runs successfully. You may need to mock `Command` for more robust testing.
    }
}
