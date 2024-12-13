use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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

        let dir = nums[0] - nums[1] > 0;
        println!("Nums: {:?} Dir: {}", nums, dir);
        for i in 0..nums.len() - 1 {
            if dir {
                let num1 = nums[i];
                let num2 = nums[i + 1];
                let diff = num1 - num2;
                if diff != 1 && diff != 2 && diff != 3 {
                    println!("Dir: Desc False: {:?} {:?}", num1, num2);
                    return false;
                }
            } else {
                let num1 = nums[i];
                let num2 = nums[i + 1];
                let diff = num2 - num1;
                if diff != 1 && diff != 2 && diff != 3 {
                    println!("Dir: Asc False: {:?} {:?}", num1, num2);
                    return false;
                }
            }
        }
        true
    });
    println!("Valid Lines: {}", valid_lines.count());
    Ok(())
}
