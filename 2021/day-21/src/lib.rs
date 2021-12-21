use std::{
    iter::{repeat_with, Cycle, Enumerate},
    ops::RangeInclusive,
};

use cached::proc_macro::cached;

pub enum Status {
    Playing,
    Won,
}

impl Status {
    /// Returns `true` if the status is [`Won`].
    ///
    /// [`Won`]: Status::Won
    pub fn is_won(&self) -> bool {
        matches!(self, Self::Won)
    }
}

pub struct DeterministicDie {
    iter: Enumerate<Cycle<RangeInclusive<usize>>>,
    rolls: usize,
}

impl DeterministicDie {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DeterministicDie {
            iter: (1..=100).cycle().enumerate(),
            rolls: 0,
        }
    }

    pub fn roll(&mut self) -> usize {
        let (total_rolls, roll) = self.iter.next().expect("iter is infinite");
        self.rolls = total_rolls + 1;
        roll
    }

    pub fn roll_sum(&mut self, count: usize) -> usize {
        repeat_with(|| self.roll()).take(count).sum()
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct Player {
    position: usize,
    score: usize,
    target_score: usize,
}

impl Player {
    pub fn advance(&mut self, roll: usize) -> Status {
        self.position = (self.position - 1 /* to allow modulo */ + roll) % 10 + 1 /* back to 1-10 8 */;
        self.score += self.position;
        if self.score >= self.target_score {
            Status::Won
        } else {
            Status::Playing
        }
    }
}

pub fn part_one(input: &'static str) -> usize {
    let mut players = parse_input(input);
    players
        .iter_mut()
        .for_each(|player| player.target_score = 1000);
    let mut die = DeterministicDie::new();

    loop {
        for (idx, player) in players.iter_mut().enumerate() {
            let roll = die.roll_sum(3);
            if player.advance(roll).is_won() {
                return die.rolls * players[(idx + 1) % players.len()].score;
            }
        }
    }
}

#[cached]
pub fn quantum_game(players: (Player, Player)) -> [usize; 2] {
    const ROLLS: [(usize, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

    let (player_one, player_two) = &players;
    let mut wins = [0, 0];

    // Possible player one dice rolls
    for (roll, freq) in ROLLS {
        let mut player_one = *player_one;

        if player_one.advance(roll).is_won() {
            wins[0] += freq;
            continue;
        }

        // Possible player two dice rolls
        for (roll, second_freq) in ROLLS {
            let mut player_two = *player_two;
            let freq = freq * second_freq;

            if player_two.advance(roll).is_won() {
                wins[1] += freq;
                continue;
            }

            // Recurse over remaining possibilities
            let further_wins = quantum_game((player_one, player_two));
            wins = [
                wins[0] + further_wins[0] * freq,
                wins[1] + further_wins[1] * freq,
            ];
        }
    }

    wins
}

pub fn part_two(input: &'static str) -> usize {
    let mut players = parse_input(input);
    players
        .iter_mut()
        .for_each(|player| player.target_score = 21);

    let wins = quantum_game((players[0], players[1]));
    wins.into_iter().max().unwrap()
}

fn parse_input(input: &'static str) -> Vec<Player> {
    input
        .lines()
        .map(|s| Player {
            position: s.chars().last().unwrap().to_digit(10).unwrap() as usize,
            score: 0,
            target_score: 0,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 739785);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 444356092776315);
    }
}
