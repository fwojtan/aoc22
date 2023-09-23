use petgraph::{prelude::Graph, visit::EdgeRef, Directed, Direction::Outgoing};

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day07;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EdgeType {
    Parent,
    ChildDir(Option<u64>),
    ChildFile(u64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeType {
    Dir((String, Option<u64>)),
    File((String, u64)),
}

impl Solution for Day07 {
    type ParsedInput = (Vec<u64>, u64);

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut filetree = Graph::new();
        let mut cwd = filetree.add_node("/".to_string());
        let root = cwd;
        for line in input_lines.lines() {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            // println!(
            //     "cwd: {}, Neighbours {:?}",
            //     filetree.node_weight(cwd).unwrap(),
            //     filetree
            //         .neighbors(cwd)
            //         .map(|neighbour| filetree.node_weight(neighbour).unwrap())
            //         .collect::<Vec<_>>()
            // );
            match parts[0] {
                "$" => match parts[1] {
                    "cd" => match parts[2] {
                        ".." => {
                            cwd = filetree
                                .edges_directed(cwd, Outgoing)
                                .find(|edge| edge.weight() == &EdgeType::Parent)
                                .expect(".. must aleady have a parent")
                                .target();
                            // println!(
                            //     "Switched to parent dir (cwd={})",
                            //     filetree.node_weight(cwd).unwrap()
                            // );
                        }
                        "/" => {
                            cwd = root;
                            // println!(
                            //     "Switched to root dir(cwd={})",
                            //     filetree.node_weight(cwd).unwrap()
                            // );
                        }
                        _ => {
                            match filetree.neighbors(cwd).find(|neighbour| {
                                *filetree.node_weight(*neighbour).unwrap() == parts[2].to_string()
                            }) {
                                Some(neighbour) => cwd = neighbour,
                                None => {
                                    let prev_wd = cwd;
                                    cwd = filetree.add_node(parts[2].to_string());
                                    filetree.add_edge(prev_wd, cwd, EdgeType::ChildDir(None));
                                    filetree.add_edge(cwd, prev_wd, EdgeType::Parent);
                                    // println!("Added dir {}", parts[2]);
                                }
                            }
                            // println!("Switched to dir {}", parts[2]);
                        }
                    },
                    "ls" => (),
                    _ => {
                        // panic!("Unknown command")
                    }
                },
                "dir" => {
                    let dirname = parts[1];
                    match filetree.neighbors(cwd).find(|neighbour| {
                        *filetree.node_weight(*neighbour).unwrap() == dirname.to_string()
                    }) {
                        Some(_neighbour) => {}
                        None => {
                            let new_node = filetree.add_node(dirname.to_string());
                            filetree.add_edge(cwd, new_node, EdgeType::ChildDir(None));
                            filetree.add_edge(new_node, cwd, EdgeType::Parent);
                            // println!("Added dir {}", dirname);
                        }
                    }
                }
                _ => {
                    match filetree.neighbors(cwd).find(|neighbour| {
                        *filetree.node_weight(*neighbour).unwrap() == parts[1].to_string()
                    }) {
                        Some(_neighbour) => {}
                        None => {
                            let new_node = filetree.add_node(parts[1].to_string());
                            filetree.add_edge(
                                cwd,
                                new_node,
                                EdgeType::ChildFile(
                                    parts[0]
                                        .parse::<u64>()
                                        .expect("Expected first file part to be size"),
                                ),
                            );
                            filetree.add_edge(new_node, cwd, EdgeType::Parent);
                            // println!("Added file {}", parts[1]);
                        }
                    }
                }
            }
        }
        // println!("{:?}", Dot::with_config(&filetree, &[Config::EdgeNoLabel]));
        let mut root_size = 0;
        let dir_sizes = filetree
            .node_indices()
            .filter_map(|node| {
                if filetree
                    .edges_directed(node, Outgoing)
                    .all(|edge| *edge.weight() == EdgeType::Parent)
                {
                    None
                } else {
                    println!(
                        "Getting dir size for {}",
                        filetree.node_weight(node).unwrap()
                    );
                    let dir_size = directory_size(&filetree, node);
                    if node == root {
                        root_size = dir_size;
                    }
                    // println!("Dir size: {}", dir_size);
                    Some(dir_size)
                }
            })
            .collect();
        (dir_sizes, root_size)
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        input
            .0
            .iter()
            .filter(|dir_size| **dir_size <= 100000)
            .sum::<u64>()
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        input
            .0
            .iter()
            .filter(|dir_size| **dir_size > input.1 - 40000000)
            .min()
            .unwrap()
            .to_string()
    }
}

fn directory_size(
    graph: &Graph<String, EdgeType, Directed>,
    node: petgraph::graph::NodeIndex,
) -> u64 {
    let mut size = 0;
    let edges = graph.edges_directed(node, Outgoing);
    // println!("Node: {}", graph.node_weight(node).unwrap(),);
    for edge in edges.clone() {
        match edge.weight() {
            EdgeType::ChildDir(None) => {
                let child_dir = edge.target();
                let child_dir_size = directory_size(graph, child_dir);
                size += child_dir_size;
                // println!(
                //     "Adding child dir size: {} from child node: {}",
                //     child_dir_size,
                //     graph.node_weight(child_dir).unwrap()
                // );
                // graph.update_edge(node, child_dir, EdgeType::ChildDir(Some(child_dir_size)));
            }
            EdgeType::ChildDir(Some(dir_size)) => {
                size += dir_size;
            }
            EdgeType::ChildFile(file_size) => {
                size += file_size;
                // println!(
                //     "Adding child file size: {} from child node: {}",
                //     file_size,
                //     graph.node_weight(edge.target()).unwrap()
                // );
            }
            EdgeType::Parent => {}
        }
    }
    size
}

// struct File {
//     name: String,
//     size: usize,
// }

// struct DirTree {
//     name: String,
//     dir_children: Vec<DirTree>,
//     file_children: Vec<File>,
//     size: Option<usize>,
//     parent: Option<&DirTree>,
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day07_part1_case1() {
        assert_eq!(
            Day07::solve_part_one(
                "$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"
            ),
            "95437".to_string()
        )
    }

    #[test]
    fn check_day07_part2_case1() {
        assert_eq!(Day07::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day07_both_case1() {
        assert_eq!(Day07::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
