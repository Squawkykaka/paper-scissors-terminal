use std::io;

use crate::game::{GameEnd, Move, parse_move};

mod game;

fn main() {
    println!("This is paper(P), scissors(S), rock(R), choose a guess to continue.");

    let mut wins = 0;

    loop {
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
                continue;
            }
        };

        let game_end = Move::compare_move(player_move, enemy_move);

        match game_end {
            GameEnd::Win => {
                println!("You won!");
                wins += 1
            }
            GameEnd::Draw => println!("Its a draw"),
            GameEnd::Loss => println!("You lost!"),
        }

        println!("{wins}")
    }
}
