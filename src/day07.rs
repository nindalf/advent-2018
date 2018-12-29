use regex::Regex;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Node {
    id: char,
    unlocks: HashSet<char>,
    dependencies: HashSet<char>,
}

impl Node {
    fn new(id: char) -> Node {
        Node {
            id: id,
            unlocks: HashSet::new(),
            dependencies: HashSet::new(),
        }
    }

    fn cost(&self, base_cost: u32) -> u32 {
        (self.id as u32) - ('A' as u32) + base_cost
    }
}

struct Graph {
    nodes: HashMap<char, Node>,
    completed: HashSet<char>,
    exec_queue: BinaryHeap<nchar>,
}

impl Graph {
    #[allow(dead_code)]
    fn new(s: &str) -> Graph {
        lazy_static! {
            static ref RE: Regex = Regex::new("Step (?P<source>[A-Z]) must be finished before step (?P<destination>[A-Z]) can begin.").unwrap();
        }
        let mut nodes: HashMap<char, Node> = HashMap::new();
        for l in s.lines() {
            let caps = RE.captures(l).unwrap();
            let source: char = caps["source"].chars().next().unwrap();
            let destination: char = caps["destination"].chars().next().unwrap();

            let source_node = nodes.entry(source).or_insert_with(|| Node::new(source));
            source_node.unlocks.insert(destination);

            let destination_node = nodes
                .entry(destination)
                .or_insert_with(|| Node::new(destination));
            destination_node.dependencies.insert(source);
        }

        let completed = HashSet::new();

        let mut exec_queue: BinaryHeap<nchar> = BinaryHeap::new();
        nodes
            .values()
            .filter(|node| node.dependencies.is_empty())
            .for_each(|node| exec_queue.push(nchar(node.id)));

        Graph {
            nodes,
            completed,
            exec_queue,
        }
    }

    #[allow(dead_code)]
    fn execution_order(&mut self) -> String {
        let mut result: Vec<char> = Vec::with_capacity(self.nodes.len());
        while let Some(node_id) = self.next() {
            self.complete_node(node_id);
            result.push(node_id);
        }
        result.iter().collect()
    }

    #[allow(dead_code)]
    fn execution_time(&mut self, num_workers: usize, base_cost: u32) -> u32 {
        let mut time: u32 = 0;
        let mut workers: Vec<WorkerStatus> = vec![WorkerStatus::Idle; num_workers];
        loop {
            // check if worker is free and assign if so
            for i in 0..workers.len() {
                if workers[i] == WorkerStatus::Idle {
                    match self.next() {
                        Some(node_id) => {
                            let node = self.nodes.get(&node_id).unwrap();
                            let completion_time = time + node.cost(base_cost);
                            workers[i] = WorkerStatus::Working(node_id, completion_time);
                        }
                        None => (),
                    };
                }
            }
            // if all workers are idle, there is no work left
            let free_workers = workers
                .iter()
                .filter(|status| **status == WorkerStatus::Idle)
                .count();
            if free_workers == workers.len() {
                break;
            }
            // check if worker has completed their work
            for i in 0..workers.len() {
                match workers[i] {
                    WorkerStatus::Idle => (),
                    WorkerStatus::Working(node, completion_time) => {
                        if time >= completion_time {
                            self.complete_node(node);
                            workers[i] = WorkerStatus::Idle;
                        }
                    }
                }
            }
            // time moves on
            time = time + 1;
        }
        time
    }

    fn complete_node(&mut self, node_id: char) {
        self.completed.insert(node_id);
        let node = self.nodes.get(&node_id).unwrap();
        for unlock in node.unlocks.iter() {
            let unlocked_node = self.nodes.get(&unlock).unwrap();
            let is_ready = unlocked_node
                .dependencies
                .iter()
                .all(|d| self.completed.contains(d));
            if is_ready {
                let nc = nchar(*unlock);
                self.exec_queue.push(nc);
            }
        }
    }
}

impl Iterator for Graph {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.exec_queue.pop() {
            Some(nc) => Some(nc.0),
            None => None,
        }
    }
}

// nchar is a newtype of char
// the only thing it does is reversing the order of comparison
// this makes the max-heap BinaryHeap into a min-heap
#[allow(non_camel_case_types)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct nchar(pub char);

