use petgraph::{algo, prelude::*};

type Pos = (usize, usize);
type Graph = GraphMap<Pos, (), Directed>;

#[derive(Debug)]
struct Matrix {
    width: usize,
    height: usize,
    items: Vec<Tile>,
}

impl Matrix {
    /// Returns `Tile` located at given coordinates `Pos`
    #[must_use]
    pub fn get(&self, pos: Pos) -> Option<Tile> {
        let (x, y) = pos;

        // pos out of bounds
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(self.items[y * self.width + x])
    }

    /// Returns adjacent neighbors of the tile located at `Pos`
    #[must_use]
    pub fn orthagonals(&self, pos: Pos) -> Option<Vec<(Pos, Tile)>> {
        let (x, y) = pos;

        // assert `pos` in bounds
        self.get(pos)?;

        Some(
            [
                (x, y.wrapping_sub(1)), // up
                (x, y + 1),             // down
                (x.wrapping_sub(1), y), // left
                (x + 1, y),             // right
            ]
            .into_iter()
            .filter_map(|pos| self.get(pos).map(|tile| (pos, tile)))
            .collect(),
        )
    }

    /// Returns an iterator over all valid (x, y) coordinates in matrix
    pub fn coords(&self) -> impl Iterator<Item = Pos> {
        let width = self.width;
        (0..self.height).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }

    /// Find coordinates of first tile matching the predicate
    #[must_use]
    pub fn find<P: Fn(Tile) -> bool>(&self, predicate: P) -> Option<Pos> {
        self.coords()
            .find(|pos| self.get(*pos).map(&predicate).unwrap_or_default())
    }

    /// Returns coordinates of the start tile
    #[must_use]
    pub fn start_pos(&self) -> Pos {
        self.find(|tile| tile.is_start())
            .expect("there is always a start node")
    }

    /// Returns coordinates of the goal tile
    #[must_use]
    pub fn goal_pos(&self) -> Pos {
        self.find(|tile| tile.is_goal())
            .expect("there is always a goal node")
    }

    /// Returns a graph representation of this matrix
    #[must_use]
    pub fn to_graph(&self) -> Graph {
        let mut graph = Graph::new();

        // instantiate all nodes
        self.coords().for_each(|pos| {
            graph.add_node(pos);
        });

        // connect edges
        self.coords()
            .map(|pos| (pos, self.get(pos).unwrap()))
            .for_each(|(pos, tile)| {
                self.orthagonals(pos)
                    .unwrap()
                    .into_iter()
                    .filter(|(_, other_tile)| tile.can_move_to(other_tile))
                    .for_each(|(other_pos, _)| {
                        graph.add_edge(pos, other_pos, ());
                    })
            });

        graph
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Start,
    Goal,
    Normal(usize),
}

impl Tile {
    #[must_use]
    fn height(&self) -> usize {
        match self {
            Tile::Start => 0,
            Tile::Goal => 25,
            Tile::Normal(height) => *height,
        }
    }

    #[must_use]
    fn can_move_to(&self, other_tile: &Tile) -> bool {
        self.height() + 1 >= other_tile.height()
    }

    #[must_use]
    fn is_start(&self) -> bool {
        matches!(self, Self::Start)
    }

    #[must_use]
    fn is_goal(&self) -> bool {
        matches!(self, Self::Goal)
    }

    #[must_use]
    fn is_a(&self) -> bool {
        matches!(self, Self::Normal(0))
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            'S' => Tile::Start,
            'E' => Tile::Goal,
            c => Tile::Normal(c as usize - 97),
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let matrix = parse_input(input);
    let graph = matrix.to_graph();
    let (start, goal) = (matrix.start_pos(), matrix.goal_pos());

    let paths = algo::dijkstra(&graph, start, Some(goal), |_| 1);
    paths[&goal]
}

pub fn part_two(input: &'static str) -> usize {
    let matrix = parse_input(input);
    let graph = matrix.to_graph();
    let goal = matrix.goal_pos();

    // there's a ton of duplicated pathfinding here that could be eliminated to speed this up, but
    // it's fast enough and got the answer so I can't be bothered right now.
    matrix
        .coords()
        .filter(|&pos| matrix.get(pos).map(|tile| tile.is_a()).unwrap_or_default())
        .filter_map(|start| algo::dijkstra(&graph, start, Some(goal), |_| 1).remove(&goal))
        .min()
        .unwrap()
}

fn parse_input(input: &'static str) -> Matrix {
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    let items = input
        .lines()
        .flat_map(|s| s.chars().map(|c| c.into()))
        .collect();

    Matrix {
        width,
        height,
        items,
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 31);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 29);
    }
}
