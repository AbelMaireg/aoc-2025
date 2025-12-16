use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

struct Machine {
    pub target: u32,
    pub buttons: Vec<u32>,
    pub joltage: Vec<u64>,
}

impl Machine {
    fn from_input(line: &str) -> Machine {
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut target = 0;
        for (i, c) in parts[0][1..parts[0].len() - 1].chars().enumerate() {
            if c == '#' {
                target |= 1 << i;
            }
        }

        let last_part = parts.last().unwrap();
        let goal_counters = last_part[1..last_part.len() - 1]
            .split(',')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut button_masks = Vec::new();
        for part in &parts[1..parts.len() - 1] {
            let mut mask = 0;
            let inner = if part.starts_with('(') || part.starts_with('{') {
                &part[1..part.len() - 1]
            } else {
                part
            };

            for num_str in inner.split(',') {
                if let Ok(bit) = num_str.parse::<u32>() {
                    mask |= 1 << bit;
                }
            }
            button_masks.push(mask);
        }

        Machine {
            target,
            joltage: goal_counters,
            buttons: button_masks,
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    input.lines().map(Machine::from_input).collect()
}

fn solve_bfs(machine: &Machine) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((0u32, 0));

    let mut visited = HashSet::new();
    visited.insert(0);

    while let Some((curr, steps)) = queue.pop_front() {
        if curr == machine.target {
            return steps;
        }

        for &b_mask in &machine.buttons {
            let nxt = curr ^ b_mask;
            if visited.insert(nxt) {
                queue.push_back((nxt, steps + 1));
            }
        }
    }
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse(input);
    let mut total_steps = 0;

    for machine in &machines {
        let steps = solve_bfs(machine);
        total_steps += steps;
    }

    Some(total_steps)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
