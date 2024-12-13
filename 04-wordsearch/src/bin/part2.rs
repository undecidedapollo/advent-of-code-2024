use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use wordsearch_04::{get_word_in_direction, Directions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let graph = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut times_word_found = 0;
    for x in 0..graph.len() {
        for y in 0..graph[x].len() {
            let char = graph[x][y];
            if char != 'A' || x == 0 || y == 0 {
                continue;
            }
            let Some(word1) = get_word_in_direction(&graph, x - 1, y - 1, Directions::DownRight, 3)
            else {
                continue;
            };
            let Some(word2) = get_word_in_direction(&graph, x + 1, y - 1, Directions::UpRight, 3)
            else {
                continue;
            };
            // println!("[{},{}] {:?} {:?}", x, y, word1, word2);
            if (word1 == "MAS" || word1 == "SAM") && (word2 == "MAS" || word2 == "SAM") {
                times_word_found += 1;
            }
        }
    }
    println!("Word found {} times", times_word_found);
    Ok(())
}

// Go char by char until you hit an x, when you do check all directions for matching the phrase xmas
