pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn process_part1(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    let result: i32 = std::iter::zip(left,right)
        .map(|(l,r)| (l-r).abs())
        .sum();


    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<usize>().unwrap());
        right.push(items.next().unwrap().parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();

    let result: usize = left.iter().map(|number| {
        number
            * right
            .iter()
            .filter(|r| &number == r)
            .count()
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = process_part1(input);
        assert_eq!(result, "11");
    }

    #[test]
    fn it_works_2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = process_part2(input);
        assert_eq!(result, "11");
    }
}
