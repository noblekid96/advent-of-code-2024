use regex::Regex;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn process_part1(input: &str) -> String {
    let mut result: usize = 0;

    let pattern = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)";

    // Compile the regex
    let re = Regex::new(pattern).unwrap();

    // Find all matches with capture groups
    for cap in re.captures_iter(input) {
        let num1: usize = cap[1].parse().unwrap();
        let num2: usize = cap[2].parse().unwrap();
        let product = num1 * num2;

        result += product;
    }

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result: usize = 0;

    let pattern = r"mul\(([0-9]{1,3}),([0-9]{1,3})\)|(don't\(\))|(do\(\))";

    // Compile the regex
    let re = Regex::new(pattern).unwrap();

    let matches: Vec<_> = re.captures_iter(input).collect();

    // Find all matches with capture groups
    let mut current = 0;
    for cap in matches.into_iter().rev() {
        // dbg!(&cap);
        if let Some(match_text) = cap.get(0) {
            if let (Some(num1), Some(num2)) = (cap.get(1), cap.get(2)) {
                let num1: usize = num1.as_str().parse().unwrap();
                let num2: usize = num2.as_str().parse().unwrap();
                current += num1 * num2;
            } else if match_text.as_str() == "don't()" {
                current = 0;
            } else if match_text.as_str() == "do()" {
                result += current;
                current = 0;
            }
        }
    }

    result += current;

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = process_part1(input);
        assert_eq!(result, "2");
    }

    #[test]
    fn it_works_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = process_part2(input);
        assert_eq!(result, "48");
    }
}
