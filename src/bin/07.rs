advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let width = input.lines().next()?.chars().count();
    let mut splits = vec![0; width];
    splits[width / 2] = 1;

    let mut count = 0;
    input
        .lines()
        .step_by(2)
        .enumerate()
        .skip(1)
        .for_each(|(i, line)| {
            let l = width / 2 - i; // left offset
            let r = width / 2 + i; // right offset

            line[l..r].chars().enumerate().for_each(|(s, c)| {
                let pos = s + l;
                if c == '^' && splits[pos] != 0 {
                    let beams = splits[pos];
                    splits[pos + 1] += beams;
                    splits[pos - 1] += beams;
                    splits[pos] = 0;
                    count += 1;
                }
            })
        });

    count.into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let width = input.lines().next()?.chars().count();
    let mut splits = vec![0; width];
    splits[width / 2] = 1;

    let mut count = 1;
    input
        .lines()
        .step_by(2)
        .enumerate()
        .skip(1)
        .for_each(|(i, line)| {
            let l = width / 2 - i; // left offset
            let r = width / 2 + i; // right offset

            line[l..r].chars().enumerate().for_each(|(s, c)| {
                let pos = s + l;
                if c == '^' && splits[pos] != 0 {
                    let beams = splits[pos];
                    splits[pos + 1] += beams;
                    splits[pos - 1] += beams;
                    splits[pos] = 0;
                    count += beams;
                }
            })
        });

    count.into()
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
        assert_eq!(result, Some(40));
    }
}
