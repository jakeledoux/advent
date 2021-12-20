use itertools::Itertools;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SnailfishError {
    #[error("invalid token `{0}`")]
    ParseError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
pub enum Token {
    OpenBracket,
    CloseBracket,
    Comma,
    Number(usize),
}

impl Token {
    pub fn depth_offset(&self) -> i32 {
        match self {
            Self::OpenBracket => 1,
            Self::CloseBracket => -1,
            _ => 0,
        }
    }

    /// Returns `true` if the token is [`Number`].
    ///
    /// [`Number`]: Token::Number
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }

    /// Returns `true` if the token is [`Comma`].
    ///
    /// [`Comma`]: Token::Comma
    pub fn is_comma(&self) -> bool {
        matches!(self, Self::Comma)
    }
}

impl<'a> TryFrom<&str> for Token {
    type Error = SnailfishError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "[" => Self::OpenBracket,
            "]" => Self::CloseBracket,
            "," => Self::Comma,
            n if is_numeric(n) => Self::Number(n.parse().unwrap()),
            s => return Err(SnailfishError::ParseError(s.to_string())),
        })
    }
}

impl From<&Token> for String {
    fn from(token: &Token) -> Self {
        match token {
            Token::OpenBracket => "[".to_string(),
            Token::CloseBracket => "]".to_string(),
            Token::Comma => ",".to_string(),
            Token::Number(n) => n.to_string(),
        }
    }
}

pub fn is_numeric(value: &str) -> bool {
    value.chars().all(|c| c.is_digit(10))
}

pub fn tokens_to_string(tokens: &[Token]) -> String {
    tokens.iter().map(String::from).collect()
}

pub fn parse_snailfish(input: &str) -> Result<Vec<Token>, SnailfishError> {
    let mut buf = String::new();
    let mut tokens = Vec::new();
    for char in input.chars() {
        if char.is_digit(10) {
            buf.push(char);
        } else {
            if !buf.is_empty() {
                tokens.push(Token::try_from(buf.as_str())?);
                buf.clear();
            }
            tokens.push(Token::try_from(char.to_string().as_str())?);
        }
    }
    Ok(tokens)
}

pub fn explode(tokens: &[Token], recurse: bool) -> Vec<Token> {
    println!("Exploding: {}", tokens_to_string(&tokens));
    let mut tokens = tokens.to_vec();
    let mut depth = 0;
    let mut pos = 0;

    loop {
        let token = tokens[pos];
        depth += token.depth_offset();

        // Found pair to explode
        if depth == 5 {
            // Remove pair
            if let (
                _open_bracket,
                Token::Number(left),
                _comma,
                Token::Number(right),
                _close_bracket,
            ) = (
                tokens.remove(pos),
                tokens.remove(pos),
                tokens.remove(pos),
                tokens.remove(pos),
                tokens.remove(pos),
            ) {
                // Rise one level
                depth -= 1;

                // Insert 0 where pair was
                tokens.insert(pos, Token::Number(0));

                // Add left
                {
                    let mut pos = pos - 1;
                    loop {
                        let token = tokens[pos];
                        if let Token::Number(n) = token {
                            tokens[pos] = Token::Number(n + left);
                            break;
                        }
                        if pos == 0 {
                            break;
                        } else {
                            pos -= 1;
                        }
                    }
                }
                // Add Right
                {
                    let mut pos = pos + 1;
                    while pos < tokens.len() {
                        let token = tokens[pos];
                        if let Token::Number(n) = token {
                            tokens[pos] = Token::Number(n + right);
                            break;
                        }
                        pos += 1;
                    }
                }

                if recurse {
                    tokens = explode(&tokens, recurse);
                } else {
                    return tokens;
                }
            } else {
                unreachable!()
            }
        }

        pos += 1;
        if pos == tokens.len() {
            return tokens;
        }
    }
}

pub fn split(tokens: &[Token], recurse: bool) -> Vec<Token> {
    println!("Splitting: {}", tokens_to_string(&tokens));
    let mut tokens = tokens.to_vec();
    let mut pos = 0;

    loop {
        let token = tokens[pos];

        if let Token::Number(n) = token {
            // Found number to split
            if n >= 10 {
                let (floor, ceil) = (
                    (n as f64 / 2.0).floor() as usize,
                    (n as f64 / 2.0).ceil() as usize,
                );
                tokens.remove(pos);
                let (before, after) = tokens.split_at(pos);
                let new_pair = [
                    Token::OpenBracket,
                    Token::Number(floor),
                    Token::Comma,
                    Token::Number(ceil),
                    Token::CloseBracket,
                ];

                tokens = before
                    .into_iter()
                    .chain(&new_pair)
                    .chain(after.into_iter())
                    .copied()
                    .collect();

                tokens = explode(&tokens, true);

                if recurse {
                    return split(&tokens, recurse);
                } else {
                    return tokens;
                }
            }
        }

        pos += 1;
        if pos == tokens.len() {
            return tokens;
        }
    }
}

