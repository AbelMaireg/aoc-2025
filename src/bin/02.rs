advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(",")
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            (start.parse().unwrap(), end.parse().unwrap())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    input
        .iter()
        .map(|(start, end)| {
            let mut sum = 0;
            for n in *start..=*end {
                let digits_size = n.ilog10() + 1;

                let left = n / 10u64.pow(digits_size / 2);
                let right = n % 10u64.pow(digits_size / 2);

                if left == right {
                    sum += n;
                }
            }
            sum
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = parse_input(input);

    input
        .iter()
        .map(|(start, end)| {
            let mut sum = 0;
            for n in *start..=*end {
                let digits: String = n.to_string();
                let size = digits.len();

                'outer: for i in 1..=(size / 2) {
                    let init = &digits[0..i];
                    for j in (i..size).step_by(i) {
                        let end = (j + i).min(size);

                        if &digits[j..end] != init {
                            continue 'outer;
                        }
                    }

                    sum += n;
                    break;
                }
            }
            sum
        })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
