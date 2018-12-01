extern crate console;
extern crate indicatif;
extern crate reqwest;

mod day1;

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

trait Day {
    fn name(&self) -> &str;

    fn part_1(&mut self, _: &str) -> String { "(none)".to_string() }

    fn part_2(&mut self, _: &str) -> String { "(none)".to_string() }
}

fn run(day_num: usize, day: &mut Day, aoc_session: &str) {
    let input_file_path = format!("inputs/{}.txt", day_num);

    let pb = indicatif::ProgressBar::new_spinner();
    pb.set_style(indicatif::ProgressStyle::default_spinner().template("{elapsed:>3} {spinner} {prefix:<35!.cyan} {msg}"));
    pb.set_prefix(&format!("Day {}: {}", day_num, day.name()));
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
    let part_1 = day.part_1(&input);

    pb.set_message(&format!("1: {}, 2: {}", console::style(&part_1).green(), console::style("...").red()));
    let part_2 = day.part_2(&input);

    pb.finish_with_message(&format!("1: {}, 2: {}", console::style(&part_1).green(), console::style(&part_2).green()));
}

fn main() {
    let session = std::env::var("AOC_SESSION").expect("Expected AoC session in $AOC_SESSION");

    println!(" --- \u{1f384} \u{2728} Advent of Code 2018 \u{2728} \u{1f384} --- ");

    let mut days = [
        Box::new(day1::Day1)
    ];

    if let Some(day) = std::env::args().nth(1) {
        let day_num: usize = day.parse::<usize>().unwrap();

        if day_num < 1 || day_num > days.len() {
            eprintln!("Day {} not found.", day_num);
            return;
        }
        run(day_num, days.get_mut(day_num - 1).unwrap().as_mut(), &session);
    } else {
        for (i, day) in days.iter_mut().enumerate() {
            run(i + 1, day.as_mut(), &session);
        }
    }
}
