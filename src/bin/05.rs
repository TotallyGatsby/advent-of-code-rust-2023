use advent_of_code::helpers::get_numbers_from_str;
advent_of_code::solution!(5);

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

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
