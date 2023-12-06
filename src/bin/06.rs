use advent_of_code::helpers::get_numbers_from_str;
advent_of_code::solution!(6);

fn get_distance(held_time: u64, total_time: u64) -> u64 {
    (total_time - held_time) * held_time
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let limits = get_numbers_from_str(lines.next().unwrap().split_once(":").unwrap().1);
    let records = get_numbers_from_str(lines.next().unwrap().split_once(":").unwrap().1);

    println!("{:?} {:?}", limits, records);
    Some(
        limits
            .iter()
            .zip(records.iter())
            .fold(1, |acc, (limit, record)| {
                // Find the index for which the f(idx) > record
                let mut test_idx = 0;
                let mut test_distance = 0;
                while test_distance <= *record {
                    test_distance = get_distance(test_idx, *limit);
                    test_idx = test_idx + 1;
                }
                test_idx = test_idx - 1;

                println!(
                    "First index: {}, n-count {}",
                    test_idx,
                    (*limit + 1 - 2 * test_idx)
                );
                acc * (*limit + 1 - 2 * test_idx)
            }),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
