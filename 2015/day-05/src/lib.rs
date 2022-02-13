use itertools::Itertools;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
const BLACKLIST: [&str; 4] = ["ab", "cd", "pq", "xy"];

pub fn count_vowels(s: &str) -> usize {
    s.chars().filter(|c| VOWELS.contains(c)).count()
}

pub fn longest_run(s: &str) -> usize {
    s.chars()
        .tuple_windows()
        .fold((1, 1), |(mut current_run, longest_run), (a, b)| {
            if a == b {
                current_run += 1;
            } else {
                current_run = 1;
            }
            (current_run, longest_run.max(current_run))
        })
        .1
}

pub fn excludes(s: &str, blacklist: &[&str]) -> bool {
    blacklist.iter().all(|e| !s.contains(e))
}

fn has_double_letter(s: &str) -> bool {
    s.chars()
        .tuple_windows()
        .map(|(a, b)| format!("{}{}", a, b))
        .any(|pair| {
            s.match_indices(&pair)
                .combinations(2)
                .any(|matches| matches[0].0 > matches[1].0 + 1 || matches[0].0 + 1 < matches[1].0)
        })
}

fn has_double_pairs(s: &str) -> bool {
    s.chars().tuple_windows().any(|(a, _, c)| a == c)
}

pub fn is_nice(s: &str) -> bool {
    count_vowels(s) >= 3 && longest_run(s) >= 2 && excludes(s, &BLACKLIST)
}

pub fn is_nice_revised(s: &str) -> bool {
    has_double_pairs(s) && has_double_letter(s)
}
pub fn part_one(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|line| is_nice(line))
        .count()
}

pub fn part_two(input: &'static str) -> usize {
    parse_input(input)
        .into_iter()
        .filter(|line| is_nice_revised(line))
        .count()
}

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input.trim().lines().map(str::trim).collect()
}

#[cfg(test)]
mod tests {
    use super::{is_nice, is_nice_revised};

    #[test]
    fn test_is_nice() {
        assert_eq!(is_nice("ugknbfddgicrmopn"), true);
        assert_eq!(is_nice("aaa"), true);
        assert_eq!(is_nice("jchzalrnumimnmhp"), false);
        assert_eq!(is_nice("haegwjzuvuyypxyu"), false);
        assert_eq!(is_nice("dvszwmarrgswjxmb"), false);
    }

    #[test]
    fn test_is_nice_revised() {
        assert_eq!(is_nice_revised("qjhvhtzxzqqjkmpb"), true);
        assert_eq!(is_nice_revised("xxyxx"), true);
        assert_eq!(is_nice_revised("uurcxstgmygtbstg"), false);
        assert_eq!(is_nice_revised("ieodomkazucvgmuy"), false);
    }
}
