use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(2);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn is_input_safe(numbers: &[i32]) -> bool {
    // Before entering into the loop, get the first number from the list
    let mut prev_num: i32 = numbers[0];
    let mut prev_diff: i32 = 0;

    for num in &numbers[1..] {
        let diff: i32 = num - prev_num;
        if (diff * prev_diff) < 0 || diff == 0 || diff.abs() > 3 {
            return false;
        }

        prev_diff = diff;
        prev_num = *num;
    }

    true
}

fn part_a(input: &str) -> String {
    // Keep track of the number of safe tests
    let mut safe_tests: u32 = 0;
    // Split the input into lines
    let lines = input.lines();
    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();
        if is_input_safe(&numbers[..]) {
            safe_tests += 1
        }
    }

    format!("{}", safe_tests)
}

fn part_b(input: &str) -> String {
    let mut safe_tests: u32 = 0;

    let lines = input.lines();
    for line in lines {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        // Iterate through the items in the second list and remove each of them and test, if any pass, pass otherwise fail
        for item_index in 0..numbers.len() {
            if is_input_safe(&[&numbers[..item_index], &numbers[item_index + 1..]].concat()) {
                safe_tests += 1;
                break;
            }
        }
    }

    format!("{}", safe_tests)
}

#[cfg(test)]
mod test_day2 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let expected_answer: String = String::from("2");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let expected_answer: String = String::from("4");
        assert_eq!(expected_answer, part_b(input))
    }
}
