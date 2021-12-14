use std::collections::HashSet;

fn is_lowest_neighbor(map: &[Vec<usize>], pos: (usize, usize)) -> Option<usize> {
    let this_point = map[pos.1][pos.0];

    if get_neighbors(map, pos)
        .iter()
        .map(|(v, _pos)| v)
        .all(|&h| h > this_point)
    {
        Some(this_point)
    } else {
        None
    }
}

fn get_neighbors(map: &[Vec<usize>], pos: (usize, usize)) -> Vec<(usize, (usize, usize))> {
    let (width, height) = (map[0].len(), map.len());
    let (xi, yi) = pos;
    [(0, -1), (0, 1), (-1, 0), (1, 0)]
        .iter()
        .filter_map(|(xo, yo)| {
            let (x, y) = (xo + xi as isize, yo + yi as isize);
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                let (x, y) = (x as usize, y as usize);
                Some((map[y][x], (x, y)))
            } else {
                None
            }
        })
        .collect()
}

fn fill_basin(map: &[Vec<usize>], pos: (usize, usize)) -> Vec<(usize, (usize, usize))> {
    let this_point = map[pos.1][pos.0];
    let neighbors = get_neighbors(map, pos);
    let flowers = neighbors
        .iter()
        .filter(|(v, _pos)| *v > this_point && *v < 9)
        .flat_map(|(v, pos)| {
            let mut basin_section = fill_basin(map, *pos);
            basin_section.push((*v, *pos));
            basin_section
        });
    let mut flowers: HashSet<_> = flowers.collect();
    flowers.insert((this_point, pos));
    flowers.iter().copied().collect()
}

fn is_basin(map: &[Vec<usize>], pos: (usize, usize)) -> Option<usize> {
    if is_lowest_neighbor(map, pos).is_some() {
        Some(fill_basin(map, pos).len())
    } else {
        None
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    let mut lowest_points = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(height) = is_lowest_neighbor(&input, (x, y)) {
                lowest_points.push(height);
            }
        }
    }
    lowest_points.iter().map(|h| h + 1).sum()
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    let mut basins = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[0].len() {
            if let Some(basin) = is_basin(&input, (x, y)) {
                basins.push(basin);
            }
        }
    }
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

fn parse_input(input: &'static str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|s| {
            s.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 1134);
    }
}
