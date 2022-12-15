use aoc::*;
use std::str::FromStr;

#[derive(Debug)]
struct Item(u8);

#[derive(Debug, Clone)]
struct Rucksack {
    compartments: [Vec<u8>; 2],
}

impl FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decode = |b| match b {
            'a'..='z' => b as u8 - 96,
            'A'..='Z' => b as u8 - 38,
            _ => unreachable!(),
        };

        let (a, b) = s.split_at(s.len() / 2);
        Ok(Self {
            compartments: [
                a.chars().map(decode).collect(),
                b.chars().map(decode).collect(),
            ],
        })
    }
}

impl<'a> IntoIterator for &'a Rucksack {
    type Item = &'a u8;
    type IntoIter = RucksackIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RucksackIter {
            items: self
                .compartments
                .iter()
                .flat_map(|c| c.iter().collect::<Vec<&u8>>())
                .collect(),
            index: 0,
        }
    }
}

struct RucksackIter<'a> {
    items: Vec<&'a u8>,
    index: usize,
}

impl<'a> Iterator for RucksackIter<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.items.len() {
            return None;
        }
        self.index += 1;
        Some(self.items[self.index - 1])
    }
}

impl Rucksack {
    fn intersect(&self) -> u8 {
        for a in self.compartments[0].iter() {
            for b in self.compartments[1].iter() {
                if *a == *b {
                    return *a;
                }
            }
        }
        0
    }
}

#[derive(Debug)]
struct Group {
    rucksacks: Vec<Rucksack>,
}

impl Group {
    fn intersect(&self) -> u8 {
        for a in self.rucksacks[0].into_iter() {
            for b in self.rucksacks[1].into_iter() {
                if a == b {
                    for c in self.rucksacks[2].into_iter() {
                        if a == c {
                            return *a;
                        }
                    }
                }
            }
        }
        0
    }
}

fn main() {
    let input = read::<Rucksack>("d03/input.txt", SINGLELINE);
    let input2 = input.chunks_exact(3).map(|c| Group {
        rucksacks: c.to_vec(),
    });

    output!(
        input.iter().map(|r| r.intersect() as u32).sum::<u32>(),
        input2.map(|g| g.intersect() as u32).sum::<u32>()
    );
}
