use std::collections::BinaryHeap;

use crate::utils::read_file;

const TASK_VERSION: u8 = 2;

#[derive(Clone, Copy)]
struct Coor {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Coor {
    pub fn calc_dist2(&self, other: &Coor) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

pub fn junction_boxes(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => junction_boxes_01(path),
        2 => junction_boxes_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn junction_boxes_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let boxes: Vec<Coor> = input
        .lines()
        .map(parse_coordinate)
        .collect::<Result<_, _>>()?;

    let count = boxes.len();
    if count < 3 {
        return Err("Need at least 3 junction boxes".into());
    }

    // Keep the 1000 shortest edges using a max-heap of (distance, i, j).
    // The heap stores the largest distance at the top, so when we see a
    // new distance smaller than the top we replace it.
    let mut closest_pairs: BinaryHeap<(i64, usize, usize)> = BinaryHeap::new();

    for i in 0..count {
        for j in (i + 1)..count {
            let distance = boxes[i].calc_dist2(&boxes[j]);
            if closest_pairs.len() < 1000 {
                closest_pairs.push((distance, i, j));
            } else if distance < closest_pairs.peek().unwrap().0 {
                // current distance is smaller than the largest in the heap;
                // replace the largest with this smaller distance
                closest_pairs.pop();
                closest_pairs.push((distance, i, j));
            }
        }
    }

    // Create union–find structure
    let mut parents: Vec<usize> = (0..count).collect();
    let mut sizes: Vec<u64> = vec![1; count];

    for (_, a, b) in closest_pairs {
        union(&mut parents, &mut sizes, a, b);
    }

    // Collect component sizes
    let mut component_sizes = Vec::new();
    for i in 0..count {
        if find(&mut parents, i) == i {
            component_sizes.push(sizes[i]);
        }
    }

    component_sizes.sort_unstable_by(|a, b| b.cmp(a));

    if component_sizes.len() < 3 {
        return Err("Not enough circuits to multiply top 3".into());
    }

    Ok(component_sizes[0] * component_sizes[1] * component_sizes[2])
}

pub fn junction_boxes_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let boxes: Vec<Coor> = input
        .lines()
        .map(parse_coordinate)
        .collect::<Result<_, _>>()?;

    let count = boxes.len();
    if count < 2 {
        return Err("Need at least 2 junction boxes".into());
    }

    // Build all pair distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    edges.reserve(count * (count - 1) / 2);

    for i in 0..count {
        for j in (i + 1)..count {
            let d = boxes[i].calc_dist2(&boxes[j]);
            edges.push((d, i, j));
        }
    }

    // Sort edges by distance ascending
    edges.sort_unstable_by_key(|e| e.0);

    // Union–find
    let mut parents: Vec<usize> = (0..count).collect();
    let mut sizes: Vec<u64> = vec![1; count];
    let mut components = count;

    let mut last_pair: Option<(usize, usize)> = None;

    for &(_, a, b) in &edges {
        let root_a = find(&mut parents, a);
        let root_b = find(&mut parents, b);

        if root_a != root_b {
            union(&mut parents, &mut sizes, a, b);
            components -= 1;

            // This pair actually merged two components
            last_pair = Some((a, b));

            if components == 1 {
                break;
            }
        }
    }

    let (a, b) = last_pair.ok_or("Graph was already connected")?;
    let result = (boxes[a].x as u64) * (boxes[b].x as u64);

    Ok(result)
}

fn parse_coordinate(line: &str) -> Result<Coor, String> {
    let parts: Vec<&str> = line.split(',').collect();

    if parts.len() != 3 {
        return Err(format!("Cannot parse coordinates: {line}"));
    }

    Ok(Coor {
        x: parts[0].parse::<i64>().map_err(|e| format!("{e}"))?,
        y: parts[1].parse::<i64>().map_err(|e| format!("{e}"))?,
        z: parts[2].parse::<i64>().map_err(|e| format!("{e}"))?,
    })
}

fn find(parents: &mut [usize], x: usize) -> usize {
    if parents[x] != x {
        parents[x] = find(parents, parents[x]);
    }
    parents[x]
}

fn union(parents: &mut [usize], sizes: &mut [u64], a: usize, b: usize) {
    let mut root_a = find(parents, a);
    let mut root_b = find(parents, b);

    if root_a == root_b {
        return;
    }

    if sizes[root_a] < sizes[root_b] {
        std::mem::swap(&mut root_a, &mut root_b);
    }

    parents[root_b] = root_a;
    sizes[root_a] += sizes[root_b];
}
