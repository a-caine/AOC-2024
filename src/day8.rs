use std::collections::{HashMap, HashSet};

const MAP_SIZE: i32 = 50;

use crate::helpers;

pub fn run() {
    let input: String = helpers::read_file_contents(8);

    let (part_a_result, part_b_result): (String, String) = get_results(&input, MAP_SIZE);

    println!("Answer for part a: {}", part_a_result);
    println!("Answer for part b: {}", part_b_result);
}

fn get_pairs<T: Copy>(elements: &[T]) -> Vec<(T, T)> {
    elements
        .iter()
        .enumerate()
        .flat_map(|(index, &first)| {
            elements[index + 1..]
                .iter()
                .map(move |&second| (first, second))
        })
        .collect()
}

fn get_antinode((pos_a, pos_b): ((i32, i32), (i32, i32)), map_size: i32) -> Vec<(i32, i32)> {
    let mut antinode_positions: Vec<(i32, i32)> = Vec::new();

    let dif: (i32, i32) = (pos_a.0 - pos_b.0, pos_a.1 - pos_b.1);

    let position: (i32, i32) = (pos_b.0 - dif.0, pos_b.1 - dif.1);

    if (position.0 < map_size && position.0 >= 0) && (position.1 < map_size && position.1 >= 0) {
        antinode_positions.push(position);
    }

    let position: (i32, i32) = (pos_a.0 + dif.0, pos_a.1 + dif.1);

    if (position.0 < map_size && position.0 >= 0) && (position.1 < map_size && position.1 >= 0) {
        antinode_positions.push(position);
    }

    antinode_positions
}

fn get_antinodes_updated(
    (pos_a, pos_b): ((i32, i32), (i32, i32)),
    map_size: i32,
) -> Vec<(i32, i32)> {
    let mut antinode_positions: Vec<(i32, i32)> = Vec::new();

    // Find the difference
    let dif: (i32, i32) = (pos_a.0 - pos_b.0, pos_a.1 - pos_b.1);

    // Find all of the antinodes in one direction, starting from pos_a:
    let mut curr_pos: (i32, i32) = pos_a;

    while (curr_pos.0 < map_size && curr_pos.0 >= 0) && (curr_pos.1 < map_size && curr_pos.1 >= 0) {
        antinode_positions.push(curr_pos);

        curr_pos.0 += dif.0;
        curr_pos.1 += dif.1;
    }

    // Then search the other way
    curr_pos = pos_b;
    while (curr_pos.0 < map_size && curr_pos.0 >= 0) && (curr_pos.1 < map_size && curr_pos.1 >= 0) {
        antinode_positions.push(curr_pos);
        curr_pos.0 -= dif.0;
        curr_pos.1 -= dif.1;
    }

    antinode_positions
}

fn get_results(input: &str, map_size: i32) -> (String, String) {
    // From the input list, find all positions of different characters

    let frequency_locations: HashMap<char, Vec<(i32, i32)>> = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(column, c)| (c, (row, column)))
        })
        .filter(|(c, _)| *c != '.')
        .fold(
            HashMap::new(),
            |mut map: HashMap<char, Vec<(i32, i32)>>, (c, pos)| {
                map.entry(c)
                    .or_default()
                    .push((pos.0.try_into().unwrap(), pos.1.try_into().unwrap()));
                map
            },
        );

    let unique_positions: HashSet<(i32, i32)> = frequency_locations
        .iter()
        .flat_map(|(_c, positions)| {
            get_pairs(positions)
                .iter()
                .map(|position_pair| get_antinode(*position_pair, map_size))
                .collect::<Vec<Vec<(i32, i32)>>>()
        })
        .fold(
            HashSet::new(),
            |mut set: HashSet<(i32, i32)>, antinodes: Vec<(i32, i32)>| {
                set.extend(antinodes);
                set
            },
        );

    let unique_positions_new: HashSet<(i32, i32)> = frequency_locations
        .iter()
        .flat_map(|(_c, positions)| {
            get_pairs(positions)
                .iter()
                .map(|position_pair| get_antinodes_updated(*position_pair, map_size))
                .collect::<Vec<Vec<(i32, i32)>>>()
        })
        .fold(
            HashSet::new(),
            |mut set: HashSet<(i32, i32)>, antinodes: Vec<(i32, i32)>| {
                set.extend(antinodes);
                set
            },
        );

    (
        format!("{}", unique_positions.len()),
        format!("{}", unique_positions_new.len()),
    )
}

#[cfg(test)]
mod test_day8 {
    use super::*;

    const MAP_SIZE_TEST: i32 = 12;

    #[test]
    fn test_part_a() {
        let input: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";
        let expected_answer: String = String::from("14");
        assert_eq!(expected_answer, get_results(input, MAP_SIZE_TEST).0)
    }

    #[test]
    fn test_part_b() {
        let input: &str = "............\n........0...\n.....0......\n.......0....\n....0.......\n......A.....\n............\n............\n........A...\n.........A..\n............\n............";
        let expected_answer: String = String::from("34");
        assert_eq!(expected_answer, get_results(input, MAP_SIZE_TEST).1)
    }
}
