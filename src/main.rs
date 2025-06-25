use core::net;
use std::{
    io,
    process::{Command, Stdio},
};

use clap::{Parser, command};

use crate::game::{GameEnd, Move, parse_move};

mod game;

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "This is paper(P), scissors(S), rock(R), choose a guess to continue."
)]
struct Args {
    #[arg(short, long)]
    command: String,
}

fn main() {
    let args = Args::parse();

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let enemy_move = match rand::random_range(0..=2) {
        0 => Move::Paper,
        1 => Move::Scissors,
        _ => Move::Rock,
    };
    let player_move = match parse_move(guess.trim_end()) {
        Ok(current_move) => current_move,
        Err(current_move) => {
            println!("{current_move} is not a valid move. Choose another");
            return;
        }
    };

    let game_end = Move::compare_move(player_move, enemy_move);

    match game_end {
        GameEnd::Win => {
            println!("You won!");
            run_command(args.command)
        }
        GameEnd::Draw => println!("Its a draw"),
        GameEnd::Loss => println!("You lost!"),
    }
}

fn run_command(string_command: String) {
    let command_args: Vec<&str> = string_command.split(" ").collect();

    let mut binding = Command::new(command_args.first().unwrap());
    let new_command = binding.args(&command_args[1..]);

    new_command
        .stdout(Stdio::inherit())
        .spawn()
        .expect("Command failed to run")
        .wait()
        .expect("Zombie process spawned");
}
