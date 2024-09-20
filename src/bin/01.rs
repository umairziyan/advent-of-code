advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;

    for line in lines {
        sum += do_calc(line);
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut sum: u32 = 0;

    for line in lines {
        let mut first = ' ';
        let mut last = ' ';

        let chars = line.chars().collect::<Vec<_>>();
        for i in 0..chars.len() {
            if chars[i].is_ascii_digit() {
                first = chars[i];
                break;
            }
            let (value, found) = has_prefixes(&line[i..]);
            if found {
                first = value;
                break;
            }
        }

        for i in (0..chars.len()).rev() {
            if chars[i].is_ascii_digit() {
                last = chars[i];
                break;
            }
            let (value, found) = has_prefixes(&line[i..]);
            if found {
                last = value;
                break;
            }
        }

        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        let value = first * 10 + last;
        sum += value;
    }

    Some(sum)
}

fn do_calc(line: &str) -> u32 {
    let mut sum: u32 = 0;

    let line = line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<Vec<_>>();

    if let (Some(first), Some(last)) = (line.first(), line.last()) {
        let first_value = first.to_digit(10).unwrap();
        let last_value = last.to_digit(10).unwrap();
        let value = first_value * 10 + last_value;
        sum += value
    }

    sum
}
fn has_prefixes(line: &str) -> (char, bool) {
    let prefixes = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];

    for (key, value) in prefixes {
        if line.starts_with(key) {
            return (value, true);
        }
    }
    (' ', false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
