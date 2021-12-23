use std::collections::HashMap;

use cached::proc_macro::cached;

const ADDITIONAL_ROWS: [(Amphipod, Amphipod); 4] = [
    (Amphipod::Desert, Amphipod::Desert),
    (Amphipod::Copper, Amphipod::Bronze),
    (Amphipod::Bronze, Amphipod::Amber),
    (Amphipod::Amber, Amphipod::Copper),
];

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    pub fn room_index(&self) -> usize {
        match self {
            Amphipod::Amber => 0,
            Amphipod::Bronze => 1,
            Amphipod::Copper => 2,
            Amphipod::Desert => 3,
        }
    }
    pub fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

impl std::fmt::Debug for Amphipod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amber => write!(f, "A"),
            Self::Bronze => write!(f, "B"),
            Self::Copper => write!(f, "C"),
            Self::Desert => write!(f, "D"),
        }
    }
}

impl From<&str> for Amphipod {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Amber,
            "B" => Self::Bronze,
            "C" => Self::Copper,
            "D" => Self::Desert,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
    capacity: usize,
}

impl Burrow {
    pub fn new(capacity: usize) -> Self {
        Self {
            hallway: Hallway::default(),
            capacity,
            rooms: [
                Room {
                    kind: Amphipod::Amber,
                    capacity,
                    ..Default::default()
                },
                Room {
                    kind: Amphipod::Bronze,
                    capacity,
                    ..Default::default()
                },
                Room {
                    kind: Amphipod::Copper,
                    capacity,
                    ..Default::default()
                },
                Room {
                    kind: Amphipod::Desert,
                    capacity,
                    ..Default::default()
                },
            ],
        }
    }

    pub fn is_finished(&self) -> bool {
        self.hallway.is_empty() && self.rooms.iter().all(|room| room.is_complete())
    }

    pub fn moves(&self) -> Vec<(Self, usize)> {
        // Hallway -> Room moves
        let mut moves: Vec<_> = self
            .hallway
            .amphipod_locations()
            .into_iter()
            .filter_map(|(pos, amphipod)| {
                let room_index = amphipod.room_index();

                if self.rooms[room_index].can_enter(amphipod) {
                    let door_pos = (room_index + 1) * 2;
                    if self.hallway.can_travel(pos, door_pos) {
                        let mut burrow = *self;
                        let mut energy_spent = burrow.hallway.move_out(pos, door_pos).unwrap();
                        energy_spent += burrow.rooms[room_index].push(amphipod).unwrap();
                        energy_spent *= amphipod.energy();
                        return Some((burrow, energy_spent));
                    }
                }
                None
            })
            .collect();

        // Skip if any amphipods can be moved into their final room
        if moves.is_empty() {
            // Room -> Hallway moves
            moves.extend(
                self.rooms
                    .iter()
                    .enumerate()
                    .filter(|(_i, room)| !room.is_complete() && !room.is_empty())
                    .flat_map(|(room_index, _room)| {
                        let door_pos = (room_index + 1) * 2;
                        // Iterate over valid resting places
                        (0..11)
                            .filter(|&n| n % 2 == 1 || n == 0 || n == 10)
                            .filter_map(move |pos| {
                                if self.hallway.can_travel(door_pos, pos) {
                                    let mut burrow = *self;
                                    let (amphipod, mut energy_spent) =
                                        burrow.rooms[room_index].pop().unwrap();
                                    energy_spent +=
                                        burrow.hallway.move_in(door_pos, pos, amphipod).unwrap();
                                    energy_spent *= amphipod.energy();
                                    Some((burrow, energy_spent))
                                } else {
                                    None
                                }
                            })
                    }),
            );
        }

        moves
    }
}

