use crate::utils::read_file;

const TASK_VERSION: u8 = 1;

pub fn ingredients(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => ingredients_01(path),
        2 => ingredients_01(path),
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

    let result = 0;

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