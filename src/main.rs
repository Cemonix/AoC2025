mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod utils;

const DAY: &str = "day_12";

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
                Ok(result) => println!("Day 5, Fresh ingredients {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_6" => {
            match day_6::grand_total("data/day_6.txt") {
                Ok(result) => println!("Day 6, Grand total {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_7" => {
            match day_7::tachyon("data/day_7.txt") {
                Ok(result) => println!("Day 7, Tachyon beam {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_8" => {
            match day_8::junction_boxes("data/day_8.txt") {
                Ok(result) => println!("Day 8, Largest curcuits {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_9" => {
            match day_9::rectangle("data/day_9.txt") {
                Ok(result) => println!("Day 9, Largest area {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_10" => {
            match day_10::buttons("data/day_10.txt") {
                Ok(result) => println!("Day 10, Buttons pressed {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_11" => {
            match day_11::rack("data/day_11.txt") {
                Ok(result) => println!("Day 11, Rack paths {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        "day_12" => {
            match day_12::present("data/day_12.txt") {
                Ok(result) => println!("Day 12, Region presents fits {}", result),
                Err(err) => println!("Error: {}", err),
            }
        },
        _ => unreachable!(),
    }
    
}
