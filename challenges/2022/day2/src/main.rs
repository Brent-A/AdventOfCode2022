enum GameChoice {
    Rock,
    Paper,
    Scissors,
}

impl GameChoice {
    pub fn value(&self) -> u32 {
        match self {
            GameChoice::Rock => 1,
            GameChoice::Paper => 2,
            GameChoice::Scissors => 3,
        }
    }

    pub fn from_str(s: &str) -> GameChoice {
        match s {
            "X" | "A" => GameChoice::Rock,
            "Y" | "B" => GameChoice::Paper,
            "Z" | "C" => GameChoice::Scissors,
            _ => panic!("'{}' is invalid representation of GameChoice", s),
        }
    }
}

struct Round {
    them: GameChoice,
    us: GameChoice,
}

enum Winner {
    Them,
    Us,
    Tie,
}

impl Winner {
    pub fn value(&self) -> u32 {
        match self {
            Winner::Them => 0,
            Winner::Us => 6,
            Winner::Tie => 3,
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "X" | "A" => Winner::Them,
            "Y" | "B" => Winner::Tie,
            "Z" | "C" => Winner::Us,
            _ => panic!("'{}' is invalid representation of Winner", s),
        }
    }
}
impl Round {
    pub fn new(them: GameChoice, us: GameChoice) -> Self {
        Self { them, us }
    }

    pub fn from_outcome(them: GameChoice, outcome: Winner) -> Self {
        let us = match &them {
            GameChoice::Rock => match outcome {
                Winner::Them => GameChoice::Scissors,
                Winner::Us => GameChoice::Paper,
                Winner::Tie => GameChoice::Rock,
            },
            GameChoice::Paper => match outcome {
                Winner::Them => GameChoice::Rock,
                Winner::Us => GameChoice::Scissors,
                Winner::Tie => GameChoice::Paper,
            },
            GameChoice::Scissors => match outcome {
                Winner::Them => GameChoice::Paper,
                Winner::Us => GameChoice::Rock,
                Winner::Tie => GameChoice::Scissors,
            },
        };
        Self { us, them }
    }

    pub fn winner(&self) -> Winner {
        match self.us {
            GameChoice::Rock => match self.them {
                GameChoice::Rock => Winner::Tie,
                GameChoice::Paper => Winner::Them,
                GameChoice::Scissors => Winner::Us,
            },
            GameChoice::Paper => match self.them {
                GameChoice::Rock => Winner::Us,
                GameChoice::Paper => Winner::Tie,
                GameChoice::Scissors => Winner::Them,
            },
            GameChoice::Scissors => match self.them {
                GameChoice::Rock => Winner::Them,
                GameChoice::Paper => Winner::Us,
                GameChoice::Scissors => Winner::Tie,
            },
        }
    }

    pub fn points(&self) -> u32 {
        self.us.value() + self.winner().value()
    }
}

fn main() {
    let input = aoc::load_input(env!("CARGO_MANIFEST_DIR"), "input.txt");

    println!("Part1:");
    let mut total = 0;
    for line in input.lines() {
        let (them, us) = line.split_once(' ').unwrap();

        let round = Round::new(GameChoice::from_str(them), GameChoice::from_str(us));

        total = total + round.points();
    }
    println!("Total: {}", total);

    println!("Part2:");
    let mut total = 0;
    for line in input.lines() {
        let (them, us) = line.split_once(' ').unwrap();

        let round = Round::from_outcome(GameChoice::from_str(them), Winner::from_str(us));

        total = total + round.points();
    }
    println!("Total: {}", total);
}
