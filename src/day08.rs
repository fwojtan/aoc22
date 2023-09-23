use std::sync::Arc;

use crate::Solution;

#[derive(Clone, Debug)]
pub struct Day08;

impl Solution for Day08 {
    type ParsedInput = Vec<Vec<u8>>;

    fn parse_input(input_lines: &str) -> Self::ParsedInput {
        // You can leave this as-is if you want to handle the input differently for each part.
        // Alternatively, you can parse the input into two completely separate structs and pass
        // them through together in a tuple.
        input_lines
            .lines()
            .map(|line| {
                line.chars()
                    .map(|digit| {
                        // println!("{:?}", digit);
                        digit.to_digit(10).unwrap() as u8
                    }) // speculatively make u8 for better ? SIMD ?
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>()
    }

    fn part_one(input: &mut Self::ParsedInput) -> String {
        let input = input
            .iter()
            .map(|row| row.iter().map(|&x| x + 1).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>();
        // Shift +1 so we can use 0
        let mut visbility = vec![vec![false; input[0].len()]; input.len()];

        for x in 0..input[0].len() {
            let mut tallest_tree = 0;
            for y in 0..input.len() {
                if input[y][x] > tallest_tree {
                    tallest_tree = input[y][x];
                    visbility[y][x] = true;
                }
            }
        }

        for x in 0..input[0].len() {
            let mut tallest_tree = 0;
            for y in (0..input.len()).rev() {
                if input[y][x] > tallest_tree {
                    tallest_tree = input[y][x];
                    visbility[y][x] = true;
                }
            }
        }

        for y in 0..input.len() {
            let mut tallest_tree = 0;
            for x in 0..input[0].len() {
                if input[y][x] > tallest_tree {
                    tallest_tree = input[y][x];
                    visbility[y][x] = true;
                }
            }
        }

        for y in 0..input.len() {
            let mut tallest_tree = 0;
            for x in (0..input[0].len()).rev() {
                if input[y][x] > tallest_tree {
                    tallest_tree = input[y][x];
                    visbility[y][x] = true;
                }
            }
        }

        visbility
            .iter()
            .map(|row| row.iter().filter(|&&x| x).count())
            .sum::<usize>()
            .to_string()
    }

    fn part_two(input: &mut Self::ParsedInput) -> String {
        let mut scores = vec![vec![0; input[0].len()]; input.len()];
        let forest = Arc::new(input.clone());
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                let mut tree = Tree::new(x, y, input[y][x], forest.clone());
                let score = tree.score();
                // println!(
                //     "x: {}, y: {}, height: {}, score: {}",
                //     x, y, input[y][x], score
                // );
                scores[y][x] = score;
            }
        }
        scores
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
            .to_string()
    }
}

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

struct Tree {
    _x: usize,
    _y: usize,
    height: u8,
    walkers: Vec<Walker>,
}

impl Tree {
    fn new(x: usize, y: usize, height: u8, trees: Arc<Vec<Vec<u8>>>) -> Self {
        let mut tree = Self {
            _x: x,
            _y: y,
            height,
            walkers: Vec::new(),
        };
        let walkers = vec![
            Walker {
                x,
                y,
                dir: 1,
                axis: Axis::X,
                forest: trees.clone(),
            },
            Walker {
                x,
                y,
                dir: 1,
                axis: Axis::Y,
                forest: trees.clone(),
            },
            Walker {
                x,
                y,
                dir: -1,
                axis: Axis::X,
                forest: trees.clone(),
            },
            Walker {
                x,
                y,
                dir: -1,
                axis: Axis::Y,
                forest: trees,
            },
        ];
        tree.walkers = walkers;
        tree
    }

    fn score(&mut self) -> usize {
        // println!("Walkers: {:?}", self.walkers);
        self.walkers
            .iter_mut()
            .map(|walker| {
                let mut score = 0;
                // println!("walker: {:?}", walker);
                for tree in &mut *walker {
                    score += 1;
                    if tree >= self.height {
                        break;
                    }
                }
                // let mut score = walker.take_while(|item| item < &self.height).count();
                // if walker.next().is_some() {
                //     score += 1;
                // }
                // let score = max(score, 1);
                // println!(
                //     "score: {} (dir: {}, axis: {:?})",
                //     score, walker.dir, walker.axis
                // );
                score
            })
            .product()
    }
}

/// Generates trees on the walk away from starting point.
#[derive(Debug)]
struct Walker {
    x: usize,
    y: usize,
    dir: i32,
    axis: Axis,
    forest: Arc<Vec<Vec<u8>>>,
}

impl Iterator for Walker {
    fn next(&mut self) -> Option<u8> {
        match self.axis {
            Axis::X => {
                self.x = (self.x as i32 + self.dir).try_into().ok()?;
                self.forest[self.y].get(self.x).copied()
            }
            Axis::Y => {
                self.y = (self.y as i32 + self.dir).try_into().ok()?;
                self.forest.get(self.y).and_then(|row| Some(row[self.x]))
            }
        }
    }

    type Item = u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_day08_part1_case1() {
        assert_eq!(Day08::solve_part_one(""), "0".to_string())
    }

    #[test]
    fn check_day08_part2_case1() {
        assert_eq!(
            Day08::solve_part_two(
                r"30373
25512
65332
33549
35390"
            ),
            "8".to_string()
        )
    }

    #[test]
    fn check_day08_both_case1() {
        assert_eq!(Day08::solve("", false), ("0".to_string(), "0".to_string()))
    }
}
