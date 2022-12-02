pub fn process_part1(input: &str) -> String {
    let result = input.split("\n\n");
    let end_result = result
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|item| match item.parse::<u32>() {
                    Ok(number) => number,
                    Err(_error) => 0,
                })
                .sum::<u32>()
        })
        .max()
        .unwrap();
    end_result.to_string()
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

    const INPUT: &str = "1000
2000
3000
4000
5000
6000
7000
8000
9000
10000";

    #[test]
    fn it_works() {
        let result = process_part1(INPUT);
        assert_eq!(result, "24000");
    }

    #[test]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "45000");
    }
}
