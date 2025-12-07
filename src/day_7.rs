use std::collections::{ HashSet, HashMap };

use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

pub fn tachyon(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => tachyon_01(path),
        2 => tachyon_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn tachyon_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let grid: Vec<String> = input
        .lines()
        .map(|line| line.to_string())
        .collect();

    if grid.is_empty() {
        return Err("Input grid is empty".into());
    }

    let h = grid.len();
    let w = grid[0].len();

    // --- Find S ---
    let mut start = None;
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.find('S') {
            start = Some((x as isize, y as isize));
            break;
        }
    }

    let (sx, sy) = match start {
        Some(pos) => pos,
        None => return Err("Could not find 'S' in the input".into()),
    };

    let mut visited: HashSet<(isize, isize)> = HashSet::new();
    let mut splits: u64 = 0;

    // Stack = beam origins
    let mut stack = vec![(sx, sy)];

    while let Some((x, mut y)) = stack.pop() {
        // Beam travels downward
        loop {
            y += 1;
            if y < 0 || y >= h as isize {
                break; // out of grid
            }

            let cell = grid[y as usize].as_bytes()[x as usize] as char;

            match cell {
                '^' => {
                    // Beam stops here
                    if visited.insert((x, y)) {
                        splits += 1;

                        // spawn left and right beams
                        let left = x - 1;
                        let right = x + 1;

                        if left >= 0 && left < w as isize {
                            stack.push((left, y));
                        }
                        if right >= 0 && right < w as isize {
                            stack.push((right, y));
                        }
                    }
                    break; // splitter stops this beam
                }
                '.' | 'S' => {
                    // Keep falling
                }
                _ => {
                    // Any unknown char = treat as empty
                }
            }
        }
    }

    Ok(splits)
}

pub fn tachyon_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    // Parse grid
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    if grid.is_empty() {
        return Err("Grid is empty".into());
    }

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    // --- Find S ---
    let mut start = None;
    for (y, row) in grid.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == 'S') {
            start = Some((x as isize, y as isize));
            break;
        }
    }
    let (sx, sy) = match start {
        Some(pos) => pos,
        None => return Err("No 'S' found in input".into()),
    };

    // Memoization table: (x,y) -> number of beam completions
    let mut memo: HashMap<(isize, isize), u64> = HashMap::new();

    // Recursive function that counts all completions from (x,y)
    fn ways_from(
        x: isize,
        y: isize,
        grid: &Vec<Vec<char>>,
        w: isize,
        h: isize,
        memo: &mut HashMap<(isize, isize), u64>,
    ) -> u64 {
        if let Some(&cached) = memo.get(&(x, y)) {
            return cached;
        }

        let mut cy = y;
        // Drop downward until leaving grid or hitting a splitter
        loop {
            cy += 1;
            if cy >= h {
                // Out of grid → this path contributes exactly 1 completion
                memo.insert((x, y), 1);
                return 1;
            }

            let cell = grid[cy as usize][x as usize];

            if cell == '^' {
                // Hit a splitter: the beam can go left OR right
                let mut total = 0;

                let left = x - 1;
                if left >= 0 {
                    total += ways_from(left, cy, grid, w, h, memo);
                }

                let right = x + 1;
                if right < w {
                    total += ways_from(right, cy, grid, w, h, memo);
                }

                memo.insert((x, y), total);
                return total;
            }

            // Otherwise '.', 'S', etc. → keep falling downward
        }
    }

    // Calculate number of paths from the starting beam position
    let total = ways_from(sx, sy, &grid, w, h, &mut memo);

    Ok(total)
}