pub fn part_one(input: &'static str) -> u32 {
    let input = parse_input(input);
    input.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

pub fn part_two(input: &'static str) -> u32 {
    let input = parse_input(input);
    let mut totals: Vec<u32> = input.iter().map(|elf| elf.iter().sum()).collect();
    totals.sort();
    totals.reverse();
    totals.iter().take(3).sum()
}

fn parse_input(input: &'static str) -> Vec<Vec<u32>> {
    let mut elves = Vec::new();
    let mut snacks = Vec::new();
    for line in input.lines() {
        if line.trim() == "" {
            elves.push(snacks);
            snacks = Vec::new();
            continue;
        }
        if let Ok(calories) = line.trim().parse() {
            snacks.push(calories);
        }
    }
    elves.push(snacks);

    elves
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 45000);
    }
}
