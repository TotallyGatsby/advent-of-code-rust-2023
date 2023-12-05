use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

// Takes a string of numbers, and returns them as a vector of u32s
// e.g. " 42 17 24 13" -> [42, 17, 24, 13]
fn get_numbers_from_str(input: &str) -> Vec<u32> {
    input
        .trim()
        .split(" ")
        .filter_map(|word| word.parse::<u32>().ok())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines() // For each line
            // Remove the "Card XYZ: ", since it is useless
            .map(|line| line.rsplit_once(":").unwrap().1)
            .fold(0, |acc, line| {
                // Get a vector that is two vectors of our numbers, e.g [[1, 2, 3], [10, 11, 12]]
                let numbers = line
                    .split("|")
                    .map(get_numbers_from_str)
                    .collect::<Vec<_>>();

                // Construct two hashsets, so we can intersect them later
                let winnings: HashSet<u32> = HashSet::from_iter(numbers[0].iter().cloned());
                let draws: HashSet<u32> = HashSet::from_iter(numbers[1].iter().cloned());

                // The intersection returns all elements in both Sets, but we only need the count
                let matches: u32 = winnings.intersection(&draws).count().try_into().unwrap();

                if matches == 0 {
                    return acc;
                }

                acc + 2_u32.pow(matches - 1)
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    // Key/Value store for Card counts [card number, count of that card]
    let mut cardcount: HashMap<usize, u32> = HashMap::new();

    Some(
        input
            .lines() // For each line
            // Remove the "Card XYZ: ", since it is useless
            .map(|line| line.rsplit_once(":").unwrap().1)
            .enumerate()
            .fold(0, |acc, (idx, line)| {
                if !cardcount.contains_key(&idx) {
                    cardcount.insert(idx, 1);
                }

                // Get a vector that is two vectors of our numbers, e.g [[1, 2, 3], [10, 11, 12]]
                let numbers = line
                    .split("|")
                    .map(get_numbers_from_str)
                    .collect::<Vec<_>>();

                // Construct two hashsets, so we can intersect them later
                let winnings: HashSet<u32> = HashSet::from_iter(numbers[0].iter().cloned());
                let draws: HashSet<u32> = HashSet::from_iter(numbers[1].iter().cloned());

                // The intersection returns all elements in both Sets, but we only need the count
                let matches: usize = winnings.intersection(&draws).count().try_into().unwrap();

                if matches != 0 {
                    for i in 1..matches + 1 {
                        if !cardcount.contains_key(&(i + idx)) {
                            cardcount.insert(i + idx, 1);
                        }
                        *cardcount.get_mut(&(i + idx)).unwrap() += cardcount[&idx];
                    }
                }

                return acc + cardcount[&idx];
            }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
