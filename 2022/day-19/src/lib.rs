#![allow(unused)]

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    rc::Rc,
};

use cached::proc_macro::cached;
use derive_more::{AddAssign, SubAssign};
use itertools::Itertools;
use regex::Regex;
use strum::EnumString;

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Resource {
    #[must_use]
    fn is_geode(&self) -> bool {
        matches!(self, Self::Geode)
    }
}

#[derive(
    Debug, Hash, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, AddAssign, SubAssign,
)]
struct Resources {
    geode: usize,
    obsidian: usize,
    clay: usize,
    ore: usize,
}

impl Resources {
    fn get(&self, resource: &Resource) -> usize {
        match resource {
            Resource::Ore => self.ore,
            Resource::Clay => self.clay,
            Resource::Obsidian => self.obsidian,
            Resource::Geode => self.geode,
        }
    }

    fn store(&mut self, resources: Resources) {
        *self += resources
    }

    fn can_afford(&self, resources: &Resources) -> bool {
        self.geode >= resources.geode
            && self.obsidian >= resources.obsidian
            && self.clay >= resources.clay
            && self.ore >= resources.ore
    }
}

impl From<HashMap<Resource, usize>> for Resources {
    fn from(map: HashMap<Resource, usize>) -> Self {
        map.into_iter().collect()
    }
}

impl FromIterator<(Resource, usize)> for Resources {
    fn from_iter<T: IntoIterator<Item = (Resource, usize)>>(iter: T) -> Self {
        let mut resources = Resources::default();
        iter.into_iter().for_each(|(resource, count)| {
            *match resource {
                Resource::Ore => &mut resources.ore,
                Resource::Clay => &mut resources.ore,
                Resource::Obsidian => &mut resources.obsidian,
                Resource::Geode => &mut resources.geode,
            } += count
        });

        resources
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Recipe {
    resources: Resources,
    robot: Robot,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Robot(Resource);

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Blueprint {
    recipes: Vec<Recipe>,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct State {
    blueprint: Blueprint,
    resources: Resources,
    robots: Vec<Robot>,
    time: usize,
}

impl State {
    pub fn new(blueprint: Blueprint) -> Self {
        Self {
            blueprint,
            resources: Default::default(),
            robots: vec![Robot(Resource::Ore)],
            time: Default::default(),
        }
    }

    pub fn can_utilize(&self, robot: Robot) -> bool {
        if robot.0.is_geode() {
            return true;
        }

        let max = self
            .blueprint
            .recipes
            .iter()
            .map(|recipe| recipe.resources.get(&robot.0))
            .max()
            .unwrap();
        !(*self
            .robots
            .iter()
            .map(|Robot(resource)| resource)
            .counts()
            .get(&robot.0)
            .unwrap_or(&0)
            == max)
    }
}

#[cached]
fn possibilities(mut state: State) -> Vec<State> {
    state.time += 1;

    // build new robots
    let new_robots = state
        .blueprint
        .recipes
        .iter()
        .rev()
        .filter(|recipe| {
            state.resources.can_afford(&recipe.resources) && state.can_utilize(recipe.robot)
        })
        .collect_vec();

    // mine resources
    let new_resources: Resources = state
        .robots
        .iter()
        .map(|Robot(resource)| *resource)
        .counts()
        .into();
    state.resources.store(new_resources);

    // deploy new robots
    let mut new_states = new_robots
        .into_iter()
        .map(|recipe| {
            let mut new_state = state.clone();
            new_state.resources -= recipe.resources;
            new_state.robots.push(recipe.robot);
            new_state
        })
        .collect_vec();
    new_states.push(state);
    new_states
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resources.cmp(&other.resources)
    }
}

pub fn part_one(input: &'static str) -> usize {
    let blueprints = parse_input(input);
    for blueprint in blueprints.into_iter().take(1) {
        let mut queue = BinaryHeap::from([State::new(blueprint)]);
        let mut completed = HashSet::new();

        loop {
            let Some(state) = queue.pop() else {
                break;
            };

            if state.time == 24 {
                completed.insert(state.resources.geode);
            } else {
                queue.extend(possibilities(state));
            }
        }
        println!(
            "{:?}",
            completed.iter().sorted().rev().take(10).collect_vec()
        );
    }
    1
}

pub fn part_two(input: &'static str) -> usize {
    let blueprints = parse_input(input);
    0
}

fn parse_input(input: &'static str) -> Vec<Blueprint> {
    let resource_pattern = Regex::new(r"(\d+) (ore|clay|obsidian)").unwrap();
    input
        .lines()
        .map(|s| {
            let recipes = s
                .split_once(": ")
                .unwrap()
                .1
                .split(". ")
                .zip([
                    Resource::Ore,
                    Resource::Clay,
                    Resource::Obsidian,
                    Resource::Geode,
                ])
                .map(|(s, robot_resource)| {
                    let resources = resource_pattern
                        .captures_iter(s)
                        .map(|m| {
                            let amount = m.get(1).unwrap().as_str().parse().unwrap();
                            let resource = m.get(2).unwrap().as_str().parse().unwrap();
                            (resource, amount)
                        })
                        .collect();
                    Recipe {
                        robot: Robot(robot_resource),
                        resources,
                    }
                })
                .collect_vec();
            Blueprint { recipes }
        })
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    const SAMPLE: &str = include_str!("../sample.txt");

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(SAMPLE), 0);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(SAMPLE), 0);
    }
}