impl Default for Burrow {
    fn default() -> Self {
        Self::new(2)
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Room {
    kind: Amphipod,
    stack: [Option<Amphipod>; 4],
    capacity: usize,
}

impl Room {
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, amphipod: Amphipod) -> Result<usize, Amphipod> {
        if self.can_enter(amphipod) {
            let energy_spent = self.capacity - self.len();
            self.stack.rotate_right(1);
            self.stack[0] = Some(amphipod);
            Ok(energy_spent)
        } else {
            Err(amphipod)
        }
    }

    pub fn push_unchecked(&mut self, amphipod: Amphipod) -> Result<(), Amphipod> {
        if self.stack.iter().filter(|option| option.is_some()).count() < self.capacity {
            self.stack.rotate_right(1);
            self.stack[0] = Some(amphipod);
            Ok(())
        } else {
            Err(amphipod)
        }
    }

    pub fn pop(&mut self) -> Option<(Amphipod, usize)> {
        let energy_spent = self.capacity - self.len() + 1;
        let amphipod = self.stack[0].take();
        self.stack.rotate_left(1);
        amphipod.map(|amphipod| (amphipod, energy_spent))
    }

    pub fn peek(&mut self) -> Option<Amphipod> {
        self.stack[0]
    }

    pub fn can_enter(&self, amphipod: Amphipod) -> bool {
        let stack: Vec<Amphipod> = self.stack.iter().filter_map(|&option| option).collect();
        stack.len() < self.capacity
            && self.kind == amphipod
            && !stack.iter().any(|&other| other != amphipod)
    }

    pub fn is_complete(&self) -> bool {
        !self
            .stack
            .iter()
            .filter_map(|option| *option)
            .any(|amphipod| amphipod != self.kind)
    }

    pub fn len(&self) -> usize {
        self.stack.iter().copied().filter(Option::is_some).count()
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            kind: Amphipod::Amber,
            stack: [None; 4],
            capacity: 2,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Hallway {
    spaces: [Option<Amphipod>; 11],
}

impl Hallway {
    pub fn can_travel(&self, mut from: usize, to: usize) -> bool {
        from = (from as isize + (to as isize - from as isize).signum()) as usize;
        let path = from.min(to)..=from.max(to);
        self.spaces[path].iter().all(|space| space.is_none())
    }

    pub fn move_in(&mut self, from: usize, to: usize, amphipod: Amphipod) -> Option<usize> {
        if self.can_travel(from, to) && self.spaces[to].is_none() {
            self.spaces[to] = Some(amphipod);
            Some((from as isize - to as isize).abs() as usize)
        } else {
            None
        }
    }

    pub fn move_out(&mut self, from: usize, to: usize) -> Option<usize> {
        if self.can_travel(from, to) && self.spaces[from].take().is_some() {
            Some((from as isize - to as isize).abs() as usize)
        } else {
            None
        }
    }

    pub fn amphipod_locations(&self) -> Vec<(usize, Amphipod)> {
        self.spaces
            .iter()
            .enumerate()
            .filter_map(|(pos, amphipod)| amphipod.map(|amphipod| (pos, amphipod)))
            .collect()
    }

    fn is_empty(&self) -> bool {
        self.spaces.iter().all(|option| option.is_none())
    }
}

#[cached]
pub fn burrow_moves(burrow: Burrow) -> Vec<(Burrow, usize)> {
    burrow.moves()
}

pub fn cheapest_solution(burrow: Burrow) -> Option<usize> {
    let initial_state = burrow;

    let mut total_energy: HashMap<Burrow, usize> = HashMap::new();
    total_energy.insert(initial_state, 0);
    let mut best_parent: HashMap<Burrow, Burrow> = HashMap::new();
    let mut queue: Vec<Burrow> = vec![initial_state];

    while !queue.is_empty() {
        // Get most promising elem from queue
        let new_state = queue.pop().unwrap();

        // Found target
        if new_state.is_finished() {
            return Some(total_energy[&new_state]);
        }

        for (child_state, child_energy) in burrow_moves(new_state) {
            // Check if this path to `neighbor` is less dangerest than previous best
            let alt_total_energy = total_energy[&new_state] + child_energy;
            if alt_total_energy < *total_energy.get(&child_state).unwrap_or(&usize::MAX) {
                // Update best path
                total_energy.insert(child_state, alt_total_energy);
                best_parent.insert(child_state, new_state);

                // Queue neighbor for expansion (insert pre-sorted)
                let idx = queue
                    .binary_search_by(|other| alt_total_energy.cmp(&total_energy[other]))
                    .unwrap_or_else(|i| i);
                queue.insert(idx, child_state);
            }
        }
    }
    None
}

pub fn part_one(input: &'static str) -> usize {
    let burrow = parse_input(input, 2);
    cheapest_solution(burrow).unwrap()
}

pub fn part_two(input: &'static str) -> usize {
    let mut burrow = parse_input(input, 4);
    // Insert new rows
    for (i, room) in burrow.rooms.iter_mut().enumerate() {
        room.stack.swap(1, 3);
        room.stack[1] = Some(ADDITIONAL_ROWS[i].0);
        room.stack[2] = Some(ADDITIONAL_ROWS[i].1);
    }
    cheapest_solution(burrow).unwrap()
}

fn parse_input(input: &'static str, capacity: usize) -> Burrow {
    let room_pattern = regex::Regex::new(r#"#(\w)#(\w)#(\w)#(\w)#"#).unwrap();
    let mut burrow = Burrow::new(capacity);
    input.lines().rev().for_each(|line| {
        if let Some(captures) = room_pattern.captures(line) {
            let mut groups = captures
                .iter()
                .skip(1)
                .map(|group| Amphipod::from(group.unwrap().as_str()));
            for room in burrow.rooms.iter_mut() {
                (*room).push_unchecked(groups.next().unwrap()).unwrap()
            }
        }
    });
    burrow
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 12521);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 44169);
    }
}
