use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(4);

    let part_a_result: String = part_a(&input);
    let part_b_result: String = part_b(&input);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn get_char(grid: &[Vec<char>], x: i32, y: i32) -> Option<&char> {
    if x < 0 || y < 0 {
        return None;
    }
    let inner_vec: Option<&Vec<char>> = grid.get(usize::try_from(y).unwrap());
    inner_vec?;
    inner_vec.unwrap().get(usize::try_from(x).unwrap())
}

fn check_all_xmas(grid: &[Vec<char>], x: i32, y: i32) -> u32 {
    let mut xmas_count = 0;
    for i in -1..=1 {
        for j in -1..=1 {
            if check_xmas(grid, x, y, (i, j)) {
                xmas_count += 1;
            }
        }
    }
    xmas_count
}

fn check_xmas(grid: &[Vec<char>], x: i32, y: i32, dir: (i32, i32)) -> bool {
    let mut c: Option<&char> = get_char(grid, x + dir.0, y + dir.1);
    if !(c.is_some() && c.unwrap() == &'M') {
        return false;
    }
    c = get_char(grid, x + dir.0 * 2, y + dir.1 * 2);
    if !(c.is_some() && c.unwrap() == &'A') {
        return false;
    }
    c = get_char(grid, x + dir.0 * 3, y + dir.1 * 3);
    if !(c.is_some() && c.unwrap() == &'S') {
        return false;
    }
    true
}

fn check_x_mas(grid: &[Vec<char>], x: i32, y: i32) -> bool {
    // Fetch the top left and top right characters
    let top_left = get_char(grid, x - 1, y - 1);
    let top_right = get_char(grid, x + 1, y - 1);
    let bottom_left = get_char(grid, x - 1, y + 1);
    let bottom_right = get_char(grid, x + 1, y + 1);

    if top_left.is_none() || top_right.is_none() || bottom_left.is_none() || bottom_right.is_none()
    {
        return false;
    }

    if !(top_left.unwrap() == &'S' || top_left.unwrap() == &'M') {
        return false;
    }
    if !(top_right.unwrap() == &'S' || top_right.unwrap() == &'M') {
        return false;
    }
    if !(bottom_left.unwrap() == &'S' || bottom_left.unwrap() == &'M') {
        return false;
    }
    if !(bottom_right.unwrap() == &'S' || bottom_right.unwrap() == &'M') {
        return false;
    }

    if (top_left.unwrap() == bottom_right.unwrap()) || (top_right.unwrap() == bottom_left.unwrap())
    {
        return false;
    }

    true
}

fn part_a(input: &str) -> String {
    let lines = input.lines();
    let mut character_grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        character_grid.push(line.chars().collect());
    }

    let mut total_xmas: u32 = 0;

    // Iterate through the grid, keeping track of the x and y values
    for y in 0..character_grid.len() {
        for x in 0..character_grid.first().unwrap().len() {
            // Read the character, if it is anything but an 'X' move on
            let fetched_char = get_char(
                &character_grid,
                i32::try_from(x).unwrap(),
                i32::try_from(y).unwrap(),
            );
            if fetched_char.is_none() {
                continue;
            }
            if fetched_char.unwrap() == &'X' {
                total_xmas += check_all_xmas(
                    &character_grid,
                    i32::try_from(x).unwrap(),
                    i32::try_from(y).unwrap(),
                );
            }
        }
    }

    format!("{}", total_xmas)
}

fn part_b(input: &str) -> String {
    let lines = input.lines();
    let mut character_grid: Vec<Vec<char>> = Vec::new();

    for line in lines {
        character_grid.push(line.chars().collect());
    }

    let mut total_x_mas: u32 = 0;

    // Iterate through the grid, keeping track of the x and y values
    for y in 0..character_grid.len() {
        for x in 0..character_grid.first().unwrap().len() {
            // Read the character, if it is anything but an 'X' move on
            let fetched_char = get_char(
                &character_grid,
                i32::try_from(x).unwrap(),
                i32::try_from(y).unwrap(),
            );
            if fetched_char.is_none() {
                continue;
            }
            if fetched_char.unwrap() == &'A'
                && check_x_mas(
                    &character_grid,
                    i32::try_from(x).unwrap(),
                    i32::try_from(y).unwrap(),
                )
            {
                total_x_mas += 1;
            }
        }
    }

    format!("{}", total_x_mas)
}

#[cfg(test)]
mod test_day4 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let expected_answer: String = String::from("18");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";
        let expected_answer: String = String::from("9");
        assert_eq!(expected_answer, part_b(input))
    }
}
