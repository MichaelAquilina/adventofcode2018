use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Step(char);

type DAG = HashMap<Step, Vec<Step>>;

fn process_dag(contents: &str) -> DAG {
    let mut result: HashMap<Step, Vec<Step>> = HashMap::new();

    for line in contents.lines() {
        let mut tokens = line.split(" must be finished before step ");
        if let Some(parent) = tokens.next() {
            let parent = Step(parent.chars().last().unwrap());
            if let Some(child) = tokens.next() {
                let child = Step(child.chars().next().unwrap());

                let values = result.entry(child).or_insert(vec![]);
                values.push(parent);

                result.entry(parent).or_insert(vec![]);
            }
        }
    }

    result
}

fn find_next_steps(graph: &DAG, done: &HashSet<Step>) -> Option<Step> {
    let mut next = vec![];

    for (step, dependencies) in graph {
        if !done.contains(step) && dependencies.iter().all(|d| done.contains(d)) {
            next.push(*step);
        }
    }

    next.sort_unstable();
    next.reverse();
    next.pop()
}

fn topological_sort(graph: &DAG) -> Vec<Step> {
    let mut result = vec![];
    let mut done: HashSet<Step> = HashSet::new();

    loop {
        match find_next_steps(&graph, &done) {
            Some(step) => {
                done.insert(step);
                result.push(step);
            }
            None => break,
        };
    }

    result
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut contents = String::new();

    std::io::stdin().read_to_string(&mut contents)?;

    let graph = process_dag(&contents);
    let order = topological_sort(&graph);

    let result: String = order.iter().map(|s| s.0).collect();
    println!("{}", result);

    Ok(())
}

#[cfg(test)]
mod test_process_dag {
    use super::*;

    #[test]
    fn test_provided_example() {
        let contents = vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
        .join("\n");

        let result = process_dag(&contents);

        let mut graph = HashMap::new();
        graph.insert(Step('C'), vec![]);
        graph.insert(Step('A'), vec![Step('C')]);
        graph.insert(Step('F'), vec![Step('C')]);
        graph.insert(Step('B'), vec![Step('A')]);
        graph.insert(Step('D'), vec![Step('A')]);
        graph.insert(Step('E'), vec![Step('B'), Step('D'), Step('F')]);

        assert_eq!(result, graph);

        let result = topological_sort(&result);
        let result: String = result.iter().map(|s| s.0).collect();

        let expected = "CABDFE";

        assert_eq!(result, expected);
    }
}
