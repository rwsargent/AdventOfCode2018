use util::input::get_input;

fn sum_tree(data: Vec<i32>) -> i32 {
    let root = build_node(&mut data.iter());
    sum(&root)
}

fn sum(root: &Node) -> i32 {
    let mut childs_sum = 0;
    for child in &root.children {
        childs_sum += sum(&child);
    }
    root.metadata.iter().sum::<i32>() + childs_sum
}

fn build_node<'a, I>(data: &mut I) -> Node
where
    I: Iterator<Item = &'a i32>,
{
    let mut children = Vec::new();
    let mut meta_data = Vec::new();
    for _ in 0..*data.next().unwrap_or(&0) {
        children.push(build_node(data));
    }

    for _ in 0..*data.next().unwrap_or(&0) {
        meta_data.push(*data.next().unwrap_or(&0));
    }
    Node::new(meta_data, children)
}

impl Node {
    fn new(meta: Vec<i32>, childs: Vec<Node>) -> Self {
        Node {
            metadata: meta,
            children: childs,
        }
    }
}

struct Node {
    metadata: Vec<i32>,
    children: Vec<Node>,
}

fn part_one() {
    let nums = get_input("inputs/day08.txt").as_ints();
}
