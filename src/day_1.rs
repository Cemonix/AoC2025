use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

const INITIAL_POSITION: u8 = 50;
const LOCK_SIZE: i32 = 100;

pub fn safe(path: &str) -> Result<u32, String> {
    match TASK_VERSION {
        1 => safe_01(path),
        2 => safe_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn safe_01(path: &str) -> Result<u32, String> {
    let puzzle = read_file(path);

    let combinations = parse_puzzle(&puzzle)?;
    
    let mut curr_position = INITIAL_POSITION as i32;
    let mut result = 0;
    for comb in combinations {
        if comb.0 == 'L' {
            curr_position = (curr_position - comb.1).rem_euclid(LOCK_SIZE);
        }
        else if comb.0 == 'R' {
            curr_position = (curr_position + comb.1).rem_euclid(LOCK_SIZE);
        }
        else {
            return Err("Wrong direction!".into());
        }

        if curr_position == 0 {
            result += 1;
        }
    }

    Ok(result)
}

pub fn safe_02(path: &str) -> Result<u32, String> {
    let puzzle = read_file(path);

    let combinations = parse_puzzle(&puzzle)?;
    
    let mut curr_position = INITIAL_POSITION as i32;
    let mut result = 0;
    for comb in combinations {
        let turns = (comb.1 / LOCK_SIZE) as u32;
        result += turns;
        
        if comb.0 == 'L' {
            let tmp = comb.1.rem_euclid(LOCK_SIZE);
            let sub = curr_position - tmp;
            if sub < 0 && curr_position != 0 {
                result += 1;
            }

            curr_position = (curr_position - comb.1).rem_euclid(LOCK_SIZE);
        }
        else if comb.0 == 'R' {
            let tmp = comb.1.rem_euclid(LOCK_SIZE);
            let sub = curr_position + tmp;
            if sub > LOCK_SIZE && curr_position != 0 {
                result += 1;
            }

            curr_position = (curr_position + comb.1).rem_euclid(LOCK_SIZE);
        }
        else {
            return Err("Wrong direction!".into());
        }

        if curr_position == 0 {
            result += 1;
        }
    }

    Ok(result)
}


/// Each line containes Left or Right lock turns and positive integer from 0 to 99: L10, R5, L0, R99
fn parse_puzzle(puzzle: &str) -> Result<Vec<(char, i32)>, String> {
    let mut turns: Vec<(char, i32)> = Vec::new();

    for line in puzzle.lines() {
        let (dir, str_value) = line.split_at(1);

        let value = str_value.parse::<i32>().map_err(|err| format!("Wrong lock combination!: {err}"))?;

        turns.push((dir.chars().next().ok_or("Cannot get direction!")?, value));
    }

    Ok(turns)
}