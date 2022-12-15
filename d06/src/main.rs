use aoc::*;
use itertools::Itertools;

fn main() {
    let input = read_input::<String>("d06/input.txt");
    let message = |size| {
        input
            .as_bytes()
            .windows(size)
            .position(|marker| marker.iter().all_unique())
            .unwrap()
            + size
    };

    output!(message(4), message(14));
}
