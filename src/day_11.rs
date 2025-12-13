use crate::utils::read_file;
use std::collections::HashMap;

const TASK_VERSION: u8 = 2;

pub fn rack(path: &str) -> Result<u64, String> {
    match TASK_VERSION {
        1 => rack_01(path),
        2 => rack_02(path),
        _ => Err("Invalid task version!".into()),
    }
}

pub fn rack_01(path: &str) -> Result<u64, String> {
    let input = read_file(path);

    let graph = parse_graph(&input);

    let mut memo: HashMap<String, u64> = HashMap::new();
    let total = count_paths("you", &graph, &mut memo);

    Ok(total)
}


pub fn rack_02(path: &str) -> Result<u64, String> {
    let input = read_file(path);
    let graph = parse_graph(&input);

    let mut memo = HashMap::new();
    let total = count_paths_with_constraints(
        "svr",
        false,
        false,
        &graph,
        &mut memo,
    );

    Ok(total)
}

fn parse_graph(input: &str) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for line in input.lines() {
        let (from, to) = line
            .split_once(": ")
            .expect("Invalid input format");

        let outputs = to.split_whitespace().map(String::from).collect();
        graph.insert(from.to_string(), outputs);
    }

    graph
}

fn count_paths(
    node: &str,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    if node == "out" {
        return 1;
    }

    if let Some(&cached) = memo.get(node) {
        return cached;
    }

    let mut sum = 0;

    if let Some(neighbors) = graph.get(node) {
        for next in neighbors {
            sum += count_paths(next, graph, memo);
        }
    }

    memo.insert(node.to_string(), sum);
    sum
}

fn count_paths_with_constraints(
    node: &str,
    seen_dac: bool,
    seen_fft: bool,
    graph: &HashMap<String, Vec<String>>,
    memo: &mut HashMap<(String, bool, bool), u64>,
) -> u64 {
    let seen_dac = seen_dac || node == "dac";
    let seen_fft = seen_fft || node == "fft";

    let key = (node.to_string(), seen_dac, seen_fft);

    if let Some(&cached) = memo.get(&key) {
        return cached;
    }

    if node == "out" {
        return if seen_dac && seen_fft { 1 } else { 0 };
    }

    let mut sum = 0;

    if let Some(neighbors) = graph.get(node) {
        for next in neighbors {
            sum += count_paths_with_constraints(
                next,
                seen_dac,
                seen_fft,
                graph,
                memo,
            );
        }
    }

    memo.insert(key, sum);
    sum
}
