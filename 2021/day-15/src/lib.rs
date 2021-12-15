use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;

type Pos = (usize, usize);
type Danger = usize;

#[derive(Debug, Clone, Default)]
pub struct CaveMap {
    cells: Vec<Danger>,
    width: usize,
    height: usize,
}

impl CaveMap {
    pub fn new(cells: &[Danger], width: usize, height: usize) -> Result<Self> {
        Ok(Self {
            cells: cells.to_vec(),
            width,
            height,
        })
    }

    pub fn get_cell(&self, position: Pos) -> Option<Danger> {
        if position.0 < self.width && position.1 < self.height {
            let index = self.index_from_pos(position);
            self.cells.get(index).copied()
        } else {
            None
        }
    }

    pub fn get_adjacent_cells(&self, position: Pos) -> Vec<(Pos, Danger)> {
        let (x, y) = position;
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(xo, yo)| {
                let (x, y) = (xo + x as isize, yo + y as isize);
                if let (Ok(x), Ok(y)) = (x.try_into(), y.try_into()) {
                    if let Some(cell) = self.get_cell((x, y)) {
                        return Some(((x, y), cell));
                    }
                }
                None
            })
            .flatten()
            .collect()
    }

    pub fn cells(&self) -> Vec<(Pos, Danger)> {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, &cell)| (self.pos_from_index(i), cell))
            .collect()
    }

    fn index_from_pos(&self, position: Pos) -> usize {
        position.0 + position.1 * self.width
    }

    fn pos_from_index(&self, index: usize) -> Pos {
        (index % self.width, index / self.width)
    }
}

/// Finds the safest path using Dijkstra and returns its total danger value
fn safest_path(map: &CaveMap) -> Option<Danger> {
    let (start_pos, end_pos) = ((0, 0), (map.width - 1, map.height - 1));

    let mut total_danger: HashMap<Pos, Danger> = map
        .cells()
        .into_iter()
        .map(|(pos, _)| (pos, if pos == start_pos { 0 } else { Danger::MAX }))
        .collect();
    let mut best_parent: HashMap<Pos, Pos> = HashMap::new();
    let mut queue: Vec<Pos> = vec![start_pos];

    while !queue.is_empty() {
        // Get most promising elem from queue
        let pos = queue.pop().unwrap();

        // Found target
        if pos == end_pos {
            return Some(total_danger[&end_pos]);
        }

        for (neighbor_pos, neighbor_danger) in map.get_adjacent_cells(pos) {
            // Check if this path to `neighbor` is less dangerest than previous best
            let alt_total_danger = total_danger[&pos] + neighbor_danger;
            if alt_total_danger < total_danger[&neighbor_pos] {
                // Update best path
                total_danger.insert(neighbor_pos, alt_total_danger);
                best_parent.insert(neighbor_pos, pos);

                // Queue neighbor for expansion (insert pre-sorted)
                let idx = queue
                    .binary_search_by(|other| alt_total_danger.cmp(&total_danger[other]))
                    .unwrap_or_else(|i| i);
                queue.insert(idx, neighbor_pos);
            }
        }
    }
    None
}

pub fn part_one(input: &'static str) -> usize {
    let map = parse_input(input, 1);
    safest_path(&map).unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let map = parse_input(input, 5);
    safest_path(&map).unwrap()
}

fn parse_input(input: &'static str, scale: usize) -> CaveMap {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    CaveMap::new(
        &(0..scale)
            .flat_map(|y| {
                input.lines().flat_map(move |line| {
                    let cells = line.chars().map(|c| c.to_digit(10).unwrap() as usize);
                    (0..scale).flat_map(move |x| {
                        cells.clone().map(|c| (c + x + y - 1) % 9 + 1).collect_vec()
                    })
                })
            })
            .collect_vec(),
        width * scale,
        height * scale,
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 315);
    }
}
