use std::collections::HashMap;

use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(5);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn check_num_is_valid(bad_nums: &[u8], previous_nums: &[u8]) -> Option<usize> {
    for bad_num in bad_nums {
        for (index, num) in previous_nums.iter().enumerate() {
            if bad_num == num {
                return Some(index);
            }
        }
    }

    None
}

fn create_rule_book(input: &str) -> HashMap<u8, Vec<u8>> {
    // Parse the rules into a rule book
    let mut rule_book: HashMap<u8, Vec<u8>> = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            // Break out of the loop once we reach the end of the rules (empty line)
            break;
        }

        // Parse out the numbers
        let before_num: u8 = line[..2].parse().unwrap();
        let after_num: u8 = line[3..5].parse().unwrap();
        let nums = rule_book.entry(before_num).or_default();
        nums.push(after_num);
    }
    rule_book
}

fn part_a(input: &str) -> String {
    let rule_book = create_rule_book(input);

    let mut total_middle_values: u32 = 0;
    let mut found_updates = false;
    'line_iterator: for line in input.lines() {
        // Iterate through until we have found the start of the updates
        if !found_updates {
            if line.is_empty() {
                found_updates = true
            }
            continue;
        }

        let updates: Vec<u8> = line
            .split(',')
            .map(|string| string.parse().unwrap())
            .collect();
        for i in 1..updates.len() {
            let nums = rule_book.get(&updates[i]);
            if nums.is_none() {
                continue;
            }
            if check_num_is_valid(nums.unwrap(), &updates[..i]).is_some() {
                continue 'line_iterator;
            }
        }

        // Find the middle value and add it to the running total
        total_middle_values += u32::from(updates[updates.len() / 2]);
    }

    format!("{}", total_middle_values)
}

fn part_b(input: &str) -> String {
    let rule_book = create_rule_book(input);

    let mut total_middle_values: u32 = 0;
    let mut found_updates = false;
    for line in input.lines() {
        // Iterate through until we have found the start of the updates
        if !found_updates {
            if line.is_empty() {
                found_updates = true
            }
            continue;
        }

        let mut good_input = true;
        let mut modified = true;
        let mut updates: Vec<u8> = line
            .split(',')
            .map(|string| string.parse().unwrap())
            .collect();
        while modified {
            modified = false;
            for i in 1..updates.len() {
                let nums = rule_book.get(&updates[i]);
                if nums.is_none() {
                    continue;
                }
                let index = check_num_is_valid(nums.unwrap(), &updates[..i]);
                if index.is_some() {
                    good_input = false;

                    // Fix the number up by shifting the number that was previous to the right of the current number
                    updates[index.unwrap()..=i].rotate_left(1);
                    modified = true;
                }
            }
        }
        if good_input {
            continue;
        }

        total_middle_values += u32::from(updates[updates.len() / 2]);
    }
    format!("{}", total_middle_values)
}

#[cfg(test)]
mod test_day5 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let expected_answer: String = String::from("143");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let expected_answer: String = String::from("123");
        assert_eq!(expected_answer, part_b(input))
    }
}
