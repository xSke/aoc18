mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

fn run<T>(aoc_session: &str, day_num: usize, name: &'static str, part_1_fn: impl Fn(&str) -> (String, T), part_2_fn: impl Fn(&str, T) -> String) {
    let input_file_path = format!("inputs/{}.txt", day_num);

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(indicatif::ProgressStyle::default_spinner().template("{elapsed:>3} {spinner} {prefix:<35!.cyan} {msg}"));
    pb.set_prefix(&format!("Day {}: {}", day_num, name));
    pb.enable_steady_tick(100);

    let input = if Path::new(&input_file_path).exists() {
        let mut input = String::new();
        File::open(input_file_path).unwrap().read_to_string(&mut input).unwrap();
        input
    } else {
        pb.set_message("fetching puzzle input...");

        let client = reqwest::Client::new();
        let mut response = client.get(&format!("https://adventofcode.com/2018/day/{}/input", day_num))
            .header("Cookie", format!("session={}", aoc_session))
            .send().unwrap();

        let input = response.text().unwrap();
        fs::create_dir_all("inputs/").unwrap();
        File::create(input_file_path).unwrap().write_all(input.as_bytes()).unwrap();
        input
    };

    pb.set_message(&format!("1: {}", console::style("...").red()));
    let (part_1, data) = part_1_fn(&input);

    pb.set_message(&format!("1: {}, 2: {}", console::style(&part_1).green(), console::style("...").red()));
    let part_2 = part_2_fn(&input, data);

    pb.finish_with_message(&format!("1: {}, 2: {}", console::style(&part_1).green(), console::style(&part_2).green()));
}

fn main() {
    let session = std::env::var("AOC_SESSION").expect("Expected AoC session in $AOC_SESSION");

    println!(" --- \u{1f384} \u{2728} Advent of Code 2018 \u{2728} \u{1f384} --- ");

    let mut days: Vec<Box<dyn Fn(&str)>> = vec![
        Box::new(|s| run(s, 1, "Chronal Calibration", day1::part1, day1::part2)),
        Box::new(|s| run(s, 2, "Inventory Management System", day2::part1, day2::part2)),
        Box::new(|s| run(s, 3, "No Matter How You Slice It", day3::part1, day3::part2)),
        Box::new(|s| run(s, 4, "Repose Record", day4::part1, day4::part2)),
        Box::new(|s| run(s, 5, "Alchemical Reduction", day5::part1, day5::part2)),
        Box::new(|s| run(s, 6, "Chronal Coordinates", day6::part1, day6::part2)),
        Box::new(|s| run(s, 7, "The Sum of Its Parts", day7::part1, day7::part2)),
        Box::new(|s| run(s, 8, "Memory Maneuver", day8::part1, day8::part2)),
        Box::new(|s| run(s, 9, "Marble Mania", day9::part1, day9::part2)),
        Box::new(|s| run(s, 10, "The Stars Align", day10::part1, day10::part2)),
        Box::new(|s| run(s, 11, "Chronal Charge", day11::part1, day11::part2)),
        Box::new(|s| run(s, 12, "Subterranean Sustainability", day12::part1, day12::part2)),
        Box::new(|s| run(s, 13, "Mine Cart Madness", day13::part1, day13::part2)),
        Box::new(|s| run(s, 14, "Chocolate Charts", day14::part1, day14::part2)),
        Box::new(|s| run(s, 15, "Beverage Bandits", day15::part1, day15::part2)),
        Box::new(|s| run(s, 16, "Chronal Classification", day16::part1, day16::part2))
    ];

    if let Some(day) = std::env::args().nth(1) {
        let day_num: usize = day.parse::<usize>().unwrap();

        if day_num < 1 || day_num > days.len() {
            eprintln!("Day {} not found.", day_num);
            return;
        }
        (days.get_mut(day_num - 1).unwrap())(&session);
    } else {
        for day in days.iter_mut() {
            (day)(&session);
        }
    }
}
