use aoc::*;
use std::any::Any;
use std::str::FromStr;

const DISK_SIZE: usize = 70_000_000;
const REQUIRED_DISK_SPACE: usize = 30_000_000;

#[derive(Debug)]
enum Command {
    ChangeDirectory(Directory),
    List(List),
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some(("cd", arguments)) = s.split_once(' ') {
            Command::ChangeDirectory(arguments.parse()?)
        } else {
            Command::List(s.parse()?)
        })
    }
}

impl FromStr for Directory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Directory::new(s.trim_end().to_string()))
    }
}

#[derive(Debug)]
struct List {
    files: Vec<Box<dyn FileType>>,
}

impl FromStr for List {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(List {
            files: s
                .lines()
                .skip(1)
                .map(|l| {
                    let (file_type, name) = l.split_once(' ').unwrap();
                    let name = name.trim_end().to_string();
                    match file_type {
                        "dir" => Box::new(Directory::new(name)) as Box<dyn FileType>,
                        size => Box::new(File::new(name, size.parse().unwrap())),
                    }
                })
                .collect(),
        })
    }
}

trait FileType: std::fmt::Debug + FileTypeToAny {
    fn name(&self) -> String;
    fn size(&self) -> usize;
    fn flatten(&self) -> Vec<&Directory>;
}

trait FileTypeToAny: 'static + Any {
    fn as_any(&mut self) -> &mut dyn Any;
}

impl<T: 'static + Any> FileTypeToAny for T {
    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

impl File {
    pub fn new(name: String, size: usize) -> Self {
        File { name, size }
    }
}

#[derive(Debug)]
struct Directory {
    name: String,
    children: Vec<Box<dyn FileType>>,
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            children: vec![],
        }
    }

    fn extend(&mut self, list: List) {
        self.children.extend(list.files.into_iter())
    }

    pub fn ingest<I: IntoIterator<Item = Command>>(
        &mut self,
        iter: I,
    ) -> <I as IntoIterator>::IntoIter {
        let mut input = iter.into_iter();
        while let Some(command) = input.next() {
            match command {
                Command::ChangeDirectory(cd) => {
                    if cd.name() != *".." {
                        let index = self
                            .children
                            .iter()
                            .position(|r| r.name() == cd.name())
                            .unwrap();
                        let it: &mut dyn Any = (*self.children[index]).as_any();
                        input = if let Some(dir) = it.downcast_mut::<Directory>() {
                            dir.ingest(input)
                        } else {
                            input
                        };
                    } else {
                        break;
                    }
                }
                Command::List(ls) => self.extend(ls),
            }
        }
        input
    }

    pub fn iter(&self) -> DirectoryIter {
        DirectoryIter {
            files: self.flatten(),
            count: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.children.iter().map(|f| f.size()).sum()
    }
}

impl FileType for Directory {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> usize {
        self.children.iter().map(|f| f.size()).sum()
    }

    fn flatten(&self) -> Vec<&Directory> {
        [
            vec![self],
            self.children.iter().flat_map(|c| c.flatten()).collect(),
        ]
        .concat()
    }
}
impl FileType for File {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn size(&self) -> usize {
        self.size
    }

    fn flatten(&self) -> Vec<&Directory> {
        vec![]
    }
}

impl FromIterator<Command> for Directory {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        iter.next();
        if let Some(Command::ChangeDirectory(cd)) = iter.next() {
            let mut root = Directory::new(cd.name());
            root.ingest(&mut iter);
            return root;
        }
        unreachable!()
    }
}

struct DirectoryIter<'a> {
    files: Vec<&'a Directory>,
    count: usize,
}

impl<'a> Iterator for DirectoryIter<'a> {
    type Item = &'a Directory;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.files.len() {
            self.count += 1;
            Some(self.files[self.count - 1])
        } else {
            None
        }
    }
}

fn main() {
    let input = read::<Command>("d07/input.txt", "$ ");

    let root: Directory = input.into_iter().collect();

    output!(
        root.iter()
            .filter_map(|d| if d.size() <= 100000 {
                Some(d.size())
            } else {
                None
            })
            .sum::<usize>(),
        root.iter()
            .filter_map(|d| {
                if d.size() as isize - (REQUIRED_DISK_SPACE - (DISK_SIZE - root.size())) as isize
                    >= 0
                {
                    Some(d.size())
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    );
}
