use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

#[derive(Debug)]
struct Machine {
    n_lights: usize,
    m_buttons: usize,
    // each button is a list of indices it toggles/increments
    buttons: Vec<Vec<usize>>,
    // target vector for part 1 (binary)
    target: Vec<u8>,
    // joltage requirements for part 2 (integers)
    joltage: Vec<i64>,
}

pub fn buttons(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => buttons_01(path),
        2 => buttons_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn buttons_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);
    let machines = parse_input(&input);

    let mut total: u64 = 0;
    for m in machines {
        match solve_machine(&m) {
            Ok(min_presses) => total += min_presses as u64,
            Err(e) => return Err(format!("No solution for a machine: {}", e)),
        }
    }

    Ok(total)
}


pub fn buttons_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);
    let machines = parse_input(&input);

    let mut total: u64 = 0;
    for m in machines {
        match solve_machine_joltage(&m) {
            Ok(min_presses) => total += min_presses as u64,
            Err(e) => return Err(format!("No solution for a machine: {}", e)),
        }
    }

    Ok(total)
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut out = Vec::new();

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }

        // split into tokens by whitespace
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.is_empty() { continue; }

        // first token is [..#..]
        let lights_tok = tokens[0];
        let lights_inner = &lights_tok[1..lights_tok.len()-1];
        let target: Vec<u8> = lights_inner.chars().map(|c| if c == '#' {1} else {0}).collect();
        let n_lights = target.len();

        // following tokens until a token starting with '{' are buttons
        let mut buttons = Vec::new();
        let mut joltage_tok = None;
        for &tok in tokens.iter().skip(1) {
            if tok.starts_with('{') {
                joltage_tok = Some(tok);
                break;
            }
            let inner = &tok[1..tok.len()-1];
            if inner.trim().is_empty() {
                buttons.push(vec![]);
            } else {
                let nums = inner.split(',')
                    .map(|s| s.parse::<usize>().expect("bad index"))
                    .collect::<Vec<_>>();
                buttons.push(nums);
            }
        }

        let m_buttons = buttons.len();

        // Parse joltage requirements {3,5,4,7}
        let joltage = if let Some(tok) = joltage_tok {
            let inner = &tok[1..tok.len()-1];
            if inner.trim().is_empty() {
                vec![]
            } else {
                inner.split(',')
                    .map(|s| s.parse::<i64>().expect("bad joltage value"))
                    .collect()
            }
        } else {
            vec![]
        };

        out.push(Machine { n_lights, m_buttons, buttons, target, joltage });
    }

    out
}

/// Solve one machine using Gaussian elimination in GF(2).
/// Returns the minimum number of button presses needed.
fn solve_machine(m: &Machine) -> Result<usize, String> {
    let n_lights = m.n_lights;
    let n_buttons = m.m_buttons;

    if n_buttons == 0 {
        let all_off = m.target.iter().all(|&x| x == 0);
        return if all_off { Ok(0) } else { Err("No buttons but target is not all zeros".into()) };
    }

    if n_buttons > 128 {
        return Err("Too many buttons (>128) for u128 solver".into());
    }

    // Build augmented matrix: each row represents a light (equation)
    // Row format: (coefficients as u128 bitmask, target value as u8)
    let mut rows: Vec<(u128, u8)> = vec![(0, 0); n_lights];

    // Fill coefficient matrix: button j toggles certain lights
    for (button_idx, button) in m.buttons.iter().enumerate() {
        for &light_idx in button {
            rows[light_idx].0 ^= 1u128 << button_idx;
        }
    }

    // Set target values
    for (i, &target) in m.target.iter().enumerate() {
        rows[i].1 = target;
    }

    // Gaussian elimination to row echelon form
    let (rank, pivot_cols) = gaussian_elimination(&mut rows, n_lights, n_buttons);

    // Check for inconsistency (unsolvable system)
    for r in rank..n_lights {
        if rows[r].0 == 0 && rows[r].1 == 1 {
            return Err("Inconsistent system: no solution".into());
        }
    }

    // Find free variables (columns without pivots)
    let free_vars: Vec<usize> = (0..n_buttons)
        .filter(|&col| !pivot_cols.contains(&col))
        .collect();

    // Try all combinations of free variables to find minimum weight solution
    let min_presses = find_minimum_solution(&rows, &pivot_cols, &free_vars, n_buttons, rank)?;

    Ok(min_presses)
}

