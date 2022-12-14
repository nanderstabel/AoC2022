use aoc::*;
use std::str::FromStr;

const DISK_SIZE: usize = 70_000_000;
const REQUIRED_DISK_SPACE: usize = 30_000_000;

#[derive(Debug)]
enum Command {
    ChangeDirectory(ChangeDirectory),
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

#[derive(Debug)]
struct ChangeDirectory {
    directory: FileType,
}

impl ChangeDirectory {
    pub fn name(&self) -> Option<String> {
        if let FileType::Directory(dir) = &self.directory {
            Some(dir.name.clone())
        } else {
            None
        }
    }
}

impl FromStr for ChangeDirectory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ChangeDirectory {
            directory: FileType::new_directory(s.trim_end().to_string()),
        })
    }
}

#[derive(Debug)]
struct List {
    files: Vec<FileType>,
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
                        "dir" => FileType::new_directory(name),
                        size => FileType::new_file(name, size.parse().unwrap()),
                    }
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    children: Vec<FileType>,
}

impl Directory {
    pub fn size(&self) -> usize {
        self.children.iter().map(|f| f.size()).sum()
    }
}

#[derive(Debug, Clone)]
enum FileType {
    Directory(Directory),
    File(File),
}

impl FromIterator<Command> for FileType {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        let mut iter = iter.into_iter();
        iter.next();
        if let Some(Command::ChangeDirectory(cd)) = iter.next() {
            let mut root = FileType::new_directory(cd.name().unwrap());
            root.ingest(&mut iter);
            return root;
        }
        unreachable!()
    }
}

impl FileType {
    pub fn new_directory(name: String) -> Self {
        Self::Directory(Directory {
            name,
            children: vec![],
        })
    }

    pub fn new_file(name: String, size: usize) -> Self {
        Self::File(File { name, size })
    }

    fn name(&self) -> String {
        match self {
            FileType::Directory(d) => d.name.clone(),
            FileType::File(f) => f.name.clone(),
        }
    }

    fn size(&self) -> usize {
        match self {
            FileType::Directory(d) => d.size(),
            FileType::File(f) => f.size,
        }
    }

    fn extend(&mut self, list: &List) {
        if let FileType::Directory(dir) = self {
            dir.children.extend(list.files.clone().into_iter())
        }
    }

    pub fn ingest<I: IntoIterator<Item = Command>>(
        &mut self,
        iter: I,
    ) -> <I as IntoIterator>::IntoIter {
        let mut input = iter.into_iter();
        while let Some(command) = input.next() {
            match command {
                Command::ChangeDirectory(cd) => {
                    if let Some(s) = cd.name() {
                        if s != *".." {
                            if let FileType::Directory(dir) = self {
                                let index =
                                    dir.children.iter().position(|r| r.name() == s).unwrap();
                                input = dir.children[index].ingest(input);
                            }
                        } else {
                            break;
                        }
                    }
                }
                Command::List(ls) => self.extend(&ls),
            }
        }
        input
    }

    pub fn flatten(&self) -> Vec<&FileType> {
        if let FileType::Directory(dir) = self {
            [
                vec![self],
                dir.children.iter().flat_map(|c| c.flatten()).collect(),
            ]
            .concat()
        } else {
            vec![]
        }
    }

    pub fn iter(&self) -> FileTypeIter {
        FileTypeIter {
            files: self.flatten(),
            count: 0,
        }
    }
}

struct FileTypeIter<'a> {
    files: Vec<&'a FileType>,
    count: usize,
}

impl<'a> Iterator for FileTypeIter<'a> {
    type Item = &'a FileType;

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
    let input = read::<Command>("d07/input", "$ ");

    let root: FileType = input.into_iter().collect();

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
