use rand::Rng;
use std::{fmt, io};
use Guessable::*;

#[derive(Debug)]
enum Outcome {
    Tied,
    Won,
    Lost,
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Tied => "tied",
            Self::Won => "won",
            Self::Lost => "lost",
        };

        write!(f, "{}", text)
    }
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

impl fmt::Display for Guessable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Rock => "Rock",
            Paper => "Paper",
            Scissors => "Scissors",
        };

        write!(f, "{}", text)
    }
}

impl From<u8> for Guessable {
    fn from(value: u8) -> Self {
        match value % 3 {
            0 => Rock,
            1 => Paper,
            _ => Scissors,
        }
    }
}

impl TryFrom<&str> for Guessable {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "rock" => Ok(Rock),
            "paper" => Ok(Paper),
            "scissors" => Ok(Scissors),
            x => Err(x.to_string()),
        }
    }
}

fn main() {
    println!("Welcome to rock-paper-scissors-rs!");

    loop {
        println!("");
        println!("What do you pick? Rock, Paper or Scissors? (Press CTRL+C to exit)");

        let mut user_pick = String::new();
        io::stdin()
            .read_line(&mut user_pick)
            .expect("Failed to read line");

        let selected = match user_pick.trim().try_into() {
            Ok(x) => x,
            Err(e) => {
                println!("{} is not a valid pick, try again...", e);
                continue;
            }
        };

        let pc_pick: u8 = rand::thread_rng().gen();
        let pc_pick = pc_pick.into();

        println!("Your pick: {}, PC pick: {}", selected, pc_pick);

        let winner = Guessable::winner(selected, pc_pick);
        println!("You {}!", winner);
    }
}
