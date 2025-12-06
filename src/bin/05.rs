advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<usize> {
    use Bar::*;
    let (range_input, ids) = input.split_once("\n\n")?;

    let mut rngs = range_input
        .lines()
        .flat_map(Bar::from_range)
        .collect::<Vec<_>>();
    rngs.sort();

    let mut ll: Bar = rngs[0];
    let mut bc = (0, 0);

    let mut ranges = Vec::new();
    for bar in rngs {
        if bc.0 == 0 {
            ll = bar;
        }
        match bar {
            Left(_) => {
                bc.0 += 1;
            }
            Right(_) => {
                bc.1 += 1;
            }
        };
        if bc.0 == bc.1 {
            ranges.push((ll.value(), bar.value()));
            bc.0 = 0;
            bc.1 = 0;
        }
    }

    let ids = ids
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    ids.iter()
        .filter(|&id| {
            for (l, r) in ranges.iter() {
                if id >= l && id <= r {
                    return true;
                }
            }

            false
        })
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    use Bar::*;
    let mut rngs = input
        .split("\n\n")
        .next()?
        .lines()
        .flat_map(Bar::from_range)
        .collect::<Vec<_>>();
    rngs.sort();

    let mut ll: Bar = rngs[0]; // point to Last Left bar
    let mut bc = (0, 0); // accumulate Bound Counts of (left bound, light bound)
    let mut count: u64 = 0; // total Count of covered numbers

    for bar in rngs {
        // Update last left bar
        if bc.0 == 0 {
            ll = bar;
        }

        // Accumulate left and right bound counts
        match bar {
            Left(_) => {
                bc.0 += 1;
            }
            Right(_) => {
                bc.1 += 1;
            }
        };

        if bc.0 == bc.1 {
            count += 1 + bar.value() - ll.value();
            bc.0 = 0;
            bc.1 = 0;
        }
    }

    Some(count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bar {
    Left(u64),
    Right(u64),
}

impl Bar {
    fn from_range(input: &str) -> [Self; 2] {
        let (l, r) = input.split_once('-').unwrap();
        [
            Bar::Left(l.parse().unwrap()),
            Bar::Right(r.parse().unwrap()),
        ]
    }

    fn value(&self) -> u64 {
        match self {
            Bar::Left(v) => *v,
            Bar::Right(v) => *v,
        }
    }
}

impl PartialOrd for Bar {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Ordering Bars such that they can be sorted by their value,
/// and in case of a tie, Left comes before Right.
/// E.g., Left(5) < Right(5)
impl Ord for Bar {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = match self {
            Bar::Left(v) => *v,
            Bar::Right(v) => *v,
        };

        let right = match other {
            Bar::Left(v) => *v,
            Bar::Right(v) => *v,
        };

        // Handle tie case
        if left == right {
            return match (self, other) {
                (Bar::Left(_), Bar::Right(_)) => std::cmp::Ordering::Less,
                (Bar::Right(_), Bar::Left(_)) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            };
        }

        // Normal case
        left.cmp(&right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
