use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

pub fn ingredients(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => ingredients_01(path),
        2 => ingredients_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn ingredients_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let parsed_input = parse_input(&input)?;
    let ranges = parsed_input.0;
    let ingredients = parsed_input.1;

    let mut result = 0;
    for ingredient in &ingredients {
        if ranges.iter().any(|(start, end)| ingredient >= start && ingredient <= end) {
            result += 1;
        }
    }

    Ok(result)
}

pub fn ingredients_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let parsed_input = parse_input(&input)?;
    let ranges = parsed_input.0;

    // Merge ranges - take spaces into account
    
    let merged_ranges = merge_ranges(ranges);

    // Subtract the range start and end and add 2 for inclusivity
    let mut result = 0;
    for (start, end) in merged_ranges {
        result += end - start + 1
    }

    Ok(result)
}

fn parse_input(input: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>), String> {
    let mut ranges = Vec::new();
    let mut ingredients = Vec::new();
    for line in input.lines() {
        if line.contains('-') {
            let mut range = line.split('-');
            ranges.push(
                (
                    range.next().ok_or("Wrong input!")?.parse::<u64>().map_err(|err| format!("Wrong input {err}"))?,
                    range.next().ok_or("Wrong input!")?.parse::<u64>().map_err(|err| format!("Wrong input {err}"))?
                )
            )
        }
        else {
            if line.len() > 0 {
                ingredients.push(line.parse::<u64>().map_err(|err| format!("Wrong input {err}"))?);
            }
        }
    }

    Ok((ranges, ingredients))
}

fn merge_ranges(ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut merged_ranges: Vec<(u64, u64)> = vec![ranges[0]];
    for new_range in &ranges[1..] {
        let mut overlaps = false;
        for i in 0..merged_ranges.len() {
            if do_ranges_overlap(new_range, &merged_ranges[i]) {
                overlaps = true;
                let new_start = new_range.0.min(merged_ranges[i].0);
                let new_end = new_range.1.max(merged_ranges[i].1);
                
                merged_ranges[i] = (new_start, new_end);
                break;
            }
        }
        
        if !overlaps {
            merged_ranges.push(*new_range);
        }
    }

    // No merged happened
    if ranges.len() == merged_ranges.len() {
        return merged_ranges
    }

    merge_ranges(merged_ranges)
}

fn do_ranges_overlap(range_1: &(u64, u64), range_2: &(u64, u64)) -> bool {
    // Ranges overlap if range_1 doesn't end before range_2 starts
    // AND range_2 doesn't end before range_1 starts
    range_1.1 >= range_2.0 && range_2.1 >= range_1.0
}