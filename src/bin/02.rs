advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;
    let game_sum = input
        .lines()
        .filter_map(|line| {
            let (game_bit, pulls) = line.split_once(":").unwrap();
            let game_number = game_bit.replace("Game ", "").parse::<u32>().unwrap();
            let is_invalid = pulls.trim().split(";").any(|pull| {
                pull.split(",").any(|cubes| {
                    let (qty_str, color) = cubes.trim().split_once(" ").unwrap();
                    let qty = qty_str.parse::<i32>().unwrap();

                    match color {
                        "blue" => return qty > max_blue,
                        "green" => return qty > max_green,
                        "red" => return qty > max_red,
                        &_ => return true,
                    }
                })
            });
            if is_invalid {
                None
            } else {
                Some(game_number)
            }
        })
        .sum();
    Some(game_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let game_sum = input
        .lines()
        .filter_map(|line| {
            let mut green_max = 0u32;
            let mut blue_max = 0u32;
            let mut red_max = 0u32;
            let (_, pulls) = line.split_once(":").unwrap();

            pulls.trim().split(";").for_each(|pull| {
                pull.split(",").for_each(|cubes| {
                    let (qty_str, color) = cubes.trim().split_once(" ").unwrap();
                    let qty = qty_str.parse::<u32>().unwrap();

                    match color {
                        "blue" => blue_max = blue_max.max(qty),
                        "green" => green_max = green_max.max(qty),
                        "red" => red_max = red_max.max(qty),
                        &_ => return,
                    }
                })
            });
            Some(green_max * blue_max * red_max)
        })
        .sum();
    Some(game_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
