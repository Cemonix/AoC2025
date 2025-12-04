use crate::utils::read_file;

const TASK_VERSION: u8 = 2;
const BATTERIES_NEEDED: usize = 12;

pub fn batteries(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => batteries_01(path),
        2 => batteries_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn batteries_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let mut result = 0;
    for line in input.lines() {
        let batteries_joltage: Vec<u32> = line
            .chars()
            .map(|char| char.to_digit(10).ok_or_else(|| format!("Invalid digit: {}", char)))
            .collect::<Result<Vec<u32>, String>>()?;

        if batteries_joltage.len() < 2 {
            return Err("Not enough batteries to calculate voltage".into());
        }

        // Finding first maximum
        let mut max = 0;
        let mut idx = 0;
        for joltage in batteries_joltage.iter().enumerate() {
            if *joltage.1 > max {
                idx = joltage.0;
                max = *joltage.1;
            }
        }

        let is_last = idx == batteries_joltage.len() - 1;

        let batteries_slice = if is_last { &batteries_joltage[..idx] } else { &batteries_joltage[idx+1..] };

        let mut second_max = 0;
        for joltage in batteries_slice.iter().enumerate() {
            if *joltage.1 > second_max {
                second_max = *joltage.1;
            }
        }

        // Edge case - maximum is last item
        // Swap the maximums
        if is_last {
            let tmp = max;
            max = second_max;
            second_max = tmp;
        }

        // Create two digit number
        let found_voltage = (max.to_string() + &second_max.to_string()).parse::<u64>().unwrap();
        result += found_voltage;
    }

    Ok(result)
}

pub fn batteries_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let mut sum: u64 = 0;
    for line in input.lines() {
        if line.len() < BATTERIES_NEEDED {
            return Err("Not enough batteries".into());
        }

        let best = max_subsequence(line, BATTERIES_NEEDED);

        let value = best.parse::<u64>()
            .map_err(|e| format!("Parse error: {e}"))?;

        sum += value;
    }

    Ok(sum)
}

fn max_subsequence(line: &str, k: usize) -> String {
    let chars: Vec<char> = line.chars().collect();
    let n = chars.len();

    let mut result = String::with_capacity(k);
    let mut start = 0;

    for remaining in (1..=k).rev() {
        // The last index where we can pick the next digit
        let end = n - remaining;

        // Find the max digit in chars[start..=end]
        let mut max_digit = '0';
        let mut max_idx = start;

        for i in start..=end {
            if chars[i] > max_digit {
                max_digit = chars[i];
                max_idx = i;

                // Early stop if we found '9'
                if max_digit == '9' {
                    break;
                }
            }
        }

        // Append it
        result.push(max_digit);

        // Move start after the chosen digit
        start = max_idx + 1;
    }

    result
}