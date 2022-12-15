use aoc::*;
use std::str::FromStr;

#[derive(Debug, Clone, Default)]
struct Stack {
    crates: Vec<char>,
}

impl FromIterator<char> for Stack {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut stack = Stack::default();
        for i in iter {
            stack.crates.push(i);
        }
        stack
    }
}

#[derive(Debug, Default, Clone)]
struct Procedure {
    n: u8,
    from: usize,
    to: usize,
}

impl FromStr for Procedure {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<&str> = s.split_whitespace().collect();

        Ok(Procedure {
            n: values[1].parse().unwrap(),
            from: values[3].parse().unwrap(),
            to: values[5].parse().unwrap(),
        })
    }
}

#[derive(Debug)]
enum CrateMover {
    M9000(Setup),
    M9001(Setup),
}

impl CrateMover {
    fn setup(&self) -> Setup {
        match self {
            CrateMover::M9000(setup) => setup.clone(),
            CrateMover::M9001(setup) => setup.clone(),
        }
    }

    fn commit_procedures(&mut self) {
        match self {
            CrateMover::M9000(setup) => {
                for Procedure { n, from, to } in setup.procedures.iter() {
                    for _ in 0..*n {
                        if let Some(c) = setup.stacks[*from - 1].crates.pop() {
                            setup.stacks[*to - 1].crates.push(c);
                        }
                    }
                }
            }
            CrateMover::M9001(setup) => {
                for Procedure { n, from, to } in setup.procedures.iter() {
                    let mut temp = vec![];
                    for _ in 0..*n {
                        if let Some(c) = setup.stacks[*from - 1].crates.pop() {
                            temp.push(c);
                        }
                    }
                    temp.reverse();
                    setup.stacks[*to - 1].crates.append(&mut temp);
                }
            }
        }
    }

    fn get_top_crates(&self) -> String {
        match self {
            CrateMover::M9000(setup) => setup.get_top_crates(),
            CrateMover::M9001(setup) => setup.get_top_crates(),
        }
    }
}

#[derive(Debug, Clone)]
struct Setup {
    stacks: Vec<Stack>,
    procedures: Vec<Procedure>,
}

impl FromStr for Setup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (stacks, procedures) = s.split_once(MULTILINE).unwrap();
        let stacks: Vec<Vec<char>> = stacks.lines().map(|l| l.chars().collect()).collect();

        Ok(Setup {
            stacks: (0..stacks[0].len())
                .filter_map(|i| {
                    stacks[stacks.len() - 1][i].is_ascii_digit().then_some(
                        (0..stacks.len())
                            .rev()
                            .filter_map(|j| {
                                stacks[j][i].is_ascii_uppercase().then_some(stacks[j][i])
                            })
                            .collect(),
                    )
                })
                .collect(),
            procedures: procedures.lines().map(|l| l.parse().unwrap()).collect(),
        })
    }
}

impl Setup {
    fn get_top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|s| s.crates[s.crates.len() - 1])
            .collect()
    }
}

fn main() {
    let mut crate_mover_9000 = CrateMover::M9000(read_input::<Setup>("d05/input.txt"));
    let mut crate_mover_9001 = CrateMover::M9001(crate_mover_9000.setup());

    crate_mover_9000.commit_procedures();
    crate_mover_9001.commit_procedures();

    output!(
        crate_mover_9000.get_top_crates(),
        crate_mover_9001.get_top_crates()
    );
}
