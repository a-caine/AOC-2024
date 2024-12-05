use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please specify the day you wish to run.");
        return;
    }
    let day: &String = &args[1];

    println!("Running solution for day {day}");
    match day.as_str() {
        "1" => day1::run(),
        "2" => day2::run(),
        "3" => day3::run(),
        "4" => day4::run(),
        "5" => day5::run(),
        _ => println!("Soltuion for day {day} not implemented yet"),
    }
}

// Modules for solutions
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod helpers;
