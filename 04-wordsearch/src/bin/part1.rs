use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use wordsearch_04::{get_word_in_direction, Directions};

static DIRECTIONS: [Directions; 8] = [
    Directions::Up,
    Directions::Down,
    Directions::Left,
    Directions::Right,
    Directions::UpLeft,
    Directions::UpRight,
    Directions::DownLeft,
    Directions::DownRight,
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let graph = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let word_to_find = "XMAS";
    let mut times_word_found = 0;
    for x in 0..graph.len() {
        for y in 0..graph[x].len() {
            for direction in DIRECTIONS {
                let word = get_word_in_direction(&graph, x, y, direction, word_to_find.len());
                if let Some(word) = word {
                    if word == word_to_find {
                        times_word_found += 1;
                    }
                }
            }
        }
    }
    println!("Word found {} times", times_word_found);
    Ok(())
}

// Go char by char until you hit an x, when you do check all directions for matching the phrase xmas
