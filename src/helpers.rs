// public functions for helping with advent of code

use std::fs;

pub fn read_file_contents(day: u8) -> String {
    let file_contents: Result<String, std::io::Error> =
        fs::read_to_string(format!("src/inputs/day{day}.txt"));

    match file_contents {
        Ok(contents) => contents,
        Err(error) => panic!("Problem opening the file 'src/inputs/day{day}.txt': {error:?}"),
    }
}