/// Perform Gaussian elimination on the augmented matrix.
/// Returns (rank, list of pivot columns in order).
fn gaussian_elimination(rows: &mut [(u128, u8)], n_rows: usize, n_cols: usize) -> (usize, Vec<usize>) {
    let mut current_row = 0;
    let mut pivot_cols = Vec::new();

    for col in 0..n_cols {
        // Find pivot: a row with 1 in this column
        let pivot_row = (current_row..n_rows)
            .find(|&r| (rows[r].0 >> col) & 1 == 1);

        if let Some(pivot_row) = pivot_row {
            // Swap pivot row to current position
            rows.swap(current_row, pivot_row);
            pivot_cols.push(col);

            // Eliminate this column in all other rows
            for r in 0..n_rows {
                if r != current_row && (rows[r].0 >> col) & 1 == 1 {
                    rows[r].0 ^= rows[current_row].0;
                    rows[r].1 ^= rows[current_row].1;
                }
            }

            current_row += 1;
            if current_row == n_rows {
                break;
            }
        }
    }

    (current_row, pivot_cols)
}

/// Find the minimum weight solution by trying all free variable combinations.
fn find_minimum_solution(
    rows: &[(u128, u8)],
    pivot_cols: &[usize],
    free_vars: &[usize],
    n_buttons: usize,
    rank: usize,
) -> Result<usize, String> {
    if free_vars.is_empty() {
        // Unique solution: just read off pivot variables
        let solution = compute_solution(rows, pivot_cols, &[], 0, n_buttons);
        return Ok(solution.iter().map(|&b| b as usize).sum());
    }

    let k = free_vars.len();
    if k > 26 {
        return Err(format!("Too many free variables ({}). Cannot enumerate 2^{} combinations", k, k));
    }

    // Try all 2^k combinations of free variables
    let mut min_weight = usize::MAX;

    for mask in 0..(1 << k) {
        let solution = compute_solution(rows, pivot_cols, free_vars, mask, n_buttons);

        // Verify solution is correct (for debugging/safety)
        if !verify_solution(&solution, rows, rank) {
            continue;
        }

        let weight = solution.iter().map(|&b| b as usize).sum();
        min_weight = min_weight.min(weight);
    }

    Ok(min_weight)
}

/// Compute a solution given an assignment to free variables (via mask).
fn compute_solution(
    rows: &[(u128, u8)],
    pivot_cols: &[usize],
    free_vars: &[usize],
    free_var_mask: usize,
    n_buttons: usize,
) -> Vec<u8> {
    let mut solution = vec![0u8; n_buttons];

    // Set free variables according to mask
    for (i, &col) in free_vars.iter().enumerate() {
        solution[col] = ((free_var_mask >> i) & 1) as u8;
    }

    // Compute pivot variables from the equations
    for (i, &pivot_col) in pivot_cols.iter().enumerate() {
        let row_coeffs = rows[i].0;
        let target = rows[i].1;

        // Sum contributions from all variables except the pivot
        let mut sum = 0u8;
        for col in 0..n_buttons {
            if col != pivot_col && (row_coeffs >> col) & 1 == 1 {
                sum ^= solution[col];
            }
        }

        solution[pivot_col] = target ^ sum;
    }

    solution
}

