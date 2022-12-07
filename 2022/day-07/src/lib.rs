use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::anyhow;
use itertools::Itertools;

#[derive(Debug, Hash, PartialEq, Eq)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

#[derive(Debug, Hash, PartialEq, Eq)]
enum Item<'a> {
    File(File<'a>),
    Dir { name: &'a str },
}

impl<'a> TryFrom<&[&'a str]> for Item<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &[&'a str]) -> Result<Self, Self::Error> {
        Ok(match value {
            ["dir", name] => Item::Dir { name },
            [size, name] => Item::File(File {
                size: size.parse()?,
                name,
            }),
            p => return Err(anyhow!("invalid item pattern: {:#?}", p)),
        })
    }
}

enum Command<'a> {
    Cd { rel_path: &'a str },
    Ls { items: Vec<Item<'a>> },
}

impl<'a> TryFrom<&'a str> for Command<'a> {
    type Error = anyhow::Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let mut lines = s.lines().map(|line| line.split(' ').collect::<Vec<_>>());
        let mut command = match lines.next().ok_or_else(|| anyhow!("empty command"))?[..] {
            ["cd", rel_path] => Command::Cd { rel_path },
            ["ls"] => Command::Ls { items: Vec::new() },
            ref p => return Err(anyhow!("invalid command pattern: {:#?}", p)),
        };

        if let Command::Ls { ref mut items } = command {
            for line in lines {
                items.push(Item::try_from(&line[..])?)
            }
        }

        Ok(command)
    }
}

type Filesystem<'a> = HashMap<PathBuf, Vec<Item<'a>>>;
type Sizes = HashMap<PathBuf, usize>;

fn build_filesystem(commands: Vec<Command>) -> Filesystem {
    commands
        .into_iter()
        .fold(
            (PathBuf::new(), Filesystem::new()),
            |(mut cwd, mut filesystem), command| {
                match command {
                    Command::Cd { rel_path } => {
                        if rel_path == ".." {
                            cwd.pop();
                        } else {
                            cwd.push(rel_path);
                            if !filesystem.contains_key(&cwd) {
                                filesystem.insert(cwd.to_owned(), Vec::new());
                            }
                        }
                    }
                    Command::Ls { items } => items.into_iter().for_each(|item| {
                        filesystem
                            .get_mut(&cwd)
                            .expect("values are always initialized upon entering directory")
                            .push(item)
                    }),
                }
                (cwd, filesystem)
            },
        )
        .1
}

fn get_size(path: &PathBuf, sizes: &mut Sizes, filesystem: &Filesystem) -> anyhow::Result<usize> {
    if !sizes.contains_key(path) {
        let items = filesystem
            .get(path)
            .ok_or_else(|| anyhow!("invalid path"))?;
        let size = items
            .iter()
            .map(|item| match item {
                Item::File(File { size, .. }) => *size,
                Item::Dir { name } => get_size(&path.join(name), sizes, filesystem)
                    .expect("paths provided by Filesystem are always valid"),
            })
            .sum();
        sizes.insert(path.to_owned(), size);
    }

    Ok(*sizes.get(path).expect("size is ensured to be set above"))
}

fn find_sizes(filesystem: Filesystem) -> Sizes {
    filesystem.keys().fold(Sizes::new(), |mut sizes, path| {
        get_size(path, &mut sizes, &filesystem)
            .expect("paths provided by Filesystem are always valid");
        sizes
    })
}

pub fn part_one(input: &'static str) -> usize {
    find_sizes(build_filesystem(parse_input(input)))
        .into_values()
        .filter(|&size| size < 100_000)
        .sum()
}

pub fn part_two(input: &'static str) -> usize {
    const TOTAL_SPACE: usize = 70_000_000;
    const REQUIRED_SPACE: usize = 30_000_000;

    let sizes = find_sizes(build_filesystem(parse_input(input)));
    let additional_space_required =
        REQUIRED_SPACE - (TOTAL_SPACE - sizes.get(Path::new("/")).unwrap());
    sizes
        .into_values()
        .sorted()
        .find(|size| *size >= additional_space_required)
        .unwrap()
}

fn parse_input(input: &'static str) -> Vec<Command> {
    input
        .split('$')
        .filter_map(|s| (!s.is_empty()).then_some(s.trim()))
        .map(|s| Command::try_from(s).expect("AOC input will only contain valid commands"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 24933642);
    }
}
