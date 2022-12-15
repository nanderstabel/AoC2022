use crate::Visibility::*;
use aoc::*;
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::cmp::{max, Ordering};
use std::str::FromStr;

#[derive(Debug, Clone)]
enum Visibility {
    West(i8),
    East(i8),
    North(i8),
    South(i8),
    Hidden(i8),
}

impl Visibility {
    pub fn height(&self) -> i8 {
        match self {
            Self::West(height) => *height,
            Self::East(height) => *height,
            Self::North(height) => *height,
            Self::South(height) => *height,
            Self::Hidden(height) => *height,
        }
    }
}

#[derive(Debug, Clone)]
struct Forest {
    trees: Vec<Vec<Visibility>>,
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Forest {
            trees: s
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|b| Hidden(b.to_digit(10).unwrap() as i8))
                        .collect()
                })
                .collect(),
        })
    }
}

impl Forest {
    pub fn get_total_visible(&mut self) -> usize {
        macro_rules! visibility {
            ($outer_idx:ident, $outer:expr, $inner_idx:ident, $inner:expr, $direction:expr, $perp:path, $perp2:path, $opposite:path, $outer_idx2:ident, $inner_idx2:ident) => {
                $outer
                    .map(|$outer_idx| {
                        let mut height = -1;
                        $inner
                            .fold_while(0, |acc, $inner_idx| {
                                match self.trees[$inner_idx2][$outer_idx2] {
                                    Hidden(tree) => {
                                        if tree > height {
                                            self.trees[$inner_idx2][$outer_idx2] = $direction(tree);
                                            height = tree;
                                            Continue(acc + 1)
                                        } else {
                                            Continue(acc)
                                        }
                                    }
                                    $perp(tree) | $perp2(tree) => {
                                        height = max(height, tree);
                                        Continue(acc)
                                    }
                                    $opposite(_) => Done(acc),
                                    _ => Continue(acc),
                                }
                            })
                            .into_inner()
                    })
                    .sum()
            };
            (hor, $direction:expr, $opposite:path, $inner:expr) => {
                visibility!(
                    i,
                    0..self.trees.len(),
                    j,
                    $inner,
                    $direction,
                    North,
                    South,
                    $opposite,
                    j,
                    i
                )
            };
            (ver, $direction:expr, $opposite:path, $inner:expr) => {
                visibility!(
                    j,
                    0..self.trees[0].len(),
                    i,
                    $inner,
                    $direction,
                    West,
                    East,
                    $opposite,
                    j,
                    i
                )
            };
        }
        [
            visibility!(hor, West, East, 0..self.trees[0].len()),
            visibility!(hor, East, West, (0..self.trees[0].len()).rev()),
            visibility!(ver, North, South, 0..self.trees.len()),
            visibility!(ver, South, North, (0..self.trees.len()).rev()),
        ]
        .iter()
        .sum()
    }

    pub fn get_max_scenic_score(&mut self) -> usize {
        let compare = |tree: i8, height, acc| match tree.cmp(&height) {
            Ordering::Less => Continue(acc + 1),
            Ordering::Equal => Done(acc + 1),
            Ordering::Greater => Done(acc),
        };

        (1..(self.trees.len() - 1))
            .map(|x| {
                (1..(self.trees[0].len() - 1))
                    .map(|y| {
                        macro_rules! distance {
                            ($range:expr, $idx:ident, $x:ident, $y:ident) => {
                                $range
                                    .fold_while(0, |acc, $idx| {
                                        compare(
                                            self.trees[$x][$y].height(),
                                            self.trees[x][y].height(),
                                            acc,
                                        )
                                    })
                                    .into_inner()
                            };
                            (x, $range:expr) => {
                                distance!($range, j, x, j)
                            };
                            (y, $range:expr) => {
                                distance!($range, i, i, y)
                            };
                        }
                        [
                            distance!(x, ((0..y).rev())),
                            distance!(y, ((0..x).rev())),
                            distance!(x, ((y + 1)..self.trees.len())),
                            distance!(y, ((x + 1)..self.trees[0].len())),
                        ]
                        .iter()
                        .product()
                    })
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let mut input = read_input::<Forest>("d09/input.txt");

    output!(input.get_total_visible(), input.get_max_scenic_score());
}
