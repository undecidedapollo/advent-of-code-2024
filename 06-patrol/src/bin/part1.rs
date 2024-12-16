#![feature(let_chains)]
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    hash::Hash,
    io::{BufReader, Read},
};

use updates_05::rolling::RollingArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn get_row_col_adjustment(&self) -> (i64, i64) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Pos {
    pub row: u32,
    pub col: u32,
}

#[derive(Debug)]
pub struct Entity {
    pub pos: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PosDir {
    pub pos: Pos,
    pub dir: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Person {
    pub pos: Pos,
    pub dir: Direction,
}

impl Person {
    fn next_pos(&self, map: &WorldMap) -> Option<Pos> {
        let (row_adj, col_adj) = self.dir.get_row_col_adjustment();
        let next_row: i64 = <i64>::from(self.pos.row) + row_adj;
        let next_col: i64 = <i64>::from(self.pos.col) + col_adj;
        if next_row < 0
            || (next_row >= map.height.into())
            || next_col < 0
            || (next_col >= map.width.into())
        {
            return None;
        }

        let new_pos = Pos {
            row: next_row.try_into().unwrap(),
            col: next_col.try_into().unwrap(),
        };

        return Some(new_pos);
    }
}

#[derive(Debug)]
pub struct WorldMap {
    pub width: u32,
    pub height: u32,
    // pub entities: Vec<Entity>,
    pub entities: HashMap<u32, HashMap<u32, Entity>>,
    pub people: Vec<Person>,
}

impl WorldMap {
    pub fn from_iter(iter: impl Iterator<Item = char>) -> Self {
        let mut world_map = WorldMap {
            width: 0,
            height: 0,
            entities: HashMap::new(),
            people: vec![],
        };
        let mut col = 0;
        let mut row = 0;
        for char in iter {
            match char {
                '#' => {
                    let row_map = world_map.entities.entry(row).or_default();
                    row_map.insert(
                        col,
                        Entity {
                            pos: Pos { col, row },
                        },
                    );
                }
                '^' | 'V' | '<' | '>' => world_map.people.push(Person {
                    pos: Pos { row, col },
                    dir: match char {
                        '^' => Direction::Up,
                        'V' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => unreachable!("Unexpected char: {}", char),
                    },
                }),
                '\n' => {
                    col = 0;
                    row += 1;
                }
                _ => {}
            }
            if char != '\n' {
                col += 1;
            }
        }

        world_map.width = col + 1;
        world_map.height = row + 1;

        return world_map;
    }

    pub fn get_entity_at_pos(&self, pos: Pos) -> Option<&Entity> {
        self.entities
            .get(&pos.row)
            .map(|x| x.get(&pos.col))
            .flatten()
    }
}

#[derive(Debug)]
struct PossibleLoop {
    starting_pos: Pos,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;

    let chars = BufReader::new(file)
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char);
    let world_map = WorldMap::from_iter(chars);

    assert!(world_map.people.len() == 1);
    let mut person = world_map.people.get(0).unwrap().clone();
    let mut visited_positions: HashSet<Pos> = HashSet::new();
    let mut seen: HashSet<PosDir> = HashSet::new();
    let mut loops_count = 0;
    let mut special_loops_count = 0;
    let mut max_iterations = 0;
    let starting_pos = person.pos;
    loop {
        visited_positions.insert(person.pos);

        let obstruction_pos = person.next_pos(&world_map);

        if let Some(obstruction_pos) = obstruction_pos
            && obstruction_pos != starting_pos
            && visited_positions.get(&obstruction_pos).is_none()
        {
            let mut temp_person = person.clone();
            temp_person.dir = temp_person.dir.turn_right();
            let mut num_iterations = 0;
            seen.clear();
            loop {
                num_iterations += 1;
                if num_iterations % 100000 == 0 {
                    println!("Iterations: {}", num_iterations);
                }
                let posdir = PosDir {
                    pos: temp_person.pos,
                    dir: temp_person.dir,
                };
                if temp_person == person {
                    loops_count += 1;
                    if num_iterations > max_iterations {
                        max_iterations = num_iterations;
                        println!("{}", max_iterations);
                    }
                    println!("Obstruction: {:?}", obstruction_pos);
                    break;
                }

                if let Some(_) = seen.get(&posdir) {
                    loops_count += 1;
                    special_loops_count += 1;
                    if num_iterations > max_iterations {
                        max_iterations = num_iterations;
                        println!("{}", max_iterations);
                    }
                    println!("Obstruction: {:?}", obstruction_pos);
                    break;
                }

                seen.insert(posdir);

                let Some(new_pos) = temp_person.next_pos(&world_map) else {
                    break;
                };
                if let Some(_) = world_map.get_entity_at_pos(new_pos) {
                    temp_person.dir = temp_person.dir.turn_right();
                    continue;
                };
                // This would occur if you were to hit the new obstacle
                if obstruction_pos == new_pos {
                    temp_person.dir = temp_person.dir.turn_right();
                    continue;
                };
                temp_person.pos = new_pos;
            }
        }

        // println!("{:?}", person.pos);
        let Some(new_pos) = person.next_pos(&world_map) else {
            break;
        };

        if let Some(_) = world_map.get_entity_at_pos(new_pos) {
            person.dir = person.dir.turn_right();
            continue;
        };
        person.pos = new_pos;
    }
    println!(
        "Visited Positions: {} Loops Count: {} Special Loops: {}",
        visited_positions.len(),
        loops_count,
        special_loops_count
    );

    Ok(())
}
// Every time you hit three, there is a fourth. Check the last 3 and their star part, if you come to it from the right dir, that is a place an obstacle could be.
// If not then no?
