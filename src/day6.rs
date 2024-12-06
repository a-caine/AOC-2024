use std::collections::HashSet;

use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(6);

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

fn update_position(
    grid: &[Vec<char>],
    position: (i32, i32),
    direction: (i32, i32),
    new_obstacle: Option<(i32, i32)>,
) -> Option<((i32, i32), (i32, i32))> {
    let new_dir = match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        dir => dir,
    };

    if new_obstacle.is_some()
        && get_char(grid, position.0 + direction.0, position.1 + direction.1).is_some()
        && (position.0 + direction.0, position.1 + direction.1) == new_obstacle.unwrap()
    {
        return Some((position, new_dir));
    }

    match get_char(grid, position.0 + direction.0, position.1 + direction.1) {
        None => None,
        Some('#') => Some((position, new_dir)),
        Some(_) => Some((
            (position.0 + direction.0, position.1 + direction.1),
            direction,
        )),
    }
}

fn part_a(input: &str) -> String {
    let char_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    // Find the start position
    let mut pos = (0, 0);
    'initial_pos: for y in 0..char_grid.len() {
        for x in 0..char_grid.first().unwrap().len() {
            if char_grid[y][x] == '^' {
                pos = (i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                break 'initial_pos;
            }
        }
    }

    let mut position_set: HashSet<(i32, i32)> = HashSet::new();
    let mut updates: Option<((i32, i32), (i32, i32))> = Some((pos, (0, -1)));
    while updates.is_some() {
        // Record the position in our set
        position_set.insert(updates.unwrap().0);
        updates = update_position(&char_grid, updates.unwrap().0, updates.unwrap().1, None);
    }

    format!("{}", position_set.len())
}

fn test_loop(
    grid: &[Vec<char>],
    pos: (i32, i32),
    dir: (i32, i32),
    obstacle_position: (i32, i32),
) -> bool {
    let mut position_and_direction_set: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    let mut updates: Option<((i32, i32), (i32, i32))> = Some((pos, dir));

    while updates.is_some() {
        // Check if the EXACT value has already been seen (position AND direction)
        if !position_and_direction_set.insert(updates.unwrap()) {
            return true;
        }
        updates = update_position(
            grid,
            updates.unwrap().0,
            updates.unwrap().1,
            Some(obstacle_position),
        );
    }

    false
}

fn part_b(input: &str) -> String {
    let char_grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    // Find the start position
    let mut pos = (0, 0);
    'initial_pos: for y in 0..char_grid.len() {
        for x in 0..char_grid.first().unwrap().len() {
            if char_grid[y][x] == '^' {
                pos = (i32::try_from(x).unwrap(), i32::try_from(y).unwrap());
                break 'initial_pos;
            }
        }
    }

    let mut obstacle_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut updates: Option<((i32, i32), (i32, i32))> = Some((pos, (0, -1)));

    while updates.is_some() {
        let curr_pos = updates.unwrap().0;
        let curr_dir = updates.unwrap().1;
        let obstacle_position = (curr_pos.0 + curr_dir.0, curr_pos.1 + curr_dir.1);
        if test_loop(&char_grid, pos, (0, -1), obstacle_position) {
            obstacle_positions.insert(obstacle_position);
        }
        updates = update_position(&char_grid, curr_pos, curr_dir, None);
    }

    obstacle_positions.remove(&pos);

    format!("{}", obstacle_positions.len())
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    #[test]
    fn test_part_a() {
        let input: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let expected_answer: String = String::from("41");
        assert_eq!(expected_answer, part_a(input))
    }

    #[test]
    fn test_part_b() {
        let input: &str = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...";
        let expected_answer: String = String::from("6");
        assert_eq!(expected_answer, part_b(input));

        let input: &str = "...#......\n..........\n....#.....\n........#.\n..#.......\n.........#\n.#........\n....^..#..\n..#.......\n........#.";
        let expected_answer: String = String::from("7");
        assert_eq!(expected_answer, part_b(input));
    }
}
