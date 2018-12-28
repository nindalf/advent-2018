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
}

struct Graph {
    nodes: HashMap<char, Node>,
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
        Graph { nodes }
    }

    #[allow(dead_code)]
    fn execution_order(&self) -> String {
        let mut completed: HashSet<char> = HashSet::with_capacity(self.nodes.len());
        let mut result: Vec<char> = Vec::with_capacity(self.nodes.len());
        let mut queue: BinaryHeap<nchar> = BinaryHeap::new();

        self.nodes
            .values()
            .filter(|node| node.dependencies.len() == 0)
            .for_each(|node| queue.push(nchar(node.id)));

        while let Some(nc) = queue.pop() {
            let next: char = nc.0;
            completed.insert(next);
            result.push(next);

            let node = self.nodes.get(&next).unwrap();

            for unlock in node.unlocks.iter() {
                let unlocked_node = self.nodes.get(&unlock).unwrap();
                let is_ready = unlocked_node
                    .dependencies
                    .iter()
                    .all(|d| completed.contains(d));
                if is_ready {
                    let nc = nchar(*unlock);
                    queue.push(nc);
                }
            }
        }

        result.iter().collect()
    }
}

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

#[cfg(test)]
mod tests {
    use super::Graph;

    #[test]
    fn test_grid() {
        let graph = Graph::new(TEST_INPUT);
        assert_eq!(6, graph.nodes.len());
        let node_c = graph.nodes.get(&'C').unwrap();
        let node_e = graph.nodes.get(&'E').unwrap();
        assert_eq!(0, node_c.dependencies.len());
        assert_eq!(2, node_c.unlocks.len());
        assert_eq!(true, node_c.unlocks.contains(&'A'));
        assert_eq!(true, node_c.unlocks.contains(&'F'));

        assert_eq!(3, node_e.dependencies.len());
        assert_eq!(0, node_e.unlocks.len());
        assert_eq!(true, node_e.dependencies.contains(&'B'));
        assert_eq!(true, node_e.dependencies.contains(&'D'));
        assert_eq!(true, node_e.dependencies.contains(&'F'));
    }

    #[test]
    fn test_execution_order() {
        let graph = Graph::new(TEST_INPUT);
        assert_eq!("CABDFE", graph.execution_order());

        let graph = Graph::new(REAL_INPUT);
        assert_eq!("BHMOTUFLCPQKWINZVRXAJDSYEG", graph.execution_order());
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
