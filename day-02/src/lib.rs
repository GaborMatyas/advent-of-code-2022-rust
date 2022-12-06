use std::{str::FromStr, cmp::Ordering};

enum Move {
    Rock = 1,
    Paper =2,
    Scissors = 3
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string())
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        if self == Move::Scissors && other ==Move::Rock {
            Some(Ordering::Greater)
        } else if self == Move::Rock && other == Move::Scissors {
            Some(Ordering::Less)
        } else {
            Some(*self as u8).cmp(&(*other as u8))
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s|s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => {
                    3 + moves[1] as u32
                }
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => {
                    0 + moves[1] as u32
                }
                None => {
                    
                }
            }
    })
    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result2 = input
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| match item.parse::<u32>() {
                    Ok(number) => number,
                    Err(_error) => 0,
                })
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    result2.sort_by(|a, b| b.cmp(a));
    let sum: u32 = result2.iter().take(3).sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y
    B X
    C Z";


    #[test]
    fn part1_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
