use cached::proc_macro::cached;
use itertools::Itertools;

#[derive(Clone, Copy, Debug)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Dot {
    y: usize,
    x: usize,
}

impl Dot {
    pub fn fold(mut self, fold: Fold) -> Self {
        if let Fold::Vertical(x) = fold {
            self.x = x - (self.x as isize - x as isize).abs() as usize;
        } else if let Fold::Horizontal(y) = fold {
            self.y = y - (self.y as isize - y as isize).abs() as usize;
        }
        self
    }
}

fn render_dots(dots: &[Dot]) -> String {
    let mut output = String::new();

    let (mut x, mut y) = (0, 0);
    for dot in dots {
        while y < dot.y {
            output.push('\n');
            y += 1;
            x = 0;
        }
        while x < dot.x {
            output.push(' ');
            x += 1;
        }
        output.push('#');
        x += 1;
    }

    output
}

fn fold_dots(fold: Fold, dots: &[Dot]) -> Vec<Dot> {
    dots.iter()
        .map(|dot| dot.fold(fold))
        .sorted()
        .dedup()
        .collect()
}

pub fn part_one(input: &'static str) -> usize {
    let (mut dots, folds) = parse_input(input);
    folds
        .iter()
        .take(1)
        .for_each(|&fold| dots = fold_dots(fold, &dots));
    dots.len()
}

pub fn part_two(input: &'static str) -> String {
    let (mut dots, folds) = parse_input(input);
    folds.iter().for_each(|&fold| dots = fold_dots(fold, &dots));
    render_dots(&dots)
}

#[cached]
fn parse_input(input: &'static str) -> (Vec<Dot>, Vec<Fold>) {
    let (dots, folds) = input.split_once("\n\n").unwrap();
    (
        dots.lines()
            .map(|coords| {
                let (x, y) = coords.split_once(',').unwrap();
                let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
                Dot { x, y }
            })
            .collect(),
        folds
            .lines()
            .map(|fold| {
                let (dir, pos) = fold
                    .strip_prefix("fold along ")
                    .unwrap()
                    .split_once('=')
                    .unwrap();
                let pos = pos.parse().unwrap();
                match dir {
                    "x" => Fold::Vertical(pos),
                    "y" => Fold::Horizontal(pos),
                    _ => unreachable!(),
                }
            })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 17);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(
            part_two(SAMPLE),
            "
#####
#   #
#   #
#   #
#####
            "
            .trim()
        );
    }
}
