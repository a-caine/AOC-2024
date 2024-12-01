use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(1);

    let part_a_result: u32 = part_a(&input);
    let part_b_result: u128 = part_b(&input);

    println!("The total distance is {}", part_a_result);
    println!("Similarity score is {}", part_b_result);
}

fn part_a(input: &str) -> u32 {
    // Split the input by lines, then build up two arrays by splitting by whitespace
    let (mut left, mut right) = get_lists(input);
    // Sort the arrays from smallest to largest
    left.sort();
    right.sort();

    // Iterate through both of the vectors
    let it = left.iter().zip(right.iter());

    let mut diff: u32 = 0;

    for (x, y) in it {
        diff += u32::try_from((x - y).abs()).ok().unwrap();
    }

    diff
}

fn part_b(input: &str) -> u128 {
    let (left, right): (Vec<i32>, Vec<i32>) = get_lists(input);

    // Store the total similarity score
    let mut similarity_score: u128 = 0;

    // Iterate through the left array and find instances of the number in the right array
    for num in left {
        let mut matches: u128 = 0;

        for num_right in &right {
            if num == *num_right {
                matches += 1;
            }
        }

        similarity_score += matches * u128::try_from(num).ok().unwrap();
    }

    similarity_score
}

fn get_lists(input: &str) -> (Vec<i32>, Vec<i32>) {
    let lines = input.lines();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let mut numbers = line.split_whitespace();
        left.push(numbers.next().unwrap().parse().unwrap());
        right.push(numbers.next().unwrap().parse().unwrap());
    }
    (left, right)
}

#[cfg(test)]
mod test_day1 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let expected_distance: u32 = 11;
        assert_eq!(expected_distance, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        let expected_similarity: u128 = 31;
        assert_eq!(expected_similarity, part_b(input))
    }
}
