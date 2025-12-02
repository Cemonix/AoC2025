use crate::utils::read_file;

const TASK_VERSION: u8 = 2;


pub fn invalid_ids(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => invalid_ids_01(path),
        2 => invalid_ids_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn invalid_ids_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);
    let ranges = parse_input(input)?;

    let mut result = 0;
    for range in ranges {
        let (start, end) = range;

        let ids = start..=end;
        for id in ids {
            let str_id = id.to_string();

            // Only evaluate even-length ids for invalidity
            if str_id.len() % 2 != 1 {
                let splitted_id = str_id.split_at(str_id.len() / 2);
                if splitted_id.0 == splitted_id.1 {
                    result += id;
                }
            }
        }
    }

    Ok(result)
}

pub fn invalid_ids_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);
    let ranges = parse_input(input)?;

    let mut result = 0;
    for range in ranges {
        let (start, end) = range;

        let ids = start..=end;
        for id in ids {
            let str_id = id.to_string();

            // Take first char, then compare it with next sequence until reaching end or not matching 
            // If match not found, increase the window
            // Repeat until reaching half of the length of the id

            let id_len = str_id.len();
            let mut window_len = 1;
            while window_len <= id_len / 2 {
                let splitted_id = str_id.split_at(window_len);
                let current_seq = splitted_id.0;
                let rest_of_id = splitted_id.1;

                if id_len % window_len != 0 {
                    window_len += 1;
                    continue; 
                }
                
                let windows: Vec<&str> = rest_of_id.as_bytes()
                    .chunks_exact(window_len)  // only full chunks
                    .map(|buf| unsafe { str::from_utf8_unchecked(buf) })
                    .collect();

                let all_match = windows.iter().all(|window| *window == current_seq);
                if all_match {
                    result += id;
                    break;
                }

                window_len += 1;
            }
            
        }
    }

    Ok(result)
}

fn parse_input(input: String) -> Result<Vec<(u64, u64)>, String> {
    let str_ranges = input.split(',');

    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for range in str_ranges {
        let range = range.trim();
        let mut start_end_split = range.split('-');

        let start = start_end_split
            .next()
            .ok_or(format!("No start for range {range} found!"))?
            .parse::<u64>()
            .map_err(|err| format!("Cannot parse start of range: {err}"))?;
        let end = start_end_split
            .next()
            .ok_or(format!("No end for range {range} found!"))?
            .parse::<u64>()
            .map_err(|err| format!("Cannot parse end of range: {err}"))?;

        ranges.push((start, end));
    }

    Ok(ranges)
}