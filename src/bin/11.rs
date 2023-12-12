use itertools::Itertools;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut galaxy_points = vec![];

    let mut y_nudge = 0;
    let mut x_nudge = 0;
    input.lines().enumerate().for_each(|(y, row)| {
        let mut found_galaxy = false;
        x_nudge = 0;
        row.chars().enumerate().for_each(|(x, char)| {
            // This is so inefficient, lol
            if !input
                .lines()
                .any(|line| line.chars().nth(x).unwrap() == '#')
            {
                x_nudge += 1;
            }

            if char == '#' {
                galaxy_points.push((x + x_nudge, y + y_nudge));
                found_galaxy = true;
            }
        });
        if !found_galaxy {
            y_nudge += 1;
        }
    });

    Some(galaxy_points.iter().combinations(2).fold(0, |acc, c| {
        let point_a = c[0];
        let point_b = c[1];
        acc + point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
    }))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut galaxy_points = vec![];

    let mut y_nudge = 0;
    let mut x_nudge = 0;
    input.lines().enumerate().for_each(|(y, row)| {
        let mut found_galaxy = false;
        x_nudge = 0;
        row.chars().enumerate().for_each(|(x, char)| {
            // This is so inefficient, lol
            if !input
                .lines()
                .any(|line| line.chars().nth(x).unwrap() == '#')
            {
                x_nudge += 999999;
            }

            if char == '#' {
                galaxy_points.push((x + x_nudge, y + y_nudge));
                found_galaxy = true;
            }
        });
        if !found_galaxy {
            y_nudge += 999999;
        }
    });

    println!("{:?}", galaxy_points);

    Some(galaxy_points.iter().combinations(2).fold(0, |acc, c| {
        let point_a = c[0];
        let point_b = c[1];

        acc + point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82000210));
    }
}
