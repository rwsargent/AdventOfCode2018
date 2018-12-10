use std::str::SplitWhitespace;

use utils::*;

fn parse_node(input: &mut SplitWhitespace) -> Result<Node> {
  let num_children = input.next().ok_or(Box::new(InvalidInput))?.parse()?;
  let num_metadata = input.next().ok_or(Box::new(InvalidInput))?.parse()?;
  let mut children = Vec::with_capacity(num_children);
  let mut metadata = Vec::with_capacity(num_metadata);
  for _ in 0..num_children {
    children.push(parse_node(input)?);
  }
  for _ in 0..num_metadata {
    metadata.push(input.next().ok_or(Box::new(InvalidInput))?.parse()?);
  }
  Ok(Node {
    children,
    metadata,
  })
}

struct Node {
  children: Vec<Node>,
  metadata: Vec<usize>,
}

fn do_count_metadata(node: &Node) -> usize {
  let children: usize = node.children.iter().map(|n| do_count_metadata(n)).sum();
  let meta: usize = node.metadata.iter().sum();
  children + meta
}

pub fn count_metadata(input: StringInput) -> PuzzleResult {
  let tree = parse_node(&mut input.content.split_whitespace())?;
  Ok(PuzzleSolution::usize(do_count_metadata(&tree)))
}
