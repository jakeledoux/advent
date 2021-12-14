fn load_generation(fish: &[usize]) -> [usize; 9] {
    fish.iter().fold([0; 9], |mut gen, &f| {
        gen[f] += 1;
        gen
    })
}

fn advance_generation(gen: &[usize; 9]) -> [usize; 9] {
    let mut new_gen = *gen;
    new_gen.rotate_left(1);
    new_gen[6] += gen[0];
    new_gen
}

fn simulate_population(fish: &[usize], generations: usize) -> usize {
    (0..generations)
        .fold(load_generation(fish), |gen, _| advance_generation(&gen))
        .into_iter()
        .sum()
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    simulate_population(&input, 80)
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    simulate_population(&input, 256)
}

fn parse_input(input: &'static str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 5934);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 26984457539);
    }
}
