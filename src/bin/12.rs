use itertools::Itertools;

advent_of_code::solution!(12);

fn search_springs(line: &str, scores: &Vec<u32>) -> u32 {
    // If we match the pattern, return 1 (treating all ? as .)
    if scores
        .iter()
        .zip_longest(line.replace("?", ".").split(".").filter(|c| c.len() > 0))
        .all(|zip| {
            if !zip.is_both() {
                return false;
            }
            let (len, substr) = zip.both().unwrap();
            return substr.len() == *len as usize;
        })
    {
        // println!("Found");
        return 1;
    }
    let spring_groups = line.split(".").filter(|c| c.len() > 0).collect_vec();
    if spring_groups.iter().filter(|g| g.contains("#")).count() > scores.len() {
        /*println!(
            "Skipped (Spring groups too large {} vs {})\n{}",
            spring_groups.len(),
            scores.len(),
            line
        );*/
        return 0;
    }

    let max_count = spring_groups.iter().fold(0, |acc, group| {
        let q_count = group.chars().filter(|c| *c == '?').count();
        let h_count = group.split("?").filter(|c| c.len() > 0).collect_vec().len();
        // println!("{} - {}", group, h_count + ((q_count + 1) / 2));
        acc + h_count + ((q_count + 1) / 2)
    });

    if max_count < scores.len() {
        // println!("Skipped ({} max groups) \n{}", max_count, line);
        return 0;
    }

    // Replace the first ? and try both options
    if line.contains("?") {
        return search_springs(line.replacen("?", ".", 1).as_str(), scores)
            + search_springs(line.replacen("?", "#", 1).as_str(), scores);
    }

    // If we have no ?, and have no matches, return 0
    0
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().fold(0, |acc, line| {
        let chunks = line.split(" ").collect_vec();
        // println!("\n New Arrangement: ");
        let new_results = search_springs(
            chunks[0],
            &chunks[1]
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect_vec(),
        );
        // println!("Found {} results.", new_results);
        acc + new_results
    }))
}

pub fn part_two(_input: &str) -> Option<u32> {
    return None;
    /*
    Some(input.lines().fold(0, |acc, line| {
        let chunks = line.split(" ").collect_vec();
        println!("Testing: {}", chunks[0]);
        let mult_line = vec![chunks[0]].repeat(5).join("?");
        let new_results = search_springs(
            mult_line.as_str(),
            &chunks[1]
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect_vec()
                .repeat(5),
        );

        println!("Found {} results.", new_results);
        acc + new_results
    })) */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
