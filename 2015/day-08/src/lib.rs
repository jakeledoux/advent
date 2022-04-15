#[derive(Debug)]
enum State {
    Closed,
    Open,
    Escaped,
    Hex(usize),
}

fn count_code_chars(s: &str) -> usize {
    s.chars().count()
}

fn count_memory_chars(s: &str) -> Result<usize, &'static str> {
    let mut total = 0;
    let mut state = State::Closed;

    // println!("new str: {s}\n");

    for c in s.chars() {
        // println!("\tstate: {state:?}\n\ttotal: {total}\n\tnext: {c}\n");
        match state {
            State::Closed => {
                if c == '"' {
                    state = State::Open;
                } else {
                    return Err("string can only open with double quotes");
                }
            }
            State::Open => match c {
                '\\' => state = State::Escaped,
                '"' => state = State::Closed,
                _ => total += 1,
            },
            State::Escaped => match c {
                '\\' | '"' => {
                    total += 1;
                    state = State::Open;
                }
                'x' => state = State::Hex(0),
                _ => return Err("invalid escape sequence"),
            },
            State::Hex(mut i) => {
                if c.is_ascii_hexdigit() {
                    i += 1;
                    // maximum of 2 hex characters
                    if i >= 2 {
                        total += 1;
                        state = State::Open;
                    } else {
                        state = State::Hex(i);
                    }
                } else {
                    return Err("invalid escape sequence");
                }
            }
        }
    }

    Ok(total)
}

fn escape(s: &str) -> String {
    format!("{s:?}")
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    let code_chars: usize = input.iter().map(|line| count_code_chars(&line)).sum();
    let memory_chars: usize = input
        .iter()
        .map(|line| count_memory_chars(&line).unwrap())
        .sum();
    code_chars - memory_chars
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    let code_chars: usize = input.iter().map(|line| count_code_chars(&line)).sum();
    let escaped_chars: usize = input
        .iter()
        .map(|line| count_code_chars(&escape(line)))
        .sum();
    escaped_chars - code_chars
}

fn parse_input(input: &'static str) -> Vec<&'static str> {
    input
        .lines()
        .filter_map(|s| match s.trim() {
            "" => None,
            _ => Some(s.trim()),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{escape, part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 12);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 19);
    }

    #[test]
    fn test_escape() {
        for (input, output) in [
            (r#""""#, r#""\"\"""#),
            (r#""abc""#, r#""\"abc\"""#),
            (r#""aaa\"aaa""#, r#""\"aaa\\\"aaa\"""#),
            (r#""\x27""#, r#""\"\\x27\"""#),
        ] {
            println!("{}\n{}\n{}\n", input, escape(input), output);
            assert_eq!(escape(input), output);
        }
    }
}
