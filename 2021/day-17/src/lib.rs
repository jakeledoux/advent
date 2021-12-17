use std::ops::RangeInclusive;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Default, Copy, Clone)]
pub struct Pos<T> {
    x: T,
    y: T,
}

fn simulate(mut vel: Pos<isize>, cutoff: Pos<isize>) -> Vec<Pos<isize>> {
    let mut probe = Pos::default();
    let mut pos_log = Vec::new();
    while probe.x <= cutoff.x && probe.y >= cutoff.y {
        pos_log.push(probe);
        probe.x += vel.x;
        probe.y += vel.y;
        vel.x = (vel.x - 1).max(0);
        vel.y -= 1;
    }

    pos_log
}

fn find_intersecting_paths(target: Pos<RangeInclusive<isize>>) -> Vec<Vec<Pos<isize>>> {
    let in_target = |pos: &Pos<isize>| target.x.contains(&pos.x) && target.y.contains(&pos.y);
    let cutoff = Pos {
        x: *target.x.end(),
        y: *target.y.start(),
    };
    let all_coords = (0..cutoff.x * 10)
        .flat_map(move |x| (-(cutoff.y).abs()..(cutoff.y * 10).abs()).map(move |y| (x, y)));
    all_coords
        .map(|(x, y)| {
            let vel = Pos { x, y };
            simulate(vel, cutoff)
        })
        .filter(|path| path.iter().any(in_target))
        .collect()
}

pub fn part_one(input: &'static str) -> usize {
    let target_area = parse_input(input);
    find_intersecting_paths(target_area)
        .into_iter()
        .map(|path| *path.iter().map(|Pos { y, .. }| y).max().unwrap())
        .max()
        .unwrap() as usize
}

pub fn part_two(input: &'static str) -> usize {
    let target_area = parse_input(input);
    find_intersecting_paths(target_area).len()
}

pub fn parse_input(input: &'static str) -> Pos<RangeInclusive<isize>> {
    let re = Regex::new(r"x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)");
    let bounds = re
        .unwrap()
        .captures(input)
        .unwrap()
        .iter()
        .skip(1)
        .map(|s| s.unwrap().as_str().parse().unwrap())
        .collect_vec();

    Pos {
        x: bounds[0]..=bounds[1],
        y: bounds[2]..=bounds[3],
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{parse_input, part_one, part_two, simulate, Pos};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 45);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 112);
    }

    #[test]
    fn test_part_two_examples() {
        let mut examples = "
            23,-10  25,-9   27,-5   29,-6   22,-6   21,-7   9,0     27,-7   24,-5
            25,-7   26,-6   25,-5   6,8     11,-2   20,-5   29,-10  6,3     28,-7
            8,0     30,-6   29,-8   20,-10  6,7     6,4     6,1     14,-4   21,-6
            26,-10  7,-1    7,7     8,-1    21,-9   6,2     20,-7   30,-10  14,-3
            20,-8   13,-2   7,3     28,-8   29,-9   15,-3   22,-5   26,-8   25,-8
            25,-6   15,-4   9,-2    15,-2   12,-2   28,-9   12,-3   24,-6   23,-7
            25,-10  7,8     11,-3   26,-7   7,1     23,-9   6,0     22,-10  27,-6
            8,1     22,-8   13,-4   7,6     28,-6   11,-4   12,-4   26,-9   7,4
            24,-10  23,-8   30,-8   7,0     9,-1    10,-1   26,-5   22,-9   6,5
            7,5     23,-6   28,-10  10,-2   11,-1   20,-9   14,-2   29,-7   13,-3
            23,-5   24,-8   27,-9   30,-7   28,-5   21,-10  7,9     6,6     21,-5
            27,-10  7,2     30,-9   21,-8   22,-7   24,-9   20,-6   6,9     29,-5
            8,-2    27,-8   30,-5   24,-7"
            .split_whitespace()
            .map(|vel_str| {
                vel_str
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            });
        let target = parse_input(SAMPLE);
        let in_target = |pos: &Pos<isize>| target.x.contains(&pos.x) && target.y.contains(&pos.y);
        let cutoff = Pos {
            x: *target.x.end(),
            y: *target.y.start(),
        };
        assert!(examples.all(|vel: (isize, isize)| {
            let vel = Pos { x: vel.0, y: vel.1 };
            let path = simulate(vel, cutoff);
            path.iter().any(in_target)
        }));
    }
}
