use std::collections::{BTreeSet};

pub fn process_part1(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(4)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        }).unwrap();
    (sequence.0 +1 + 3).to_string()
}

pub fn process_part2(input: &str) -> String {
    let chars = input.chars().collect::<Vec<char>>();
    let sequence = chars
        .windows(14)
        .enumerate()
        .find(|(_i, slice)| {
            let set = slice.iter().collect::<BTreeSet<&char>>();
            slice.len() == set.len()
        }).unwrap();
    (sequence.0 +1 + 13).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // -------------------------------------------------------------------------------------part 1 tests
    #[test]
    fn part1_works1() {
        assert_eq!(process_part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "7");
    }
    
    #[test]
    fn part1_works2() {
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
    }
  
    #[test]
    fn part1_works3() {
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
    }
  
    #[test]
    fn part1_works4() {
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
    }
  
    #[test]
    fn part1_works5() {
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    // -------------------------------------------------------------------------------------part 2 tests
    #[test]
    fn part2_works1() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
    }
    
    #[test]
    fn part2_works2() {
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
    }
  
    #[test]
    fn part2_works3() {
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
    }
  
    #[test]
    fn part2_works4() {
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
    }
  
    #[test]
    fn part2_works5() {
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
