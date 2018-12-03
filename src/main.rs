extern crate console;
extern crate indicatif;
extern crate regex;
extern crate reqwest;

mod day1;
mod day2;
mod day3;

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

    let mut days: Vec<Box<Fn(&str)>> = vec![
        Box::new(|s| run(s, 1, "Chronal Calibration", day1::part1, day1::part2)),
        Box::new(|s| run(s, 2, "Inventory Management System", day2::part1, day2::part2)),
        Box::new(|s| run(s, 3, "No Matter How You Slice It", day3::part1, day3::part2))
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