/// Verify that a solution satisfies all equations (for debugging).
fn verify_solution(solution: &[u8], rows: &[(u128, u8)], rank: usize) -> bool {
    for i in 0..rank {
        let (coeffs, target) = rows[i];
        let mut sum = 0u8;

        for (col, &val) in solution.iter().enumerate() {
            if (coeffs >> col) & 1 == 1 {
                sum ^= val;
            }
        }

        if sum != target {
            return false;
        }
    }
    true
}

/// Solve machine joltage problem (Part 2) - integer linear system
fn solve_machine_joltage(m: &Machine) -> Result<usize, String> {
    let n_counters = m.joltage.len();
    let n_buttons = m.m_buttons;

    if n_buttons == 0 {
        let all_zero = m.joltage.iter().all(|&x| x == 0);
        return if all_zero { Ok(0) } else { Err("No buttons but joltage is not all zeros".into()) };
    }

    // Build coefficient matrix A where A[i][j] = 1 if button j affects counter i
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; n_buttons]; n_counters];

    for (button_idx, button) in m.buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_counters {
                matrix[counter_idx][button_idx] = 1;
            }
        }
    }

    // Try to solve A*x = b where b is the joltage vector
    // We need non-negative integer solution with minimum sum
    solve_integer_linear_system(&matrix, &m.joltage, n_buttons)
}

/// Solve integer linear system A*x = b with x_i >= 0 and minimize sum(x)
fn solve_integer_linear_system(matrix: &[Vec<i64>], target: &[i64], n_vars: usize) -> Result<usize, String> {
    let n_eqs = matrix.len();

    // Build augmented matrix for Gaussian elimination
    let mut aug: Vec<Vec<i64>> = matrix.iter()
        .zip(target.iter())
        .map(|(row, &t)| {
            let mut r = row.clone();
            r.push(t);
            r
        })
        .collect();

    // Gaussian elimination
    let mut pivot_cols = Vec::new();
    let mut pivot_col_set = vec![false; n_vars];
    let mut current_row = 0;

    for col in 0..n_vars {
        // Find pivot (non-zero element)
        let pivot_row = (current_row..n_eqs)
            .find(|&r| aug[r][col] != 0);

        if let Some(pivot_row) = pivot_row {
            aug.swap(current_row, pivot_row);
            pivot_cols.push((current_row, col));
            pivot_col_set[col] = true;

            // Eliminate column in other rows using integer operations
            let pivot_val = aug[current_row][col];
            for r in 0..n_eqs {
                if r != current_row && aug[r][col] != 0 {
                    let factor = aug[r][col];
                    for j in 0..=n_vars {
                        aug[r][j] = aug[r][j] * pivot_val - aug[current_row][j] * factor;
                    }
                }
            }

            current_row += 1;
            if current_row >= n_eqs {
                break;
            }
        }
    }

    // Check for inconsistency
    for r in current_row..n_eqs {
        if aug[r][n_vars] != 0 {
            return Err("Inconsistent system".into());
        }
    }

    // Find free variables
    let free_vars: Vec<usize> = (0..n_vars)
        .filter(|&col| !pivot_col_set[col])
        .collect();

    // If system has many free variables, this is complex - for now handle small cases
    if free_vars.len() > 20 {
        return Err(format!("Too many free variables: {}", free_vars.len()));
    }

    // Try combinations of free variables
    let mut min_cost = i64::MAX;

    // Determine reasonable bounds for free variables
    // For the problem, use sum of targets as upper bound
    let target_sum: i64 = target.iter().sum();
    let max_free_val = target_sum;

    if free_vars.is_empty() {
        // Unique solution - compute it
        let solution = compute_int_solution(&aug, &pivot_cols, &[], 0, n_vars)?;
        return Ok(solution.iter().sum::<i64>() as usize);
    }

    // For small number of free vars, try reasonable values
    try_free_variable_combinations(&aug, &pivot_cols, &free_vars, n_vars, max_free_val, &mut min_cost)?;

    if min_cost == i64::MAX {
        return Err("No valid non-negative solution found".into());
    }

    Ok(min_cost as usize)
}

