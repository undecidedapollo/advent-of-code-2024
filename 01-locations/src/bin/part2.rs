use core::num;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in lines {
        let line = line?;
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        list1.push(u32::from_str_radix(a, 10).unwrap());
        list2.push(u32::from_str_radix(b, 10).unwrap());
    }
    println!("List 1: {:?}", list1.len());
    println!("List 2: {:?}", list2.len());

    let mut similarity_score = 0;

    for num_1 in list1.iter() {
        let mut similar = 0;
        for j in list2.iter() {
            if num_1 == j {
                similar += 1;
            }
        }
        println!("Similarity for {}: {}", num_1, similar);
        similarity_score += num_1 * similar;
    }

    println!("Total Similarity: {}", similarity_score);
    Ok(())
}
