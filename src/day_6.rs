use std::collections::HashMap;

use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

pub fn grand_total(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => grand_total_01(path),
        2 => grand_total_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

#[derive(PartialEq)]
enum Operation {
    Addition,
    Multiplication
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Addition => write!(f, "+"),
            Operation::Multiplication => write!(f, "*"),
        }
    }
}


struct Column {
    nums: Vec<u64>,
    operation: Operation
}

impl Column {
    pub fn new() -> Self {
        Column {
            nums: Vec::new(),
            operation: Operation::Addition
        }
    }
}

pub fn grand_total_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let mut map = HashMap::new();
    for line in input.lines() {
        let nums_or_ops = line.split(" ");

        let mut idx = 0;
        for num_or_op in nums_or_ops {
            if num_or_op == "+" || num_or_op == "*" {
                let column: &mut Column = map.get_mut(&idx)
                    .ok_or_else(|| format!("No column found for index {}", idx))?;
                
                column.operation = if num_or_op == "+" {
                    Operation::Addition
                } else {
                    Operation::Multiplication
                };

                idx += 1;
            }
            else if num_or_op == "" {
                continue;
            }
            else {
                let value = num_or_op.parse::<u64>()
                    .map_err(|err| format!("Wrong input: {err}!"))?;

                map.entry(idx).or_insert_with(Column::new).nums.push(value);

                idx += 1;
            }
        }
    }

    let mut grand_total = 0;
    for column in map.values() {
        match column.operation {
            Operation::Addition => grand_total += column.nums.iter().sum::<u64>(),
            Operation::Multiplication => grand_total += column.nums.iter().product::<u64>()
        }
    }

    Ok(grand_total)
}

pub fn grand_total_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Err("Empty input".into());
    }

    let height = lines.len();
    let width = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    // Normalize all lines to equal width
    let grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|l| {
            let mut row: Vec<char> = l.chars().collect();
            row.resize(width, ' ');
            row
        })
        .collect();

    // Group columns into problems
    // A problem consists of consecutive columns that aren't all spaces
    // The operator is in the last row of the rightmost column
    let mut problems: Vec<Vec<usize>> = Vec::new();
    let mut current_problem: Vec<usize> = Vec::new();

    for col in 0..width {
        // Check if this column has any content (non-space in any row)
        let has_content = (0..height).any(|row| grid[row][col] != ' ');

        if has_content {
            // Add to current problem
            current_problem.push(col);
        } else {
            // Empty column - end current problem if it exists
            if !current_problem.is_empty() {
                problems.push(current_problem.clone());
                current_problem.clear();
            }
        }
    }

    // Don't forget the last problem
    if !current_problem.is_empty() {
        problems.push(current_problem);
    }

    let mut grand_total = 0u64;

    // Process each problem
    for problem_cols in problems {
        // The operator is in the last row of the LEFTMOST column of the problem
        let leftmost_col = *problem_cols.first().unwrap();
        let op = grid[height - 1][leftmost_col];

        // Each column forms one number (reading top to bottom)
        // We process columns right-to-left
        let mut numbers: Vec<u64> = Vec::new();

        for &col in problem_cols.iter().rev() {
            let mut num = 0u64;
            for row in 0..height - 1 {
                let c = grid[row][col];
                if c.is_ascii_digit() {
                    let digit = c.to_digit(10).unwrap() as u64;
                    num = num * 10 + digit;
                }
            }
            if num > 0 {
                numbers.push(num);
            }
        }

        if numbers.is_empty() {
            return Err(format!("Problem has operator '{op}' but no numbers."));
        }

        let result: u64 = match op {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => return Err(format!("Invalid operator '{op}'")),
        };

        grand_total += result;
    }

    Ok(grand_total)
}