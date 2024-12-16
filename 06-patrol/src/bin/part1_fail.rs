/**
 * Spent a long time trying to get clever before finally working through the example problems and realizing there were many possibilities that could cause a loop.
 * In this solution I was trying to come up with a way of using the last 3 to form a square but a square isn't the only kind of loop you can have.
 * Sometimes the loop can be longer than 4, or use different obstacles you wouldn't have used otherwise.
 */
use std::{
    collections::{HashMap, HashSet},
    fs::File,
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

#[derive(Debug, Clone, Copy)]
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
    pub entities: Vec<Entity>,
    pub people: Vec<Person>,
}

impl WorldMap {
    pub fn from_iter(iter: impl Iterator<Item = char>) -> Self {
        let mut world_map = WorldMap {
            width: 0,
            height: 0,
            entities: vec![],
            people: vec![],
        };
        let mut col = 0;
        let mut row = 0;
        for char in iter {
            match char {
                '#' => world_map.entities.push(Entity {
                    pos: Pos { col, row },
                }),
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
        for entity in self.entities.iter() {
            if entity.pos == pos {
                return Some(entity);
            }
        }
        None
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
    let mut possible_loops: HashMap<PosDir, PossibleLoop> = HashMap::new();
    let mut last_obstacles: RollingArray<PosDir> = RollingArray::new(3);
    let mut loops_count = 0;
    loop {
        visited_positions.insert(person.pos);

        if let Some(x) = possible_loops.get(&PosDir {
            pos: person.pos,
            dir: person.dir,
        }) {
            println!("Possible Matched Loop: {:?}", x);
            // Let's walk that direction to see if we would get there or not:
            let mut temp_person: Person = person.clone();
            temp_person.dir = temp_person.dir.turn_right();
            loop {
                let Some(new_pos) = temp_person.next_pos(&world_map) else {
                    println!("Failed because out of bounds");
                    break;
                };
                temp_person.pos = new_pos;

                if new_pos == x.starting_pos {
                    println!("Match found at: {:?}", person.pos);
                    loops_count += 1;
                    break;
                }

                if let Some(_) = world_map.get_entity_at_pos(new_pos) {
                    println!("Failed because entity at: {:?}", new_pos);
                    break;
                }
            }
        }

        println!("{:?}", person.pos);
        let Some(new_pos) = person.next_pos(&world_map) else {
            break;
        };

        if let Some(_) = world_map.get_entity_at_pos(new_pos) {
            last_obstacles.push(PosDir {
                pos: person.pos,
                dir: person.dir,
            });
            if last_obstacles.len() == 3 {
                // Do some magic
                let first_obstacle = last_obstacles.get(0).unwrap();
                let last_obstacle = last_obstacles.get(2).unwrap();

                let possible_obstacle =
                    if person.dir == Direction::Right || person.dir == Direction::Left {
                        PosDir {
                            pos: Pos {
                                row: first_obstacle.pos.row,
                                col: last_obstacle.pos.col,
                            },
                            dir: match person.dir {
                                Direction::Right => Direction::Down,
                                Direction::Left => Direction::Up,
                                _ => unreachable!("Unexpected direction: {:?}", person.dir),
                            },
                        }
                    } else {
                        PosDir {
                            pos: Pos {
                                row: last_obstacle.pos.row,
                                col: first_obstacle.pos.col,
                            },
                            dir: match person.dir {
                                Direction::Up => Direction::Right,
                                Direction::Down => Direction::Left,
                                _ => unreachable!("Unexpected direction: {:?}", person.dir),
                            },
                        }
                    };
                println!("Possible obstacle: {:?}", possible_obstacle);
                possible_loops.insert(
                    possible_obstacle,
                    PossibleLoop {
                        starting_pos: first_obstacle.pos.clone(),
                    },
                );
            }
            person.dir = person.dir.turn_right();
            continue;
        };
        person.pos = new_pos;
    }
    println!(
        "Visited Positions: {} Loops Count: {}",
        visited_positions.len(),
        loops_count
    );

    Ok(())
}
// Every time you hit three, there is a fourth. Check the last 3 and their star part, if you come to it from the right dir, that is a place an obstacle could be.
// If not then no?
