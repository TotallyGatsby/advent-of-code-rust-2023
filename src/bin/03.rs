use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let oneline = input.replace("\n", "");
    let mut number_accumulator = 0u32;
    let mut found_symbol = false;
    let mut symbol_adjacencies = HashSet::new();

    // Lazy two phase approach
    // Sweep the input and identify all cells adjacent to symbols
    // Use the cell's 1D position as a key
    oneline.chars().enumerate().for_each(|(pos, c)| {
        if c.is_numeric() || c == '.' {
            return;
        }
        if pos > width {
            if pos % width != 0 {
                symbol_adjacencies.insert(pos - 1 - width);
            }
            symbol_adjacencies.insert(pos - width);
            if pos % width != width - 1 {
                symbol_adjacencies.insert(pos + 1 - width);
            }
        }
        if pos > 0 && pos % width != 0 {
            symbol_adjacencies.insert(pos - 1);
        }
        if pos % width != width - 1 {
            symbol_adjacencies.insert(pos + 1);
        }
        if pos % width != 0 {
            symbol_adjacencies.insert(pos - 1 + width);
        }
        symbol_adjacencies.insert(pos + width);
        if pos % width != width - 1 {
            symbol_adjacencies.insert(pos + 1 + width);
        }
    });

    let sum = oneline.chars().enumerate().fold(0, |mut acc, (pos, c)| {
        if pos % width == 0 {
            if found_symbol {
                let new_result = acc + number_accumulator;
                acc = new_result;
            }
            found_symbol = false;
            number_accumulator = 0;
        }
        // Is this cell adjacent to a symbol
        // We want this to latch to 'true' if we find any neighbors are adjacent
        found_symbol |= symbol_adjacencies.contains(&pos) && c.is_numeric();

        if c.is_numeric() {
            // Add the digit and keep processing
            number_accumulator = (number_accumulator * 10) + c.to_digit(10).unwrap();
            acc
        } else {
            // if we're close a symbol, log it and move on
            if found_symbol {
                let mut result = acc;
                if number_accumulator != 0 {
                    result = acc + number_accumulator;
                    number_accumulator = 0;
                }
                found_symbol = false;
                result
            } else {
                number_accumulator = 0;
                acc
            }
        }
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let width = input.lines().next().unwrap().len();
    let oneline = input.replace("\n", "");
    let mut number_accumulator = 0u32;

    let mut gear_indices = HashMap::new();

    let mut gear_index = 0;
    // Lazy two phase approach
    // Sweep through and mark all adjacent cells with their 'gear index'
    // e.g. which gear they are next to
    // Later, we'll record numbers adjacent to gears, and iff 2 numbers are
    // adjacent to that gear, multiply and sum them
    oneline.chars().enumerate().for_each(|(pos, c)| {
        if c != '*' {
            return;
        }

        if pos > width {
            if pos % width != 0 {
                gear_indices.insert(pos - 1 - width, gear_index);
            }
            gear_indices.insert(pos - width, gear_index);
            if pos % width != width - 1 {
                gear_indices.insert(pos + 1 - width, gear_index);
            }
        }
        if pos > 0 && pos % width != 0 {
            gear_indices.insert(pos - 1, gear_index);
        }
        if pos % width != width - 1 {
            gear_indices.insert(pos + 1, gear_index);
        }
        if pos % width != 0 {
            gear_indices.insert(pos - 1 + width, gear_index);
        }
        gear_indices.insert(pos + width, gear_index);
        if pos % width != width - 1 {
            gear_indices.insert(pos + 1 + width, gear_index);
        }

        gear_index = gear_index + 1;
    });

    // gearings is gear index -> vector of adjacent numbers
    let mut gearings = HashMap::new();
    oneline.chars().enumerate().for_each(|(pos, c)| {
        if pos % width == 0 {
            if gear_index >= 0 {
                if !gearings.contains_key(&gear_index) {
                    gearings.insert(gear_index, vec![]);
                }
                gearings
                    .get_mut(&gear_index)
                    .unwrap()
                    .push(number_accumulator);
            }
            gear_index = -1;
            number_accumulator = 0;
        }
        // Is this cell adjacent to a symbol
        // We want this to latch to 'true' if we find any neighbors are adjacent
        if gear_indices.contains_key(&pos) && c.is_numeric() {
            gear_index = gear_indices[&pos];
        }

        if c.is_numeric() {
            // Add the digit and keep processing
            number_accumulator = (number_accumulator * 10) + c.to_digit(10).unwrap();
        } else {
            // if we're close a symbol, log it and move on
            if gear_index >= 0 {
                if number_accumulator != 0 {
                    if !gearings.contains_key(&gear_index) {
                        gearings.insert(gear_index, vec![]);
                    }
                    gearings
                        .get_mut(&gear_index)
                        .unwrap()
                        .push(number_accumulator);
                    number_accumulator = 0;
                }
                gear_index = -1;
            } else {
                number_accumulator = 0;
            }
        }
    });

    Some(gearings.iter().fold(0, |acc, (_, gearing)| {
        if gearing.len() == 2 {
            acc + gearing[0] * gearing[1]
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
