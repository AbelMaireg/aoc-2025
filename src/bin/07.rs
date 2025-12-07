use std::collections::HashMap;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let start = input.lines().next()?.find('S')?;
    let spliters: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect()
        })
        .filter(|v: &Vec<usize>| !v.is_empty())
        .collect();

    let mut untouched = 0;

    let _ = spliters
        .iter()
        .fold(HashMap::from([(start, 1u64)]), |mut acc, spliters| {
            for s in spliters {
                if let Some(&count) = acc.get(s)
                    && count == 0
                {
                    untouched += 1;
                    continue;
                }
                let count = *acc.get(s).unwrap_or(&0);
                *acc.entry(s + 1).or_default() += count;
                *acc.entry(s - 1).or_default() += count;
                *acc.get_mut(s).unwrap() = 0;
            }
            acc
        });

    Some(spliters.iter().flatten().count() as u64 - untouched)
}

pub fn part_two(input: &str) -> Option<u64> {
    let start = input.lines().next()?.find('S')?;
    let spliters: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                .collect()
        })
        .filter(|v: &Vec<usize>| !v.is_empty())
        .collect();

    let mut untouched = 0;

    spliters
        .iter()
        .fold(HashMap::from([(start, 1u64)]), |mut acc, spliters| {
            for s in spliters {
                if let Some(&count) = acc.get(s)
                    && count == 0
                {
                    untouched += 1;
                    continue;
                }
                let count = *acc.get(s).unwrap_or(&0);
                *acc.entry(s + 1).or_default() += count;
                *acc.entry(s - 1).or_default() += count;
                *acc.get_mut(s).unwrap() = 0;
            }
            acc
        })
        .into_values()
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
