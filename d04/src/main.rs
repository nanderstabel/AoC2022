use aoc::*;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range((u8, u8));

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Range(
            s.split_once('-')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap(),
        ))
    }
}

#[derive(Debug, Clone)]
struct RangePair((Range, Range));

impl FromStr for RangePair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RangePair(
            s.split_once(',')
                .map(|(start, end)| (start.parse().unwrap(), end.parse().unwrap()))
                .unwrap(),
        ))
    }
}

impl RangePair {
    fn is_fully_overlapped(&self) -> bool {
        self.0 .0 .0 .0 <= self.0 .1 .0 .0 && self.0 .0 .0 .1 >= self.0 .1 .0 .1
            || self.0 .1 .0 .0 <= self.0 .0 .0 .0 && self.0 .1 .0 .1 >= self.0 .0 .0 .1
    }

    fn is_partially_overlapped(&self) -> bool {
        for first in self.0 .0 .0 .0..=self.0 .0 .0 .1 {
            for last in self.0 .1 .0 .0..=self.0 .1 .0 .1 {
                if first == last {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let input = read::<RangePair>("d04/input", SINGLELINE);

    output!(
        input
            .iter()
            .filter_map(|c| c.is_fully_overlapped().then_some(true))
            .count(),
        input
            .iter()
            .filter_map(|c| c.is_partially_overlapped().then_some(true))
            .count()
    );
}
