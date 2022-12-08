use ndarray::Array2;
use std::collections::HashSet;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Tree {
    pos: Pos,
    height: u32,
}

type Pos = (usize, usize);

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);

    let edges = input
        .column(0)
        .into_iter()
        .chain(input.column(input.dim().1 - 1))
        .chain(input.row(0))
        .chain(input.row(input.dim().0 - 1))
        .copied();
    let mut visible: HashSet<Tree> = HashSet::from_iter(edges);

    input
        .columns()
        .into_iter()
        .chain(input.rows())
        .flat_map(|lane| -> [Box<dyn Iterator<Item = &Tree>>; 2] {
            [Box::new(lane.into_iter()), Box::new(lane.into_iter().rev())]
        })
        .for_each(|lane| {
            let mut highest = Tree::default();
            for tree in lane {
                if tree.height > highest.height {
                    highest = *tree;
                    visible.insert(highest);
                }
            }
        });
    visible.len()
}

pub fn add_dir(a: (isize, isize), b: Pos, dimensions: Pos) -> Option<Pos> {
    let (x, y) = (a.0 + b.0 as isize, a.1 + b.1 as isize);
    (x >= 0 && x < dimensions.0 as isize && y >= 0 && y < dimensions.1 as isize)
        .then_some((x as usize, y as usize))
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .iter()
        .map(|tree| {
            let mut scenic_score = 1;
            for dir in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let mut pos = tree.pos;
                let mut view = 0;
                loop {
                    if let Some(new_pos) = add_dir(dir, pos, input.dim()) {
                        pos = new_pos;
                        view += 1;

                        let other_tree = input.get(pos).unwrap();
                        if other_tree.height < tree.height {
                            continue;
                        }
                    }
                    break;
                }
                scenic_score *= view
            }
            scenic_score
        })
        .max()
        .unwrap()
}

fn parse_input(input: &'static str) -> Array2<Tree> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();

    Array2::from_shape_vec(
        (width, height),
        input
            .lines()
            .enumerate()
            .flat_map(|(y, s)| {
                s.chars().enumerate().map(move |(x, c)| Tree {
                    pos: (x, y),
                    height: c.to_digit(10).unwrap(),
                })
            })
            .collect::<Vec<_>>(),
    )
    .unwrap()
    .reversed_axes() // column-major -> row-major
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 8);
    }
}
