mod day_1;
mod day_2;
mod utils;

const DAY: &str = "day_2";

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
        "day_3" => unimplemented!(),
        "day_4" => unimplemented!(),
        "day_5" => unimplemented!(),
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
