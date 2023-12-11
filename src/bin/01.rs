advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |score, value| {
        let numbers: Vec<_> = value.matches(char::is_numeric).collect();
        score
            + (10 * numbers[0].parse::<u32>().unwrap())
            + numbers[numbers.len() - 1].parse::<u32>().unwrap()
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let numeral_input = replace_numbers(input);

    Some(numeral_input.lines().fold(0, |score, value| {
        let numbers: Vec<_> = value.matches(char::is_numeric).collect();
        score
            + (10 * numbers[0].parse::<u32>().unwrap())
            + numbers[numbers.len() - 1].parse::<u32>().unwrap()
    }))
}

// This makes eightwothree into e8ight2wot3hree, finding all the digits in the string, destructively
fn replace_numbers(input: &str) -> String {
    let replacements: Vec<(&str, &str)> = vec![
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "t3hree"),
        ("four", "f4our"),
        ("five", "f5ive"),
        ("six", "s6ix"),
        ("seven", "s7even"),
        ("eight", "e8ight"),
        ("nine", "n9ine"),
    ];
    let mut result: String = input.to_string();

    replacements
        .iter()
        .for_each(|(prefix, replacement)| result = result.replace(prefix, replacement));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        //let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        //assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
