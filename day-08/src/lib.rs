use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, digit1, newline},
    combinator::verify,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    *,
};

fn parse_trees(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, vecs) = separated_list1(
        newline,
        digit1.map(|nums: &str| nums.chars().map(|num| num.to_digit(10).unwrap()).collect()),
    )(input)?;

    Ok((input, vecs))
}

pub fn process_part1(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let max_length = trees.len() - 1;
    let mut visible_trees: Vec<Vec<bool>> = trees
        .iter()
        .enumerate()
        .map(|(i, tree_line)| {
            let line_max_length = tree_line.len() - 1;
            tree_line
                .iter()
                .enumerate()
                .map(|(line_i, _)| {
                    if i == 0 || i == max_length || line_i == 0 || line_i == line_max_length {
                        true
                    } else {
                        false
                    }
                })
                .collect()
        })
        .collect();

    for y in 0..trees.len() {
        let mut current_tree_size = 0;
        for x in 0..trees[0].len() {
            if x == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    for y in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for x in (0..trees[0].len()).rev() {
            if x == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    // Iteration for Ys
    for x in 0..trees.len() {
        let mut current_tree_size = 0;
        for y in 0..trees[0].len() {
            if y == 0 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    for x in (0..trees.len()).rev() {
        let mut current_tree_size = 0;
        for y in (0..trees[0].len()).rev() {
            if y == trees.len() - 1 {
                current_tree_size = trees[y][x] as usize;
            } else if trees[y][x] > current_tree_size as u32 {
                current_tree_size = trees[y][x] as usize;
                visible_trees[y][x] = true;
            }
        }
    }

    visible_trees
        .iter()
        .flatten()
        .filter(|&&v| v)
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, trees) = parse_trees(input).unwrap();
    let max_length = trees.len() - 1;
    let mut high_score = 0;

    let y_max = trees.len();
    let x_max = trees[0].len();

    for (y_index, tree_line) in trees.iter().enumerate() {
        for (x_index, tree_house_height) in tree_line.iter().enumerate() {
            let mut scores = [0, 0, 0, 0];
            println!("x: {x_index}, Y: {y_index}, {tree_house_height}");

            // to left
            for x_pos in (0..x_index).rev() {
                if trees[y_index][x_pos] < *tree_house_height {
                    scores[0] += 1;
                } else {
                    scores[0] += 1;
                    break;
                }
            }

            // to right
            for x_pos in (x_index + 1)..x_max {
                if trees[y_index][x_pos] < *tree_house_height {
                    scores[1] += 1;
                } else {
                    scores[1] += 1;
                    break;
                }
            }

            // to up
            for y_pos in (0..y_index).rev() {
                if trees[y_pos][x_index] < *tree_house_height {
                    scores[2] += 1;
                } else {
                    scores[2] += 1;
                    break;
                }
            }

            // to down
            for y_pos in (y_index + 1)..y_max {
                if trees[y_pos][x_index] < *tree_house_height {
                    scores[3] += 1;
                } else {
                    scores[3] += 1;
                    break;
                }
            }
            let scenic_score = scores.iter().product::<u32>();

            if scenic_score > high_score {
                high_score = scenic_score
            }
        }
    }
    high_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "21");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
