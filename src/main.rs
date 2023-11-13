use std::io;
use rand::Rng;
use Guessable::*;

#[derive(Debug)]
enum Outcome {
    Tied,
    Won,
    Lost
}

#[derive(PartialEq, Debug)]
enum Guessable {
    Rock,
    Paper,
    Scissors,
}

impl Guessable {
    fn winner(a: Guessable, b: Guessable) -> Outcome {
        if a == b {
            Outcome::Tied
        } else if a.wins_against() == b {
            Outcome::Won
        } else {
            Outcome::Lost
        }
    }

    fn wins_against(&self) -> Guessable {
        match self {
            Guessable::Rock => Guessable::Scissors,
            Guessable::Scissors => Guessable::Paper,
            Guessable::Paper => Guessable::Rock,
        }
    }

}

fn main() {
    println!("Welcome to rock-paper-scissors-rs!");

    loop {
        println!("What do you pick? Rock, Paper or Scissors? (Press CTRL+C to exit)");
    
        let mut user_pick = String::new();

        let pc_pick = rand::thread_rng().gen_range(1..=3);
        let pc_pick = match pc_pick {
            1 => Rock,
            2 => Paper, 
            3 => Scissors,
            _ => panic!("This isn't possible")
        };
        
        io::stdin()
            .read_line(&mut user_pick)
            .expect("Failed to read line");

        let selected = match user_pick.to_lowercase().as_str().trim() {
            "rock" => Rock,
            "paper" => Paper,
            "scissors" => Scissors,
            x => {
                println!("{} is not a valid pick, try again...", x);
                continue;
            }
        };

        println!("Your pick: {:?}, PC pick: {:?}", selected, pc_pick);
        
        let winner = Guessable::winner(selected, pc_pick);
        println!("You {:?}!", winner);
    }
}
