use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let lines: std::io::Lines<BufReader<File>> = BufReader::new(file).lines();
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

    let mut total_distance = 0;

    loop {
        if (list1.len() == 0) || (list2.len() == 0) {
            break;
        }
        let (first_lowest, first_lowest_idx) = get_lowest_number_idx(&list1);
        let (second_lowest, second_lowest_idx) = get_lowest_number_idx(&list2);
        total_distance += first_lowest.abs_diff(second_lowest);
        println!(
            "First Lowest: {}, Second Lowest: {} Distance: {}",
            first_lowest,
            second_lowest,
            first_lowest.abs_diff(second_lowest)
        );
        list1.remove(first_lowest_idx);
        list2.remove(second_lowest_idx);
    }

    println!("Total Distance: {}", total_distance);
    Ok(())
}

fn get_lowest_number_idx(list: &Vec<u32>) -> (u32, usize) {
    let mut lowest = list[0];
    let mut lowest_idx = 0;
    for i in 1..list.len() {
        if list[i] < lowest {
            lowest = list[i];
            lowest_idx = i;
        }
    }

    (lowest, lowest_idx)
}