fn try_free_variable_combinations(
    aug: &[Vec<i64>],
    pivot_cols: &[(usize, usize)],
    free_vars: &[usize],
    n_vars: usize,
    max_val: i64,
    min_cost: &mut i64,
) -> Result<(), String> {
    // Use iterative approach with stack to try all combinations
    if free_vars.len() == 1 {
        // Single free variable - just try values
        for val in 0..=max_val {
            let free_vals = vec![val];
            match compute_int_solution_with_free(aug, pivot_cols, free_vars, &free_vals, n_vars) {
                Ok(solution) => {
                    let cost: i64 = solution.iter().sum();
                    *min_cost = (*min_cost).min(cost);
                }
                Err(_) => {
                    continue;
                }
            }
        }
        return Ok(());
    }

    // For multiple free variables, enumerate recursively
    enumerate_free_vars(aug, pivot_cols, free_vars, n_vars, &mut vec![], 0, max_val, min_cost)
}

fn enumerate_free_vars(
    aug: &[Vec<i64>],
    pivot_cols: &[(usize, usize)],
    free_vars: &[usize],
    n_vars: usize,
    current_vals: &mut Vec<i64>,
    depth: usize,
    max_val: i64,
    min_cost: &mut i64,
) -> Result<(), String> {
    if depth == free_vars.len() {
        // Try this combination
        match compute_int_solution_with_free(aug, pivot_cols, free_vars, current_vals, n_vars) {
            Ok(solution) => {
                let cost: i64 = solution.iter().sum();
                *min_cost = (*min_cost).min(cost);
            }
            Err(_) => {}
        }
        return Ok(());
    }

    // Try values for this free variable
    for val in 0..=max_val {
        if val > *min_cost {
            break; // Pruning
        }
        current_vals.push(val);
        enumerate_free_vars(aug, pivot_cols, free_vars, n_vars, current_vals, depth + 1, max_val, min_cost)?;
        current_vals.pop();
    }

    Ok(())
}

fn compute_int_solution(
    aug: &[Vec<i64>],
    pivot_cols: &[(usize, usize)],
    _free_vars: &[usize],
    _mask: i64,
    n_vars: usize,
) -> Result<Vec<i64>, String> {
    let mut solution = vec![0i64; n_vars];

    // Compute pivot variables from equations
    for &(row, col) in pivot_cols {
        let pivot_coeff = aug[row][col];
        let rhs = aug[row][n_vars];

        // Check if divisible
        if rhs % pivot_coeff != 0 {
            return Err("Non-integer solution".into());
        }

        let val = rhs / pivot_coeff;
        if val < 0 {
            return Err("Negative solution".into());
        }

        solution[col] = val;
    }

    Ok(solution)
}

fn compute_int_solution_with_free(
    aug: &[Vec<i64>],
    pivot_cols: &[(usize, usize)],
    free_vars: &[usize],
    free_vals: &[i64],
    n_vars: usize,
) -> Result<Vec<i64>, String> {
    let mut solution = vec![0i64; n_vars];

    // Set free variables
    for (i, &col) in free_vars.iter().enumerate() {
        solution[col] = free_vals[i];
    }

    // Compute pivot variables from equations
    for &(row, col) in pivot_cols {
        let pivot_coeff = aug[row][col];
        let mut rhs = aug[row][n_vars];

        // Subtract contributions from free variables
        for j in 0..n_vars {
            if j != col {
                rhs -= aug[row][j] * solution[j];
            }
        }

        // Check if divisible
        if rhs % pivot_coeff != 0 {
            return Err("Non-integer solution".into());
        }

        let val = rhs / pivot_coeff;
        if val < 0 {
            return Err("Negative solution".into());
        }

        solution[col] = val;
    }

    Ok(solution)
}