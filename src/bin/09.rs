use std::vec;

use advent_of_code::helpers::get_numbers_from_str;
use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let rows: Vec<Vec<i32>> = input
        .lines()
        .map(|line| get_numbers_from_str(line))
        .collect_vec();

    Some(rows.iter().fold(0, |acc, row| {
        let mut history_levels: Vec<Vec<i32>> = vec![];

        history_levels.push(row.to_vec());
        while !history_levels.last().unwrap().iter().all(|item| *item == 0) {
            let cur_vec = history_levels.last().unwrap();
            history_levels.push(cur_vec.iter().tuple_windows().map(|(a, b)| b - a).collect());
        }

        let new_val = history_levels
            .iter()
            .enumerate()
            .rev()
            .fold(0, |val, (idx, _)| {
                if idx == 0 {
                    return val;
                }
                val + history_levels[idx - 1].last().unwrap()
            });

        acc + new_val
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let rows: Vec<Vec<i32>> = input
        .lines()
        .map(|line| get_numbers_from_str(line))
        .collect_vec();

    Some(rows.iter().fold(0, |acc, row| {
        let mut history_levels: Vec<Vec<i32>> = vec![];

        history_levels.push(row.to_vec());
        while !history_levels.last().unwrap().iter().all(|item| *item == 0) {
            let cur_vec = history_levels.last().unwrap();
            history_levels.push(cur_vec.iter().tuple_windows().map(|(a, b)| b - a).collect());
        }

        let new_val = history_levels
            .iter()
            .enumerate()
            .rev()
            .fold(0, |val, (idx, _)| {
                if idx == 0 {
                    return val;
                }
                history_levels[idx - 1].first().unwrap() - val
            });

        acc + new_val
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
