use aoc::*;
use num_derive::FromPrimitive;
use std::{cmp::Ordering, str::FromStr};

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Play {
    Rock = 1,
    Paper,
    Scissors,
}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (*self, *other) {
            (Play::Rock, Play::Paper) => Some(Ordering::Less),
            (Play::Rock, Play::Scissors) => Some(Ordering::Greater),
            (Play::Paper, Play::Rock) => Some(Ordering::Greater),
            (Play::Paper, Play::Scissors) => Some(Ordering::Less),
            (Play::Scissors, Play::Rock) => Some(Ordering::Less),
            (Play::Scissors, Play::Paper) => Some(Ordering::Greater),
            _ => Some(Ordering::Equal),
        }
    }
}

impl FromStr for Play {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        })
    }
}

impl Play {
    fn less(&self) -> Self {
        match *self {
            Play::Rock => Self::Scissors,
            Play::Paper => Self::Rock,
            Play::Scissors => Self::Paper,
        }
    }

    fn greater(&self) -> Self {
        match *self {
            Play::Rock => Self::Paper,
            Play::Paper => Self::Scissors,
            Play::Scissors => Self::Rock,
        }
    }
}

#[derive(Debug)]
struct Round(Play, String);

impl FromStr for Round {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut temp = s.split_whitespace();

        Ok(Self(
            temp.next().unwrap().parse()?,
            temp.next().unwrap().to_string(),
        ))
    }
}

fn get_result(round: &Round) -> u32 {
    let own_hand = round.1.parse::<Play>().unwrap();
    own_hand as u32
        + match own_hand.partial_cmp(&round.0).unwrap() {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
}

fn get_play(round: &Round) -> u32 {
    match round.1.as_str() {
        "X" => round.0.less() as u32,
        "Y" => round.0 as u32 + 3,
        "Z" => round.0.greater() as u32 + 6,
        _ => unreachable!(),
    }
}

fn main() {
    let input = read::<Round>("d02/input", SINGLELINE);

    output!(input.iter().fold((0, 0), |sum, val| {
        (sum.0 + get_result(val), sum.1 + get_play(val))
    }));
}
