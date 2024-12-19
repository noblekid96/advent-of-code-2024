use std::{collections::{HashMap, VecDeque, HashSet}, cmp::Ordering};


pub fn leads_to(source: &u32, wanted: &u32, edges: &HashMap<u32, Vec<u32>>) -> bool {
    if let Some(neighbors) = edges.get(source) {
        if neighbors.contains(wanted) {
            return true;
        }

        // for &nbr in neighbors {
        //     if leads_to(&nbr, wanted, edges) {
        //         return true;
        //     }
        // }
    } else {
        panic!("Help la");
    }

    false
}
pub fn is_downstream(source: &u32, wanted: &u32, edges: &HashMap<u32, Vec<u32>>) -> Ordering {
    if leads_to(source, wanted, edges) {
        Ordering::Less // Upstream
    } else if leads_to(wanted, source, edges) {
        Ordering::Greater // Downstream
    } else {
        Ordering::Equal // Neither upstream nor downstream
    }
}
pub fn process_part1(input: &str) -> String {
    let mut result: u32 = 0;

    let mut edges: HashMap<u32, Vec<u32>> = HashMap::new();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = parts[0].
        split_whitespace().
        collect();

    for rule in rules {
        let vertices: Vec<u32> = rule
            .split("|")
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        edges
            .entry(vertices[0])
            .or_insert_with(Vec::new)
            .push(vertices[1]);

        edges
            .entry(vertices[1])
            .or_insert_with(Vec::new);
    }

    let updates: Vec<Vec<u32>> = parts[1]
        .split_whitespace()
        .map(|update|
             update.split(",")
             .map(|item| item.parse::<u32>().unwrap())
             .collect()
        )
        .collect();


    for update in updates {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut valid = true;

        'outer: for &num in &update {
            if let Some(neighbours) = edges.get(&num) {
                for &nbr in neighbours {
                    if visited.contains(&nbr) {
                        valid = false;
                        break 'outer;
                    }
                }
            }
            visited.insert(num);
        }

        if valid {
            result += update[update.len()/2];
        }
    }

    result.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut result: u32 = 0;

    let mut edges: HashMap<u32, Vec<u32>> = HashMap::new();

    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules: Vec<&str> = parts[0].
        split_whitespace().
        collect();

    for rule in rules {
        let vertices: Vec<u32> = rule
            .split("|")
            .map(|item| item.parse::<u32>().unwrap())
            .collect();

        edges
            .entry(vertices[0])
            .or_insert_with(Vec::new)
            .push(vertices[1]);

        edges
            .entry(vertices[1])
            .or_insert_with(Vec::new);
    }

    let updates: Vec<Vec<u32>> = parts[1]
        .split_whitespace()
        .map(|update|
             update.split(",")
             .map(|item| item.parse::<u32>().unwrap())
             .collect()
        )
        .collect();

    for update in updates {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut valid = true;

        'outer: for &num in &update {
            if let Some(neighbours) = edges.get(&num) {
                for &nbr in neighbours {
                    if visited.contains(&nbr) {
                        valid = false;
                        break 'outer;
                    }
                }
            }
            visited.insert(num);
        }

        if !valid {
            let mut update_sorted = update.clone();
            update_sorted.sort_by(|a,b| {
                is_downstream(a,b, &edges)
                // if edges.get(a).unwrap().contains(b) {
                //     Ordering::Less
                // } else {
                //     Ordering::Greater
                // }
            });
            result += update_sorted[update_sorted.len()/2];
        }
    }

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = process_part1(input);
        assert_eq!(result, "143");
    }

    #[test]
    fn it_works_2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = process_part2(input);
        assert_eq!(result, "123");
    }
}
