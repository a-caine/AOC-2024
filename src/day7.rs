use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(7);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn check_valid(target: u64, curr_total: u64, remaining_nums: &[u64]) -> bool {
    // Handle the base cases first
    if curr_total > target {
        return false;
    }

    if remaining_nums.is_empty() {
        return target == curr_total;
    }

    check_valid(target, curr_total + remaining_nums[0], &remaining_nums[1..])
        || check_valid(target, curr_total * remaining_nums[0], &remaining_nums[1..])
}

fn part_a(input: &str) -> String {
    let parsed_inputs: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line: &str| line.split(':').collect())
        .map(|split_string: Vec<&str>| {
            (
                split_string[0].parse().unwrap(),
                split_string[1][1..]
                    .split(' ')
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let total_correct: u64 = parsed_inputs
        .iter()
        .filter(|(target, num_list)| check_valid(*target, num_list[0], &num_list[1..]))
        .map(|(target, _)| target)
        .sum();

    format!("{}", total_correct)
}

fn concat(left: u64, right: u64) -> u64 {
    left * 10_u64.pow(right.ilog10() + 1) + right
}

fn check_valid_with_concat(target: u64, curr_total: u64, remaining_nums: &[u64]) -> bool {
    if curr_total > target {
        return false;
    }

    if remaining_nums.is_empty() {
        return target == curr_total;
    }

    check_valid_with_concat(
        target,
        concat(curr_total, remaining_nums[0]),
        &remaining_nums[1..],
    ) || check_valid_with_concat(target, curr_total * remaining_nums[0], &remaining_nums[1..])
        || check_valid_with_concat(target, curr_total + remaining_nums[0], &remaining_nums[1..])
}

fn part_b(input: &str) -> String {
    let parsed_inputs: Vec<(u64, Vec<u64>)> = input
        .lines()
        .map(|line: &str| line.split(':').collect())
        .map(|split_string: Vec<&str>| {
            (
                split_string[0].parse().unwrap(),
                split_string[1][1..]
                    .split(' ')
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        })
        .collect();

    let total_correct: u64 = parsed_inputs
        .iter()
        .filter(|(target, num_list)| check_valid_with_concat(*target, num_list[0], &num_list[1..]))
        .map(|(target, _)| target)
        .sum();

    format!("{}", total_correct)
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let expected_answer: String = String::from("3749");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "190: 10 19\n3267: 81 40 27\n83: 17 5\n156: 15 6\n7290: 6 8 6 15\n161011: 16 10 13\n192: 17 8 14\n21037: 9 7 18 13\n292: 11 6 16 20";
        let expected_answer: String = String::from("11387");
        assert_eq!(expected_answer, part_b(input))
    }

    #[test]
    fn test_concat() {
        assert_eq!(1010, concat(10, 10));
        assert_eq!(1234, concat(123, 4));
        assert_eq!(concat(9, 99), concat(99, 9));
        assert_eq!(123456123456, concat(123456, 123456));
    }
}
