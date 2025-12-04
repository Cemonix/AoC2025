use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

pub fn rolls_of_paper(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => rolls_of_paper_01(path),
        2 => rolls_of_paper_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn rolls_of_paper_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    // Borrow each line as a byte slice
    let grid: Vec<&[u8]> = input
        .lines()
        .map(|line| line.as_bytes())
        .collect();

    if grid.is_empty() {
        return Err("Input is empty".to_string());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut accessible_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] != b'@' {
                continue;
            }

            let mut adjacent_rolls = 0;

            // Explore 8 directions around (row, col)
            for d_row in -1i32..=1 {
                for d_col in -1i32..=1 {
                    if d_row == 0 && d_col == 0 {
                        continue; // skip the tile itself
                    }

                    let nr = row as i32 + d_row; // neighbor row
                    let nc = col as i32 + d_col; // neighbor col

                    // Check bounds
                    if nr >= 0 && nr < rows as i32 &&
                       nc >= 0 && nc < cols as i32 &&
                       grid[nr as usize][nc as usize] == b'@'
                    {
                        adjacent_rolls += 1;
                    }
                }
            }

            if adjacent_rolls < 4 {
                accessible_count += 1;
            }
        }
    }

    Ok(accessible_count)
}

pub fn rolls_of_paper_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    // We need mutability now because we remove '@' rolls
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    if grid.is_empty() {
        return Err("Input is empty".to_string());
    }

    let rows = grid.len();
    let cols = grid[0].len();

    // Offsets for all 8 neighbors
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        ( 0, -1),          ( 0, 1),
        ( 1, -1), ( 1, 0), ( 1, 1),
    ];

    let mut removed_total = 0;

    loop {
        let mut to_remove = Vec::new();

        // Scan grid for removable rolls
        for row in 0..rows {
            for col in 0..cols {
                if grid[row][col] != b'@' {
                    continue;
                }

                let mut neighbors = 0;

                for (dr, dc) in directions {
                    let nr = row as i32 + dr;
                    let nc = col as i32 + dc;

                    if nr >= 0 && nr < rows as i32 &&
                       nc >= 0 && nc < cols as i32 &&
                       grid[nr as usize][nc as usize] == b'@'
                    {
                        neighbors += 1;
                    }
                }

                if neighbors < 4 {
                    to_remove.push((row, col));
                }
            }
        }

        // Stop when nothing more is removable
        if to_remove.is_empty() {
            break;
        }

        // Remove all marked rolls
        for (row, col) in to_remove {
            grid[row][col] = b'.';
            removed_total += 1;
        }
    }

    Ok(removed_total)
}
