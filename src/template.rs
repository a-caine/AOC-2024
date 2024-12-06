use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(1);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn part_a(input: &str) -> String {
    String::from("Implement me")
}

fn part_b(input: &str) -> String {
    String::from("Implement me")
}

#[cfg(test)]
mod test_dayX {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "";
        let expected_answer: String = String::from("");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "";
        let expected_answer: String = String::from("");
        assert_eq!(expected_answer, part_b(input))
    }
}
