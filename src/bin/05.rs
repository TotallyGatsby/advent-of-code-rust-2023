use advent_of_code::helpers::get_numbers_from_str;
use itertools::Itertools;
advent_of_code::solution!(5);

#[derive(Debug, Clone, Copy)]
struct Range {
    pub start: u64,
    pub length: u64,
}

// [1-3] // [4-6]
impl Range {
    pub fn intersect(self, other: Range, shift: i64) -> Vec<Range> {
        let self_max = self.start + self.length - 1;
        let other_max = other.start + other.length - 1;

        // Disjoint sets, no transform needed
        if self_max < other.start || self.start > other_max {
            return vec![];
        }
        // If self is entirely within other, shift it entirely to match the other
        if self.start >= other.start && self_max <= other_max {
            // Return a shifted range
            return vec![Range {
                start: (self.start as i64 + shift) as u64,
                length: self.length,
            }];
        }

        // Split, and shift several ranges
        let mut return_ranges = vec![];
        //[1-5] [3-10]
        if self.start < other.start {
            // Creare a new range from self.start to other start - 1
            // e.g. [1-2]
            return_ranges.push(Range {
                start: self.start,
                length: other.start - self.start,
            });
        }
        // [3-10] [0-5]
        if self_max > other_max {
            // Create a new range from other max (+1) to self_max
            // e.g. [6-10]
            return_ranges.push(Range {
                start: other_max + 1,
                length: self_max - other_max + 1,
            });
        }
        // Create the new 'inner range' that needs to be shifted
        // [1-5] [3-10] -> [3-5]
        // [3-10] [0-6] -> [3-6]
        // [0-10] [3-4] -> [3-4]
        return_ranges.push(Range {
            start: (self.start.max(other.start) as i64 + shift) as u64,
            length: self_max.min(other_max) - self.start.max(other.start) + 1,
        });

        return_ranges
    }
}
#[derive(Debug)]
struct MappingRange {
    pub from_number: u64,
    pub to_number: u64,
    pub range_number: u64,
}

#[derive(Debug)]
struct Mapping {
    pub mappings: Vec<MappingRange>,
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut seed_numbers = vec![];
    let mut mappings: Vec<Mapping> = vec![];

    input.lines().for_each(|line| {
        if line.contains("seeds:") {
            seed_numbers = get_numbers_from_str(&line.replace("seeds:", " "));
        } else if line.contains("-to-") {
            mappings.push(Mapping { mappings: vec![] });
        } else if line.len() > 1 {
            let numbers = get_numbers_from_str(line);
            mappings.last_mut().unwrap().mappings.push(MappingRange {
                from_number: numbers[1],
                to_number: numbers[0],
                range_number: numbers[2],
            });
        }
    });

    Some(
        seed_numbers
            .iter()
            .map(|seed| {
                let mut mapped_value: u64 = *seed;
                mappings.iter().for_each(|mapping| {
                    let mut mapping_found = false;
                    mapping.mappings.iter().for_each(|range| {
                        if mapping_found {
                            return;
                        };

                        if mapped_value >= range.from_number
                            && mapped_value < range.from_number + range.range_number
                        {
                            mapping_found = true;
                            mapped_value = mapped_value - range.from_number + range.to_number;
                        }
                    });
                });
                return mapped_value;
            })
            .min()
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut check_ranges: Vec<Range> = vec![];
    let mut mappings: Vec<Mapping> = vec![];

    input.lines().for_each(|line| {
        if line.contains("seeds:") {
            let seed_ranges = get_numbers_from_str(&line.replace("seeds:", " "));
            for chunk in &seed_ranges.iter().chunks(2) {
                let (seed, range) = chunk.collect_tuple().unwrap();
                check_ranges.push(Range {
                    start: *seed,
                    length: *range,
                });
            }
        } else if line.contains("-to-") {
            mappings.push(Mapping { mappings: vec![] });
        } else if line.len() > 1 {
            let numbers = get_numbers_from_str(line);
            mappings.last_mut().unwrap().mappings.push(MappingRange {
                from_number: numbers[1],
                to_number: numbers[0],
                range_number: numbers[2],
            });
        }
    });

    println!("{:?}", check_ranges);
    mappings.iter().for_each(|mapping| {
        let mut new_check_ranges: Vec<Range> = vec![];
        mapping.mappings.iter().for_each(|range_mapping| {
            check_ranges.iter().for_each(|range| {
                new_check_ranges.append(&mut range.intersect(
                    Range {
                        start: range_mapping.from_number,
                        length: range_mapping.range_number,
                    },
                    range_mapping.to_number as i64 - range_mapping.from_number as i64,
                ));
            })
        });
        check_ranges = new_check_ranges;
    });
    None
    // Ranges to check
    // [79-92] [55-67]

    // For each test range ^
    // Check for intersection with the below ranges
    // Generate new check ranges
    // A) [98-99]->[50-51], B) [50-97]->[52-99]
    // Intersect with A? No, No
    // Intersect with B? Yes, [81-94], [57-69]
    // A) [15-51]->[0-36], [52-53]->[37]

    /*
    Some(
        seed_numbers
            .iter()
            .map(|seed| {
                let mut mapped_value: u64 = *seed;

                mappings.iter().for_each(|mapping| {
                    let mut mapping_found = false;
                    mapping.mappings.iter().for_each(|range| {
                        if mapping_found {
                            return;
                        };

                        if mapped_value >= range.from_number
                            && mapped_value < range.from_number + range.range_number
                        {
                            mapping_found = true;
                            mapped_value = mapped_value - range.from_number + range.to_number;
                        }
                    });
                });
                return mapped_value;
            })
            .min()
            .unwrap(),
    )
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
