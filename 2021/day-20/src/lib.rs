use std::{collections::HashMap, ops::RangeInclusive};

use itertools::{Itertools, MinMaxResult};

type Algorithm = Vec<Cell>;
type SpatialCell = (Pos, Cell);
type CellMap = HashMap<Pos, Cell>;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Bounds {
    min: Pos,
    max: Pos,
}

impl Bounds {
    pub fn horizontal_range(&self) -> RangeInclusive<isize> {
        self.min.x..=self.max.x
    }

    pub fn vertical_range(&self) -> RangeInclusive<isize> {
        self.min.y..=self.max.y
    }

    fn contains(&self, pos: Pos) -> bool {
        self.horizontal_range().contains(&pos.x) && self.vertical_range().contains(&pos.y)
    }
}

impl Bounds {
    pub fn new(min: Pos, max: Pos) -> Self {
        Self { min, max }
    }

    pub fn expand(self) -> Self {
        Self {
            min: self.min.expand_min(),
            max: self.max.expand_max(),
        }
    }
}

impl From<&CellMap> for Bounds {
    fn from(cells: &CellMap) -> Self {
        let min_max_tuple = |result| match result {
            MinMaxResult::NoElements => (0, 0),
            MinMaxResult::OneElement(n) => (n, n),
            MinMaxResult::MinMax(min, max) => (min, max),
        };
        let min_max_x = min_max_tuple(cells.keys().map(|pos| pos.x).minmax());
        let min_max_y = min_max_tuple(cells.keys().map(|pos| pos.y).minmax());
        let min = Pos::new(min_max_x.0, min_max_y.0);
        let max = Pos::new(min_max_x.1, min_max_y.1);
        Bounds::new(min, max)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Pos { x, y }
    }

    pub fn expand_min(self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y - 1,
        }
    }

    pub fn expand_max(self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.y.cmp(&other.y) {
            std::cmp::Ordering::Equal => self.x.cmp(&other.x),
            cmp => cmp,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Cell {
    Lit,
    Dark,
}

impl Cell {
    /// Returns `true` if the cell is [`Lit`].
    ///
    /// [`Lit`]: Cell::Lit
    pub fn is_lit(&self) -> bool {
        matches!(self, Self::Lit)
    }

    pub fn as_char(&self) -> char {
        match self {
            Cell::Lit => '#',
            Cell::Dark => '.',
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        if c == '#' {
            Self::Lit
        } else {
            Self::Dark
        }
    }
}

pub struct Image {
    cells: CellMap,
    bounds: Bounds,
    rest: Cell,
}

impl Image {
    pub fn indices(&self) -> impl Iterator<Item = Pos> + '_ {
        let bounds = self.bounds.expand();
        bounds
            .vertical_range()
            .flat_map(move |y| bounds.horizontal_range().map(move |x| Pos::new(x, y)))
    }

    pub fn get_cell(&self, pos: Pos) -> Cell {
        *self
            .cells
            .get(&pos)
            .unwrap_or(&if self.bounds.contains(pos) {
                Cell::Dark
            } else {
                self.rest
            })
    }

    pub fn get_cell_value(&self, pos: Pos) -> usize {
        usize::from_str_radix(
            &self
                .get_neighbors(pos)
                .iter()
                .map(|cell| match cell {
                    Cell::Lit => '1',
                    Cell::Dark => '0',
                })
                .collect::<String>(),
            2,
        )
        .unwrap()
    }

    pub fn get_neighbors(&self, pos: Pos) -> [Cell; 9] {
        let bounds = Bounds::new(pos, pos).expand();
        bounds
            .vertical_range()
            .flat_map(move |y| {
                bounds
                    .horizontal_range()
                    .map(move |x| self.get_cell(Pos::new(x, y)))
            })
            .collect_vec()
            .try_into()
            .expect("Length will always be 9")
    }

    pub fn step(&mut self, algo: &Algorithm) {
        self.cells = self
            .indices()
            .filter_map(|pos| {
                let cell_value = self.get_cell_value(pos);
                let new_cell = algo[cell_value];
                new_cell.is_lit().then(|| (pos, new_cell))
            })
            .collect();

        self.rest = match self.rest {
            Cell::Dark => algo[0],
            Cell::Lit => *algo.last().unwrap(),
        };
        self.bounds = Bounds::from(&self.cells);
    }
}

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.bounds.vertical_range() {
            for x in self.bounds.horizontal_range() {
                write!(f, "{}", self.get_cell(Pos::new(x, y)).as_char())?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl FromIterator<SpatialCell> for Image {
    fn from_iter<I: IntoIterator<Item = SpatialCell>>(iter: I) -> Self {
        let cells = HashMap::from_iter(iter);
        Self {
            bounds: Bounds::from(&cells),
            cells,
            rest: Cell::Dark,
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let (algo, mut image) = parse_input(input);
    for _ in 0..2 {
        image.step(&algo);
    }
    image.cells.into_values().filter(Cell::is_lit).count()
}

pub fn part_two(input: &'static str) -> usize {
    let (algo, mut image) = parse_input(input);
    for _ in 0..50 {
        image.step(&algo);
    }
    image.cells.into_values().filter(Cell::is_lit).count()
}

fn parse_input(input: &'static str) -> (Algorithm, Image) {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo: Vec<Cell> = algo.chars().map(Cell::from).collect();
    let image = image
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Pos::new(x as isize, y as isize), Cell::from(c)))
        })
        .collect();

    (algo, image)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 3351);
    }
}
