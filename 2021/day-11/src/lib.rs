use cached::proc_macro::cached;

#[derive(Debug, Clone, Copy)]
enum Octopus {
    Unflashed(usize),
    Flashing,
    Flashed,
}

impl Octopus {
    pub fn new(n: usize) -> Self {
        if n > 9 {
            Self::Flashing
        } else {
            Self::Unflashed(n)
        }
    }

    pub fn to_usize(self) -> Result<usize, Self> {
        match self {
            Octopus::Unflashed(n) => Ok(n),
            Octopus::Flashed => Ok(0),
            Octopus::Flashing => Err(self),
        }
    }

    pub fn increment(self) -> Self {
        match self {
            Self::Unflashed(n) => Self::new(n + 1),
            _ => self,
        }
    }

    /// Returns `true` if the octopus is [`Flashing`].
    ///
    /// [`Flashing`]: Octopus::Flashing
    fn is_flashing(&self) -> bool {
        matches!(self, Self::Flashing)
    }
}

#[derive(Clone)]
struct Octopuses {
    cells: Vec<usize>,
    width: usize,
}

impl Octopuses {
    pub fn new(cells: &[Vec<usize>]) -> Self {
        Self {
            cells: cells.concat(),
            width: cells[0].len(),
        }
    }

    /// Returns number of flashes since last step
    pub fn step(&mut self) -> usize {
        let mut flashes = 0;

        // Increment
        let mut new_cells: Vec<Octopus> = self.cells.iter().map(|n| Octopus::new(n + 1)).collect();

        // Handle all flashes
        while new_cells.iter().any(Octopus::is_flashing) {
            for idx in new_cells
                .iter()
                .enumerate()
                .filter_map(|(idx, oct)| oct.is_flashing().then(|| idx))
                .collect::<Vec<_>>()
            {
                flashes += 1;
                new_cells[idx] = Octopus::Flashed;
                for (idx, cell) in get_neighbors(idx, &new_cells, self.width) {
                    new_cells[idx] = cell.increment();
                }
            }
        }

        // Reduce all octopuses back to usize
        let new_cells = new_cells
            .into_iter()
            .map(|o| o.to_usize().unwrap())
            .collect();
        self.cells = new_cells;

        flashes
    }
}

fn get_neighbors(idx: usize, cells: &[Octopus], width: usize) -> Vec<(usize, Octopus)> {
    let i_to_pos = |i| (i % width, idx / width);
    let pos_to_i = |x, y| x + y * width;

    let (x, y) = i_to_pos(idx);

    (-1..=1)
        .filter_map(|xo| {
            let x = x as isize + xo;
            if x >= 0 && x < width as isize {
                let x = x as usize;
                Some((-1..=1).filter_map(move |yo| {
                    let y = y as isize + yo;
                    if y >= 0 && y < (cells.len() / width) as isize {
                        let y = y as usize;
                        let idx = pos_to_i(x, y);
                        Some((idx, cells[idx]))
                    } else {
                        None
                    }
                }))
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn part_one(input: &'static str) -> usize {
    let mut octos = parse_input(input);
    (0..100).map(|_| octos.step()).sum()
}

pub fn part_two(input: &'static str) -> usize {
    let mut octos = parse_input(input);
    (1..).find(|_| octos.step() == octos.cells.len()).unwrap()
}

#[cached]
fn parse_input(input: &'static str) -> Octopuses {
    let input: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect()
        })
        .collect();
    Octopuses::new(&input)
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 1656);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 195);
    }
}
