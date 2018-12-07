use std::collections::HashMap;

struct TaskQueue {
    prerequisites: HashMap<char, Vec<char>>
}

impl TaskQueue {
    fn new(input: &str) -> TaskQueue {
        let mut prerequisites = HashMap::<char, Vec<char>>::new();

        let re = regex::Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
        for (prerequisite, step) in input.lines().map(|l| {
            let cap = re.captures(l).unwrap();
            (
                cap.get(1).unwrap().as_str().chars().next().unwrap(),
                cap.get(2).unwrap().as_str().chars().next().unwrap()
            )
        }) {
            prerequisites.entry(step)
                .and_modify(|v| v.push(prerequisite))
                .or_insert(vec![prerequisite]);
            prerequisites.entry(prerequisite).or_insert(vec![]);
        }

        TaskQueue { prerequisites }
    }

    fn pop_task(&mut self) -> Option<char> {
        if let Some(first_leaf) = self.prerequisites.iter()
            .filter(|(_, pqs)| pqs.len() == 0)
            .min_by_key(|(step, _)| *step)
            .map(|(step, _)| *step) {
                self.prerequisites.retain(|step, _| *step != first_leaf);
                Some(first_leaf)
            } else {
                None
            }
    }

    fn finish_task(&mut self, task: char) {
        self.prerequisites.iter_mut()
            .for_each(|(_, pqs)| pqs.retain(|pq| *pq != task));
    }

    fn is_done(&self) -> bool {
        self.prerequisites.len() == 0
    }
}

pub fn part1(input: &str) -> (String, ()) {
    let mut tq = TaskQueue::new(input);

    let mut order = String::new();
    while let Some(next_task) = tq.pop_task() {
        tq.finish_task(next_task);
        order.push(next_task);
    }

    (order, ())
}

#[derive(Debug)]
enum Worker {
    Working(char, usize),
    Idle
}

pub fn time_for_task(task: char) -> usize {
    return (task as usize) - 65 + 60;
}

pub fn part2(input: &str, _: ()) -> String {
    let mut tq = TaskQueue::new(input);

    let mut workers = Vec::new();
    (0..5).for_each(|_| workers.push(Worker::Idle));

    let mut order = String::new();
    let mut step = 0;
    while !tq.is_done() || workers.iter().any(|w| if let Worker::Idle = w { false } else { true }) {
        for worker in workers.iter_mut() {
            *worker = match worker {
                Worker::Working(cur_task, cur_time) =>
                    if *cur_time < time_for_task(*cur_task) {
                        Worker::Working(*cur_task, *cur_time + 1)
                    } else {
                        tq.finish_task(*cur_task);
                        order.push(*cur_task);
                        if let Some(new_task) = tq.pop_task() {
                            Worker::Working(new_task, 0)
                        } else {
                            Worker::Idle
                        }
                    },
                Worker::Idle => if let Some(new_task) = tq.pop_task() {
                    Worker::Working(new_task, 0)
                } else {
                    Worker::Idle
                }
            }
        }

        step += 1;
    }
    (step - 2).to_string()
}