#[derive(Debug)]
pub struct Node {
    child_nodes: Vec<Node>,
    metadata: Vec<usize>
}

fn parse_node(i: &mut impl Iterator<Item=usize>) -> Node {
    let nodes = i.next().unwrap();
    let metadatas = i.next().unwrap();
    
    let child_nodes = (0..nodes).map(|_| parse_node(i)).collect();
    let metadata = (0..metadatas).map(|_| i.next().unwrap()).collect();

    Node {
        child_nodes, metadata
    }
}

fn sum_metadata(node: &Node) -> usize {
    node.metadata.iter().sum::<usize>() + node.child_nodes.iter().map(sum_metadata).sum::<usize>()
}

fn value(node: &Node) -> usize {
    if node.child_nodes.len() == 0 {
        sum_metadata(node)
    } else {
        node.metadata
            .iter()
            .flat_map(|idx| node.child_nodes.get(idx - 1))
            .map(value)
            .sum()
    }
}

pub fn part1(input: &str) -> (String, Node) {
    let mut iter = input
        .split(" ")
        .map(|c| c.trim().parse::<usize>().unwrap());
    
    let root = parse_node(&mut iter);
    (sum_metadata(&root).to_string(), root)
}

pub fn part2(_: &str, root: Node) -> String {
    value(&root).to_string()
}