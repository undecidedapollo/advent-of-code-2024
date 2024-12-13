use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

enum Instruction {
    Do,
    Dont,
    Mul(u64, u64),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)")?;
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .flat_map(|line| {
            re.captures_iter(&line)
                .map(|m| {
                    let (_, [a, b]) = m.extract();
                    let a = a.parse::<u64>().unwrap();
                    let b = b.parse::<u64>().unwrap();
                    a * b
                })
                .collect::<Vec<u64>>()
        });

    let output: u64 = lines.sum();
    println!("Output: {}", output);
    Ok(())
}
