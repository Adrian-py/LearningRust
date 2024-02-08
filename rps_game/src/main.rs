use rand::Rng;
use std::io;

#[derive(Debug, PartialEq)]
enum MoveOption {
    Rock,
    Paper,
    Scissors,
}

fn handle_player_move() -> MoveOption {
    let mut player_move: String = String::new();

    println!("Possible moves:");
    println!("1. Rock");
    println!("2. Paper");
    println!("3. Scissors");
    println!("Enter your move: ");
    io::stdin()
        .read_line(&mut player_move)
        .expect("Failed to read line");

    let player_move: String = player_move.trim().to_ascii_lowercase();

    match player_move.as_str() {
        "rock" => MoveOption::Rock,
        "paper" => MoveOption::Paper,
        "scissors" => MoveOption::Scissors,
        _ => {
            println!("Invalid move, try again");
            handle_player_move()
        }
    }
}

fn determine_winner(player_move: MoveOption, computer_move: MoveOption) {
    println!("\nResult:");
    println!("Player move: {:?}", player_move);
    println!("Computer move: {:?}", computer_move);

    if player_move == computer_move {
        println!("It's a tie!");
    } else if (player_move == MoveOption::Rock && computer_move == MoveOption::Scissors)
        || (player_move == MoveOption::Paper && computer_move == MoveOption::Rock)
        || (player_move == MoveOption::Scissors && computer_move == MoveOption::Paper)
    {
        println!("Player wins!");
    } else {
        println!("Computer wins!");
    }
}

fn main() {
    println!("Rock Paper Scissors");
    println!("=====================");
    let player_move: MoveOption = handle_player_move();

    let random_number: i32 = rand::thread_rng().gen_range(1..=3);
    let computer_move: MoveOption = match random_number {
        1 => MoveOption::Rock,
        2 => MoveOption::Paper,
        3 => MoveOption::Scissors,
        _ => MoveOption::Rock,
    };

    determine_winner(player_move, computer_move);
}
