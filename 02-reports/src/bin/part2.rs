use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct DirDiff {
    dir: bool,
    diff: i32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;
    let lines = BufReader::new(file).lines();
    let valid_lines = lines.filter_map(|f| f.ok()).filter(|f| {
        let nums: Vec<i32> = f
            .split_whitespace()
            .map(|f| f.parse::<i32>().unwrap())
            .collect();

        for i in 0..nums.len() {
            let mut new_nums = nums.clone();
            new_nums.remove(i);
            let dir = new_nums[0] - new_nums[1] > 0;
            let mut is_valid = true;
            for i in 0..new_nums.len() - 1 {
                if dir {
                    let num1 = new_nums[i];
                    let num2 = new_nums[i + 1];
                    let diff = num1 - num2;
                    if diff != 1 && diff != 2 && diff != 3 {
                        is_valid = false;
                    }
                } else {
                    let num1 = new_nums[i];
                    let num2 = new_nums[i + 1];
                    let diff = num2 - num1;
                    if diff != 1 && diff != 2 && diff != 3 {
                        is_valid = false;
                    }
                }
            }
            println!("[{}] Nums: {:?} Valid: {}", i, new_nums, is_valid);
            if is_valid {
                return true;
            }
        }
        false
    });
    println!("Valid Lines: {}", valid_lines.count());
    Ok(())
}
