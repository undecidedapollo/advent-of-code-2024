use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Rules {
    before: u64,
    after: u64,
}

#[derive(Debug)]
struct Update {
    page_order: Vec<u64>,
}

static DEFAULT_EMPTY_VEC_U64: Vec<u64> = vec![];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let filename = std::env::args().nth(1).unwrap();
    println!("Opening file: {}", filename);
    let file = File::open(filename)?;

    // let mut rules: Vec<Rules> = Vec::new();
    let mut rules: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut updates: Vec<Update> = Vec::new();
    let mut hit_empty_line = false;
    BufReader::new(file)
        .lines()
        .filter_map(|f| f.ok())
        .for_each(|x| {
            if !hit_empty_line {
                if x == "" {
                    hit_empty_line = true;
                    return;
                }
                let Some((before, after)) = x.split_once("|") else {
                    return;
                };
                let before: u64 = before.parse().unwrap();
                let after: u64 = after.parse().unwrap();
                rules
                    .entry(before)
                    .and_modify(|f| {
                        f.push(after);
                    })
                    .or_insert(vec![after]);
            } else {
                let splits = x
                    .split(",")
                    .map(|x| x.parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                updates.push(Update { page_order: splits });
            }
        });
    println!("Rules: {:?}", rules);
    println!("Updates: {:?}", updates);

    let mut num_valid = 0;
    let mut sum_middle_digit = 0;
    let mut sum_middle_invalid_digit = 0;
    let total = updates.len();
    for update in updates {
        let mut valid = true;
        'outer_loop: for i in 0..update.page_order.len() {
            let current_num = update.page_order[i];
            let current_rules = rules.get(&current_num).unwrap_or(&DEFAULT_EMPTY_VEC_U64);

            for j in 0..i {
                let compare_num = update.page_order[j];
                if current_rules.contains(&compare_num) {
                    valid = false;
                    break 'outer_loop;
                }
            }
        }
        if valid {
            num_valid += 1;
            let middle_digit = update.page_order[update.page_order.len() / 2 as usize];
            println!("Middle digit for {:?}: {}", update.page_order, middle_digit);
            sum_middle_digit += middle_digit;
            continue;
        }
        let mut current_loop = Vec::from(update.page_order);
        'outer_loop: loop {
            let mut valid = true;
            for i in 0..current_loop.len() {
                let current_num = current_loop[i];
                let current_rules = rules.get(&current_num).unwrap_or(&DEFAULT_EMPTY_VEC_U64);

                for j in 0..i {
                    let compare_num = current_loop[j];
                    if current_rules.contains(&compare_num) {
                        valid = false;
                        current_loop.swap(i, j);
                        continue 'outer_loop;
                    }
                }
            }
            if valid {
                break;
            }
        }
        let middle_digit = current_loop[current_loop.len() / 2 as usize];
        println!(
            "Invalid middle digit for {:?}: {}",
            current_loop, middle_digit
        );
        sum_middle_invalid_digit += middle_digit;
    }
    println!(
        "Total: {} Valid: {} Sum Middle: {} Sum Invalid: {}",
        total, num_valid, sum_middle_digit, sum_middle_invalid_digit
    );
    Ok(())
}

// Go char by char until you hit an x, when you do check all directions for matching the phrase xmas
