advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy)]
enum Op {
    Set(usize),
    Sub,
}

type Boxes<'a> = Vec<Vec<(&'a str, Op)>>;

fn hash(str: &str) -> usize {
    str.as_bytes()
        .iter()
        .fold(0, |acc, &b| ((acc + b as usize) * 17) % 256)
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .next()
        .map(|line| line.split(',').map(hash).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .lines()
        .next()
        .and_then(|l| {
            l.split(',')
                .try_fold(vec![vec![]; 256], |mut acc: Boxes, seq| {
                    let pos = seq.chars().position(|p| p == '=' || p == '-')?;
                    let op = match seq.chars().nth(pos)? {
                        '=' => Op::Set(seq[(pos + 1)..].parse().ok()?),
                        '-' => Op::Sub,
                        _ => unreachable!(),
                    };

                    let lens = (&seq[0..pos], op);
                    let lens_hash = hash(lens.0);
                    let entry = acc.get_mut(lens_hash)?;
                    let pos = entry.iter().position(|x| x.0 == lens.0);

                    match lens.1 {
                        Op::Sub => {
                            if let Some(pos) = pos {
                                entry.remove(pos);
                            }
                        }
                        Op::Set(_) => match pos {
                            Some(pos) => {
                                *entry.get_mut(pos).unwrap() = lens;
                            }
                            None => {
                                entry.push(lens);
                            }
                        },
                    }

                    Some(acc)
                })
        })
        .map(|boxes| {
            boxes
                .into_iter()
                .enumerate()
                .map(|(key, vals)| {
                    let score = vals.into_iter().enumerate().fold(0, |acc, (i, lens)| {
                        let focal_length = match lens.1 {
                            Op::Set(x) => x,
                            _ => unreachable!(),
                        };
                        acc + ((i + 1) * focal_length)
                    });

                    (1 + key) * score
                })
                .sum()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
