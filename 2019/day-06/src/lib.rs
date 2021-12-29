use std::collections::HashMap;

type OrbitMap = HashMap<&'static str, &'static str>;

#[derive(Default, Clone, Copy)]
struct OrbitCount {
    direct: usize,
    indirect: usize,
}

impl OrbitCount {
    pub fn total(&self) -> usize {
        self.direct + self.indirect
    }
}

fn count_orbits(orbits: &OrbitMap) -> OrbitCount {
    let mut count = OrbitCount::default();
    for mut parent in orbits.values() {
        count.direct += 1;
        while let Some(grandparent) = orbits.get(parent) {
            count.indirect += 1;
            parent = grandparent;
        }
    }
    count
}

fn get_address(orbits: &OrbitMap, start: &str) -> Vec<&'static str> {
    let mut address = Vec::new();
    if let Some(&parent) = orbits.get(start) {
        address.push(parent);
        address.extend(get_address(orbits, parent));
    }
    address
}

fn plot_transfer(orbits: &OrbitMap) -> Vec<&'static str> {
    let mut start_address = get_address(orbits, "YOU");
    let mut end_address = get_address(orbits, "SAN");
    while start_address.last() == end_address.last() {
        start_address.pop();
        end_address.pop();
    }
    start_address.extend(end_address.iter().rev());
    start_address
}

pub fn part_one(input: &'static str) -> usize {
    let orbits = parse_input(input);
    count_orbits(&orbits).total()
}

pub fn part_two(input: &'static str) -> usize {
    let orbits = parse_input(input);
    plot_transfer(&orbits).len()
}

fn parse_input(input: &'static str) -> OrbitMap {
    input
        .lines()
        .map(|line| {
            let (parent, child) = line.split_once(')').unwrap();
            (child, parent)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE_1: &str = include_str!("../sample_1.txt");
    const SAMPLE_2: &str = include_str!("../sample_2.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE_1), 42);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE_2), 4);
    }
}
