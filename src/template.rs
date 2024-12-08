use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents();

    let (part_a_result, part_b_result): (String, String) = get_results(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn get_results(input: &str) -> (String, String) {
    (String::from("Implement me"), String::from("Implement me"))
}

#[cfg(test)]
mod test_dayX {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "";
        let expected_answer: String = String::from("");
        assert_eq!(expected_answer, get_results(input).0)
    }

    #[test]
    fn test_part_b() {
        let input: &str = "";
        let expected_answer: String = String::from("");
        assert_eq!(expected_answer, get_results(input).1)
    }
}