pub fn reduce(tokens: &[Token]) -> Vec<Token> {
    let mut tokens = tokens.to_vec();
    let mut previous = Vec::new();
    while tokens != previous {
        previous = tokens.clone();

        tokens = explode(&tokens, true);
        tokens = split(&tokens, true);
    }

    tokens
}

pub fn add(a: &[Token], b: &[Token]) -> Vec<Token> {
    let mut sum = vec![Token::OpenBracket];
    sum.extend(a);
    sum.push(Token::Comma);
    sum.extend(b);
    sum.push(Token::CloseBracket);
    reduce(&sum)
}

pub fn magnitude(tokens: &[Token]) -> usize {
    if let &[Token::Number(n)] = tokens {
        return n;
    } else {
        let mut depth = 0;
        for (pos, token) in tokens.iter().enumerate() {
            depth += token.depth_offset();
            if token.is_comma() && depth == 1 {
                let left = &tokens[1..pos];
                let right = &tokens[pos + 1..tokens.len() - 1];

                return magnitude(left) * 3 + magnitude(right) * 2;
            }
        }
        unreachable!()
    }
}

pub fn part_one(input: &'static str) -> usize {
    let input = parse_input(input);
    let sum = input.into_iter().reduce(|a, b| add(&a, &b)).unwrap();
    magnitude(&sum)
}

pub fn part_two(input: &'static str) -> usize {
    let input = parse_input(input);
    input
        .into_iter()
        .tuple_combinations()
        .flat_map(|(a, b)| [magnitude(&add(&a, &b)), magnitude(&add(&b, &a))])
        .max()
        .unwrap()
}

pub fn parse_input(input: &'static str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|line| parse_snailfish(line).unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::{
        add, explode, magnitude, parse_snailfish, part_one, part_two, reduce, split,
        tokens_to_string,
    };

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_parse() {
        for case in [
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
            "[[1,2],[[3,4],5]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        ] {
            assert!(parse_snailfish(case).is_ok())
        }
    }

    #[test]
    fn test_explode() {
        for (case, expected) in [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            ("[[3[[[10,10],0],0]],0]", "[[13[[0,10],0]],0]"),
            ("[[[[0,[23,40]],0],0],0]", "[[[[23,0],40],0],0]"),
            ("[[[[0,[0,40]],0],0],0]", "[[[[0,0],40],0],0]"),
            ("[[[[0,[30,0]],0],0],0]", "[[[[30,0],0],0],0]"),
            (
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[[[10,10],20],40],[[11,9],[11,0]]]]",
                "[[[[4,0],[5,4]],[[7,7],[6,10]]],[[[0,30],40],[[11,9],[11,0]]]]",
            ),
        ] {
            let tokens = parse_snailfish(case).unwrap();
            assert_eq!(tokens_to_string(&explode(&tokens, false)), expected)
        }
    }

    #[test]
    fn test_split() {
        for (case, expected) in [("[50,1]", "[[25,25],1]"), ("[0,100]", "[0,[50,50]]")] {
            let tokens = parse_snailfish(case).unwrap();
            assert_eq!(tokens_to_string(&split(&tokens, false)), expected)
        }
    }

    #[test]
    fn test_reduce() {
        for (case, expected) in [(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        )] {
            let tokens = parse_snailfish(case).unwrap();
            assert_eq!(tokens_to_string(&reduce(&tokens)), expected)
        }
    }

    #[test]
    fn test_add() {
        for (case, expected) in [
            (
                vec!["[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"],
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
            ),
            (
                vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
            ),
            (
                vec!["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
            ),
            (
                vec![
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                ],
                "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]",
            ),
            (
                vec![
                    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
                    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
                    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
                    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
                    "[7,[5,[[3,8],[1,4]]]]",
                    "[[2,[2,2]],[8,[8,1]]]",
                    "[2,9]",
                    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
                    "[[[5,[7,4]],7],1]",
                    "[[[[4,2],2],6],[8,7]]",
                ],
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
            ),
            (
                vec![
                    "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
                    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
                ],
                "[[[[7,8],[6,6]],[[6,0],[7,7]]],[[[7,8],[8,8]],[[7,9],[0,6]]]]",
            ),
        ] {
            let case = case
                .into_iter()
                .map(|case| parse_snailfish(case).unwrap())
                .collect_vec();
            assert_eq!(
                tokens_to_string(&case.into_iter().reduce(|a, b| add(&a, &b)).unwrap()),
                expected
            )
        }
    }

    #[test]
    fn test_magnitude() {
        for (case, expected) in [
            (
                "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
                4140,
            ),
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                3488,
            ),
        ] {
            let case = parse_snailfish(case).unwrap();
            assert_eq!(magnitude(&case), expected);
        }
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 4140);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 3993);
    }
}