impl Ord for nchar {
    fn cmp(&self, other: &nchar) -> Ordering {
        if self.0 == other.0 {
            Ordering::Equal
        } else if self.0 < other.0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for nchar {
    fn partial_cmp(&self, other: &nchar) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum WorkerStatus {
    Idle,
    Working(char, u32),
}

#[cfg(test)]
mod tests {
    use super::{Graph, Node};

    #[test]
    fn test_grid() {
        let graph = Graph::new(TEST_INPUT);
        assert_eq!(6, graph.nodes.len());
        let node_c = graph.nodes.get(&'C').unwrap();
        let node_e = graph.nodes.get(&'E').unwrap();
        assert_eq!(true, node_c.dependencies.is_empty());
        assert_eq!(2, node_c.unlocks.len());
        assert_eq!(true, node_c.unlocks.contains(&'A'));
        assert_eq!(true, node_c.unlocks.contains(&'F'));

        assert_eq!(3, node_e.dependencies.len());
        assert_eq!(true, node_e.unlocks.is_empty());
        assert_eq!(true, node_e.dependencies.contains(&'B'));
        assert_eq!(true, node_e.dependencies.contains(&'D'));
        assert_eq!(true, node_e.dependencies.contains(&'F'));
    }

    #[test]
    fn test_execution_order() {
        let mut graph = Graph::new(TEST_INPUT);
        assert_eq!("CABDFE", graph.execution_order());

        let mut graph = Graph::new(REAL_INPUT);
        assert_eq!("BHMOTUFLCPQKWINZVRXAJDSYEG", graph.execution_order());
    }

    #[test]
    fn test_execution_time() {
        let mut graph = Graph::new(TEST_INPUT);
        assert_eq!(15, graph.execution_time(2, 0));
        let mut graph = Graph::new(REAL_INPUT);
        assert_eq!(877, graph.execution_time(5, 60));
    }

    #[test]

    fn test_cost() {
        let node_a = Node::new('A');
        assert_eq!(100, node_a.cost(100));
        let node_z = Node::new('Z');
        assert_eq!(125, node_z.cost(100));
    }

    const TEST_INPUT: &'static str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    const REAL_INPUT: &'static str = "Step U must be finished before step A can begin.
Step F must be finished before step Z can begin.
Step B must be finished before step J can begin.
Step O must be finished before step R can begin.
Step H must be finished before step S can begin.
Step T must be finished before step R can begin.
Step L must be finished before step W can begin.
Step M must be finished before step I can begin.
Step Q must be finished before step K can begin.
Step Z must be finished before step V can begin.
Step C must be finished before step E can begin.
Step W must be finished before step I can begin.
Step K must be finished before step S can begin.
Step I must be finished before step Y can begin.
Step P must be finished before step V can begin.
Step V must be finished before step X can begin.
Step R must be finished before step E can begin.
Step N must be finished before step E can begin.
Step X must be finished before step J can begin.
Step A must be finished before step J can begin.
Step S must be finished before step G can begin.
Step J must be finished before step E can begin.
Step Y must be finished before step E can begin.
Step D must be finished before step G can begin.
Step E must be finished before step G can begin.
Step K must be finished before step N can begin.
Step B must be finished before step I can begin.
Step X must be finished before step S can begin.
Step V must be finished before step S can begin.
Step U must be finished before step L can begin.
Step N must be finished before step G can begin.
Step O must be finished before step L can begin.
Step X must be finished before step E can begin.
Step V must be finished before step E can begin.
Step Y must be finished before step G can begin.
Step A must be finished before step Y can begin.
Step M must be finished before step E can begin.
Step F must be finished before step Q can begin.
Step F must be finished before step X can begin.
Step L must be finished before step C can begin.
Step T must be finished before step L can begin.
Step B must be finished before step C can begin.
Step Q must be finished before step N can begin.
Step T must be finished before step G can begin.
Step R must be finished before step D can begin.
Step I must be finished before step A can begin.
Step B must be finished before step M can begin.
Step H must be finished before step A can begin.
Step F must be finished before step K can begin.
Step U must be finished before step F can begin.
Step R must be finished before step A can begin.
Step J must be finished before step D can begin.
Step V must be finished before step Y can begin.
Step F must be finished before step J can begin.
Step C must be finished before step K can begin.
Step M must be finished before step C can begin.
Step F must be finished before step E can begin.
Step I must be finished before step E can begin.
Step T must be finished before step A can begin.
Step J must be finished before step Y can begin.
Step R must be finished before step X can begin.
Step W must be finished before step S can begin.
Step V must be finished before step R can begin.
Step U must be finished before step V can begin.
Step C must be finished before step V can begin.
Step F must be finished before step Y can begin.
Step R must be finished before step G can begin.
Step W must be finished before step N can begin.
Step H must be finished before step N can begin.
Step H must be finished before step Y can begin.
Step B must be finished before step W can begin.
Step M must be finished before step Z can begin.
Step X must be finished before step A can begin.
Step A must be finished before step G can begin.
Step N must be finished before step A can begin.
Step H must be finished before step J can begin.
Step B must be finished before step O can begin.
Step W must be finished before step A can begin.
Step P must be finished before step N can begin.
Step Z must be finished before step G can begin.
Step W must be finished before step D can begin.
Step D must be finished before step E can begin.
Step W must be finished before step J can begin.
Step N must be finished before step D can begin.
Step C must be finished before step J can begin.
Step B must be finished before step Y can begin.
Step F must be finished before step P can begin.
Step L must be finished before step P can begin.
Step X must be finished before step G can begin.
Step R must be finished before step Y can begin.
Step K must be finished before step A can begin.
Step M must be finished before step Y can begin.
Step W must be finished before step Y can begin.
Step F must be finished before step I can begin.
Step L must be finished before step X can begin.
Step R must be finished before step J can begin.
Step V must be finished before step J can begin.
Step V must be finished before step D can begin.
Step H must be finished before step C can begin.
Step O must be finished before step G can begin.
Step P must be finished before step R can begin.";
}
