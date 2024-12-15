use itertools::Itertools;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn process_part1(input: &str) -> String {
    let mut result: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line.split_whitespace().map(|level_str| level_str.parse::<i32>().unwrap()).collect();

        let all_increasing: bool = levels
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b > a)
            .all(|increasing| increasing == true);

        let all_decreasing: bool = levels
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| b < a)
            .all(|decreasing| decreasing == true);

        if all_increasing != true && all_decreasing != true {
            continue
        }

        let safe: bool = levels
            .iter()
            .tuple_windows::<(_, _)>()
            .map(|(a, b)| (a - b).abs() >= 1 && (a - b).abs() <= 3)
            .all(|safety| safety == true);

        if safe {
            result += 1;
        }
    }

    result.to_string()
}

fn is_good(levels: &Vec<i32>) -> bool {
    // Define your condition for a "good" array here
    levels
        .windows(2)
        .all(|w| (w[1] - w[0]) <= 3 && (w[1] - w[0]) >= 1)
}

pub fn process_part2(input: &str) -> String {
    let mut result: i32 = 0;

    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .map(|level_str| level_str.parse::<i32>().unwrap())
            .collect();

        if is_good(&levels) {
            result += 1;
            continue;
        }

        for i in 0..levels.len() {
            let mut temp = levels.clone();
            temp.remove(i);

            if is_good(&temp) {
                result += 1;
                break;
            }

            let mut temp_rev: Vec<i32> = levels.clone().into_iter().rev().collect();
            temp_rev.remove(i);
            if is_good(&temp_rev) {
                result += 1;
                break;
            }
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = process_part2(input);
        assert_eq!(result, "4");
    }
}
