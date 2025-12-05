mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod utils;

const DAY: &str = "day_5";

fn main() {
    match DAY {
        "day_1" => {
            match day_1::safe("data/day_1.txt") {
                Ok(result) => println!("Day 1, Safe password: {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_2" => {
            match day_2::invalid_ids("data/day_2.txt") {
                Ok(result) => println!("Day 2, Invalid Ids sum: {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_3" => {
            match day_3::batteries("data/day_3.txt") {
                Ok(result) => println!("Day 3, Output joltage: {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_4" => {
            match day_4::rolls_of_paper("data/day_4.txt") {
                Ok(result) => println!("Day 4, Rollf os paper {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_5" => {
            match day_5::ingredients("data/day_5.txt") {
                Ok(result) => println!("Day 4, Fresh ingredients {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_6" => unimplemented!(),
        "day_7" => unimplemented!(),
        "day_8" => unimplemented!(),
        "day_9" => unimplemented!(),
        "day_10" => unimplemented!(),
        "day_11" => unimplemented!(),
        "day_12" => unimplemented!(),
        _ => unreachable!(),
    }
    
}
