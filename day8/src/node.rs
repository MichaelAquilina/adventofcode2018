use std::error::Error;
use std::str::{FromStr, SplitWhitespace};

#[derive(Debug, PartialEq)]
pub struct Node {
    pub id: i32,
    pub children: Vec<Box<Node>>,
    pub metadata: Vec<i32>,
}

impl FromStr for Node {
    type Err = NodeErr;
    fn from_str(content: &str) -> Result<Self, Self::Err> {
        let mut tokens = content.split_whitespace();
        Ok(generate_node(&mut tokens, &mut 0)?)
    }
}

// TODO: use generic trait instead of SplitWhitespace
fn generate_node(tokens: &mut SplitWhitespace, current_id: &mut i32) -> Result<Node, NodeErr> {
    let num_children: i32 = tokens
        .next()
        .ok_or(NodeErr::Missing(String::from("# children")))?
        .parse()?;

    let num_metadata: i32 = tokens
        .next()
        .ok_or(NodeErr::Missing(String::from("# metadata entries")))?
        .parse()?;

    let id = *current_id;
    let mut children = vec![];
    let mut metadata = vec![];

    for _ in 0..num_children {
        *current_id += 1;
        children.push(Box::new(generate_node(tokens, current_id)?));
    }

    for _ in 0..num_metadata {
        let data: i32 = tokens
            .next()
            .ok_or(NodeErr::Missing(String::from("metadata")))?
            .parse()?;
        metadata.push(data);
    }

    Ok(Node {
        id,
        metadata,
        children,
    })
}

impl Node {
    // solution for pt1
    pub fn metadata_sum(&self) -> i32 {
        let mut accumulator = 0;

        for value in &self.metadata {
            accumulator += value;
        }

        for node in &self.children {
            accumulator += node.metadata_sum();
        }

        accumulator
    }

    // Solution for pt2
    pub fn value(&self) -> i32 {
        let mut accumulator = 0;

        if self.children.is_empty() {
            accumulator += self.metadata.iter().sum::<i32>();
        } else {
            for &value in &self.metadata {
                let index = value as usize - 1;
                match self.children.get(index) {
                    Some(child) => accumulator += child.value(),
                    None => continue,
                }
            }
        }

        accumulator
    }
}

#[derive(Debug)]
pub enum NodeErr {
    Missing(String),
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for NodeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NodeErr::Missing(msg) => write!(f, "Missing: {}", msg),
            NodeErr::Parse(err) => write!(f, "{}", err),
        }
    }
}

impl Error for NodeErr {}

impl From<std::num::ParseIntError> for NodeErr {
    fn from(err: std::num::ParseIntError) -> Self {
        NodeErr::Parse(err)
    }
}

#[cfg(test)]
mod test_node {
    use super::*;

    #[test]
    fn test_provided_example() -> Result<(), NodeErr> {
        let result: Node = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".parse()?;

        let expected = Node {
            id: 0,
            metadata: vec![1, 1, 2],
            children: vec![
                Box::new(Node {
                    id: 1,
                    metadata: vec![10, 11, 12],
                    children: vec![],
                }),
                Box::new(Node {
                    id: 2,
                    metadata: vec![2],
                    children: vec![Box::new(Node {
                        id: 3,
                        metadata: vec![99],
                        children: vec![],
                    })],
                }),
            ],
        };

        assert_eq!(result, expected);

        assert_eq!(result.metadata_sum(), 138);
        assert_eq!(result.value(), 66);

        Ok(())
    }
}
