use crate::Solution;
use petgraph::{algo::dijkstra, prelude::Graph, stable_graph::NodeIndex};

#[derive(Clone, Debug)]
pub struct Day12;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Location {
    x: usize,
    y: usize,
    height: u8,
}

impl Location {
    fn can_travel_to(&self, other: Location) -> bool {
        other.height <= self.height + 1
    }
}

pub struct WorldMap {
    world_graph: Graph<Location, u8>,
    world_grid: Vec<Vec<(Location, NodeIndex)>>,
    start_idx: NodeIndex,
    end_idx: NodeIndex,
}

impl Solution for Day12 {
    type ParsedInput = WorldMap;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        let mut world_graph = Graph::<Location, u8>::new();
        let mut world_grid = vec![];
        let mut start_idx = None;
        let mut end_idx = None;

        for (y, line) in input_lines.lines().enumerate() {
            let mut row = vec![];
            for (x, height) in line.chars().enumerate() {
                let mut height = height;
                if height == 'S' {
                    height = 'a';
                    let height = u8::try_from(height).unwrap();
                    let loc = Location { x, y, height };
                    let idx = world_graph.add_node(loc.clone());
                    start_idx = Some(idx.clone());
                    row.push((loc, idx));
                } else if height == 'E' {
                    height = 'z';
                    let height = u8::try_from(height).unwrap();
                    let loc = Location { x, y, height };
                    let idx = world_graph.add_node(loc.clone());
                    end_idx = Some(idx.clone());
                    row.push((loc, idx));
                } else {
                    let height = u8::try_from(height).unwrap();
                    let loc = Location { x, y, height };
                    let idx = world_graph.add_node(loc.clone());
                    row.push((loc, idx));
                }
            }
            world_grid.push(row);
        }
        for source_idx in world_graph.node_indices() {
            let source_loc = world_graph[source_idx].clone();
            let mut target_idxs = vec![];
            target_idxs.push((source_loc.x + 1, source_loc.y));
            if let Some(x) = source_loc.x.checked_sub(1) {
                target_idxs.push((x, source_loc.y));
            }
            target_idxs.push((source_loc.x, source_loc.y + 1));
            if let Some(y) = source_loc.y.checked_sub(1) {
                target_idxs.push((source_loc.x, y));
            }

            for (x, y) in target_idxs {
                if let Some((target_loc, target_idx)) = world_grid.get(y).and_then(|row| row.get(x))
                {
                    if source_loc.can_travel_to(target_loc.clone()) {
                        world_graph.add_edge(source_idx, *target_idx, 1);
                    }
                }
            }
        }
        WorldMap {
            world_graph,
            world_grid,
            start_idx: start_idx.unwrap(),
            end_idx: end_idx.unwrap(),
        }
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let shortest_path = dijkstra(
            &input.world_graph,
            input.start_idx,
            Some(input.end_idx.clone()),
            |_e| 1,
        );

        shortest_path.get(&input.end_idx).unwrap().to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut paths = vec![];
        for loc in input.world_grid.iter().flatten() {
            if loc.0.height == 'a'.try_into().unwrap() {
                let shortest_path = dijkstra(
                    &input.world_graph,
                    loc.1,
                    Some(input.end_idx.clone()),
                    |_e| 1,
                );
                if let Some(shortest) = shortest_path.get(&input.end_idx) {
                    paths.push(shortest.clone());
                }
            }
        }
        paths.iter().min().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day12_part1_case1() {
        assert_eq!(Day12::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day12_part2_case1() {
        assert_eq!(Day12::solve_part_two(""), "0".to_string())
    }

    #[test]
    fn check_day12_both_case1() {
        assert_eq!(Day12::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
