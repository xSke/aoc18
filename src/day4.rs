use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct SleepSpan(isize, isize);

impl SleepSpan {
    fn contains(&self, minute: isize) -> bool {
        self.0 <= minute && self.1 > minute
    }
    
    fn length(&self) -> isize {
        self.1 - self.0
    }
}

#[derive(Debug)]
pub struct Guard(Vec<SleepSpan>);

impl Guard {
    fn total(&self) -> isize {
        self.0.iter().map(|x| x.length()).sum()
    }

    fn spans_containing(&self, minute: usize) -> usize {
        self.0.iter().filter(|x| x.contains(minute as isize)).count()
    }
}

pub fn part1(input: &str) -> (String, HashMap<usize, Guard>) {
    let mut sorted = input.lines().collect::<Vec<_>>();
    sorted.sort_unstable();

    let time_r = Regex::new(r"\[\d{4}-\d{2}-\d{2} (\d{2}):(\d{2})\] (.*)").unwrap();
    let shift_r = Regex::new(r"Guard #(\d+) begins shift").unwrap();

    let mut guards: HashMap<usize, Guard> = HashMap::new();
    let mut current_guard = 0;
    for line in sorted {
        let (hour, minute, rest) = {
            let m = time_r.captures(line).unwrap();
            (
                m.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                m.get(3).unwrap().as_str()
            )
        };
        let minute = if hour == 23 { (minute as isize) - 60 } else { minute as isize };

        if let Some(m) = shift_r.captures(line) {
            current_guard = m.get(1).unwrap().as_str().parse().unwrap();
            guards.entry(current_guard).or_insert(Guard(Vec::new()));
        } else {
            if rest == "falls asleep" {
                guards.get_mut(&current_guard).unwrap().0
                    .push(SleepSpan(minute, minute));
            } else if rest == "wakes up" {
                guards.get_mut(&current_guard).unwrap().0
                    .iter_mut().last().unwrap()
                    .1 = minute;
            }
        }
    }

    let result = {
        // NLL please...
        let sleepiest_guard = guards.iter().max_by_key(|(_, span)| span.total()).unwrap();
        let sleepiest_minute = (0..60).max_by_key(|minute| sleepiest_guard.1.spans_containing(*minute)).unwrap();
        (sleepiest_guard.0 * sleepiest_minute)
    };

    (result.to_string(), guards)
}

pub fn part2(_: &str, guards: HashMap<usize, Guard>) -> String {
    let mut minutes = vec![(0, 0); 60];

    for (guard_id, guard) in guards {
        for minute in 0..60 {
            let (sleep_time, _) = minutes[minute];
            if guard.spans_containing(minute) > sleep_time {
                minutes[minute] = (guard.spans_containing(minute), guard_id);
            }
        }
    }

    let (minute, (_, guard)) = minutes.iter().enumerate().max_by_key(|(_, (min, _))| min).unwrap();
    (minute * guard).to_string()
}