use aoc::*;
use std::iter::Sum;
use std::str::FromStr;

#[derive(Debug)]
struct Food {
    calories: u32,
}

impl FromStr for Food {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            calories: s.parse().unwrap(),
        })
    }
}

impl<'a> Sum<&'a Self> for Food {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = &'a Self>,
    {
        iter.fold(Self { calories: 0 }, |a, b| Self {
            calories: a.calories + b.calories,
        })
    }
}

impl Food {
    fn get_calories(&self) -> u32 {
        self.calories
    }
}

#[derive(Debug)]
struct Elf {
    food: Vec<Food>,
}

impl FromStr for Elf {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            food: s.lines().map(|s| s.parse().unwrap()).collect(),
        })
    }
}

impl Elf {
    pub fn get_total_calories(&self) -> u32 {
        self.food.iter().sum::<Food>().get_calories()
    }
}

fn main() {
    let input = read::<Elf>("d01/input.txt", MULTILINE);

    let mut all_calories = input
        .iter()
        .map(|elf| elf.get_total_calories())
        .collect::<Vec<u32>>();

    all_calories.sort();
    let max = *all_calories.iter().max().unwrap();
    all_calories.drain(0..(all_calories.len() - 3));

    let sum = all_calories.iter().sum::<u32>();

    output!(max, sum)
}
