use crate::utils::read_file;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pt {
    x: i64,
    y: i64,
}

const TASK_VERSION: u8 = 2;

pub fn rectangle(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => rectangle_01(path),
        2 => rectangle_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn rectangle_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let mut red: Vec<Pt> = vec![];
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (sx, sy) = line.split_once(',').ok_or(format!("Invalid line: {}", line))?;
        red.push(Pt {
            x: sx.parse().map_err(|_| "bad x")?,
            y: sy.parse().map_err(|_| "bad y")?
        });
    }

    let mut best = 0;
    for i in 0..red.len() {
        for j in i+1..red.len() {
            let area = rect_area(red[i], red[j]);
            if area > best {
                best = area;
            }
        }
    }

    Ok(best)
}

pub fn rectangle_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let mut red: Vec<Pt> = vec![];
    for line in input.lines().filter(|l| !l.trim().is_empty()) {
        let (sx, sy) = line.split_once(',').ok_or(format!("Invalid line: {}", line))?;
        red.push(Pt {
            x: sx.parse().map_err(|_| "bad x")?,
            y: sy.parse().map_err(|_| "bad y")?
        });
    }

    let mut best = 0;

    // Sort pairs by area descending for early termination
    let mut pairs: Vec<(usize, usize, u64)> = Vec::new();
    for i in 0..red.len() {
        for j in i+1..red.len() {
            let area = rect_area(red[i], red[j]);
            pairs.push((i, j, area));
        }
    }
    pairs.sort_unstable_by(|a, b| b.2.cmp(&a.2));

    for (i, j, area) in pairs {
        if area <= best {
            break;
        }

        let a = red[i];
        let b = red[j];

        // Check if rectangle is valid using constraint checking
        if is_rect_valid(&red, a, b) {
            best = area;
        }
    }

    Ok(best)
}

/// Compute inclusive tile area
fn rect_area(a: Pt, b: Pt) -> u64 {
    let width = (a.x - b.x).abs() as u64 + 1;
    let height = (a.y - b.y).abs() as u64 + 1;
    width * height
}

/// Check if a point is inside or on the boundary of a polygon
fn is_inside_or_on_polygon(polygon: &[Pt], x: i64, y: i64) -> bool {
    let n = polygon.len();

    // First check if point is on the boundary
    for i in 0..n {
        let j = (i + 1) % n;
        let p1 = polygon[i];
        let p2 = polygon[j];

        // Check if point is on the line segment between p1 and p2
        if p1.x == p2.x && p1.x == x {
            // Vertical segment
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            if y >= min_y && y <= max_y {
                return true;
            }
        } else if p1.y == p2.y && p1.y == y {
            // Horizontal segment
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            if x >= min_x && x <= max_x {
                return true;
            }
        }
    }

    // Then check if point is inside using ray casting
    let mut inside = false;
    for i in 0..n {
        let j = (i + 1) % n;
        let xi = polygon[i].x;
        let yi = polygon[i].y;
        let xj = polygon[j].x;
        let yj = polygon[j].y;

        let intersect = ((yi > y) != (yj > y))
            && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);

        if intersect {
            inside = !inside;
        }
    }

    inside
}

/// Check if rectangle is valid by checking its four corners
fn is_rect_valid(polygon: &[Pt], corner1: Pt, corner2: Pt) -> bool {
    let min_x = corner1.x.min(corner2.x);
    let max_x = corner1.x.max(corner2.x);
    let min_y = corner1.y.min(corner2.y);
    let max_y = corner1.y.max(corner2.y);

    // Check the four corners and center
    let corners = [
        (min_x, min_y),
        (max_x, min_y),
        (min_x, max_y),
        (max_x, max_y),
        ((min_x + max_x) / 2, (min_y + max_y) / 2),
    ];

    for &(x, y) in &corners {
        if !is_inside_or_on_polygon(polygon, x, y) {
            return false;
        }
    }

    // For small rectangles, check all points
    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;

    if width * height <= 100000 {
        // Check all points for small/medium rectangles
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !is_inside_or_on_polygon(polygon, x, y) {
                    return false;
                }
            }
        }
    } else {
        // For large rectangles, use dense sampling
        // Sample every single coordinate on the edges
        for x in min_x..=max_x {
            if !is_inside_or_on_polygon(polygon, x, min_y) {
                return false;
            }
            if !is_inside_or_on_polygon(polygon, x, max_y) {
                return false;
            }
        }

        for y in min_y..=max_y {
            if !is_inside_or_on_polygon(polygon, min_x, y) {
                return false;
            }
            if !is_inside_or_on_polygon(polygon, max_x, y) {
                return false;
            }
        }

        // Also sample interior with fine grid
        let step = 10; // Check every 10 units in interior
        let mut x = min_x + step;
        while x < max_x {
            let mut y = min_y + step;
            while y < max_y {
                if !is_inside_or_on_polygon(polygon, x, y) {
                    return false;
                }
                y += step;
            }
            x += step;
        }
    }

    true
}
