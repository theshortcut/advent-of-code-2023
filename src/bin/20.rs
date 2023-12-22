use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

#[derive(Debug, Clone, Copy)]
enum PulseType {
    High,
    Low,
}

#[derive(Debug, Clone, Copy)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Module {
    kind: ModuleType,
    is_on: bool,
    last_pulse: HashMap<String, PulseType>,
    dests: Vec<String>,
    input_count: usize,
}

impl Module {
    fn new(kind: ModuleType, dests: Vec<String>, input_count: usize) -> Self {
        Self {
            kind,
            dests,
            input_count,
            is_on: false,
            last_pulse: HashMap::new(),
        }
    }

    fn receive_pulse(
        &mut self,
        pulse: &PulseType,
        from: String,
    ) -> Option<(PulseType, Vec<String>)> {
        use ModuleType::*;
        use PulseType::*;

        match (self.kind, pulse) {
            (FlipFlop, High) => None,
            (FlipFlop, Low) => {
                self.is_on = !self.is_on;
                let pt = if self.is_on { High } else { Low };
                Some((pt, self.dests.clone()))
            }
            (Conjunction, pt) => {
                self.last_pulse.insert(from, pt.clone());
                let pt = if self
                    .last_pulse
                    .values()
                    .filter(|v| matches!(v, High))
                    .count()
                    == self.input_count
                {
                    Low
                } else {
                    High
                };
                Some((pt.clone(), self.dests.clone()))
            }
            (Broadcast, pt) => Some((pt.clone(), self.dests.clone())),
        }
    }
}

type ModuleMap = HashMap<String, Module>;

fn parse(input: &str) -> ModuleMap {
    input
        .lines()
        .map(|line| {
            let (name, dests) = line.split_once(" -> ").unwrap();
            let (kind, name) = match name.chars().next().unwrap() {
                'b' => (ModuleType::Broadcast, name),
                '%' => (ModuleType::FlipFlop, &name[1..]),
                '&' => (ModuleType::Conjunction, &name[1..]),
                _ => unreachable!(),
            };
            let dests = dests.split(',').map(|s| s.trim().into()).collect();
            let input_count = if matches!(kind, ModuleType::Conjunction) {
                input.lines().filter(|s| s.contains(name)).count() - 1
            } else {
                0
            };
            (name.into(), Module::new(kind, dests, input_count))
        })
        .collect()
}

fn push_button(modules: &mut ModuleMap) -> Vec<(String, PulseType, String)> {
    let mut log = vec![];
    let mut q = VecDeque::new();
    q.push_back((
        String::from("button"),
        PulseType::Low,
        String::from("broadcaster"),
    ));
    while let Some(pulse) = q.pop_front() {
        log.push(pulse.clone());
        let (from, pulse_type, dest) = pulse;
        if dest != "output" {
            if let Some(module) = modules.get_mut(&dest) {
                if let Some((pt, to)) = module.receive_pulse(&pulse_type, from) {
                    let pt = pt.clone();
                    for t in to {
                        q.push_back((dest.clone(), pt, t));
                    }
                }
            }
        }
    }
    log
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut modules = parse(input);
    let (low, high) = (0..1000).flat_map(|_| push_button(&mut modules)).fold(
        (0, 0),
        |acc, (_, pt, _)| match pt {
            PulseType::Low => (acc.0 + 1, acc.1),
            PulseType::High => (acc.0, acc.1 + 1),
        },
    );
    Some(low * high)
}

pub fn least_common_multiple(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = least_common_multiple(&nums[1..]);
    a * b / greatest_common_divisor(a, b)
}

fn greatest_common_divisor(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    greatest_common_divisor(b, a % b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut modules = parse(input);

    let parent = modules
        .iter()
        .find(|(_, v)| v.dests.contains(&"rx".into()))?
        .0;
    let grandparents = modules
        .iter()
        .filter(|(_, v)| v.dests.contains(parent))
        .map(|m| m.0.clone())
        .collect::<Vec<_>>();
    let mut grandparent_pulses = grandparents
        .iter()
        .map(|s| (s.clone(), 0))
        .collect::<HashMap<_, _>>();

    for i in 0..usize::MAX {
        let log = push_button(&mut modules);
        for l in log {
            if grandparents.contains(&l.2) && matches!(l.1, PulseType::Low) {
                grandparent_pulses.insert(l.2.clone(), i + 1);
            }
        }
        if grandparent_pulses.values().all(|&count| count > 0) {
            return Some(least_common_multiple(&grandparent_pulses.values().copied().collect::<Vec<_>>()));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(32));
    }
}
