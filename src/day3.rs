use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(3);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn is_multiple_instruction(substring: &str) -> u64 {
    // Check if the first part of the substring is 'mul('
    if &substring[..4] != "mul(" {
        return 0;
    }

    let chars = substring.chars();
    let mut comma_index: usize = 0;
    let mut final_index: usize = 0;

    for (i, c) in chars.enumerate() {
        if i < 4 {
            continue;
        }
        if c == ')' {
            final_index = i;
            break;
        } else if c == ',' {
            // If we have already found a comma then we have an issue
            if comma_index != 0 {
                return 0;
            }

            comma_index = i;
            continue;
        } else if !c.is_numeric() {
            return 0;
        }
    }

    let left_num: u64 = substring[4..comma_index].parse().unwrap_or_default();
    let right_num: u64 = substring[comma_index + 1..final_index]
        .parse()
        .unwrap_or_default();

    left_num * right_num
}

fn part_a(input: &str) -> String {
    let mut total_multiplication: u64 = 0;
    for line in input.lines() {
        // Iterate through each line by character
        for num in 0..line.len() - 7 {
            total_multiplication += is_multiple_instruction(&line[num..]);
        }
    }
    format!("{}", total_multiplication)
}

fn part_b(input: &str) -> String {
    let mut total_multiplication: u64 = 0;
    let mut enabled: bool = true;
    for line in input.lines() {
        for num in 0..line.len() - 7 {
            if enabled {
                // Check for multiplication
                total_multiplication += is_multiple_instruction(&line[num..]);

                // Check for don't instruction
                if &line[num..num + 7] == "don't()" {
                    enabled = false;
                }
            } else {
                // Check for a do instruction
                if &line[num..num + 4] == "do()" {
                    enabled = true;
                }
            }
        }
    }
    format!("{}", total_multiplication)
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected_answer: String = String::from("161");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let expected_answer: String = String::from("48");
        assert_eq!(expected_answer, part_b(input))
    }
}
