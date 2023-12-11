advent_of_code::solution!(10);

#[derive(Debug)]
enum Connection {
    UpDown,
    LeftRight,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
    None,
    Start,
}

impl std::fmt::Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connection::UpDown => write!(f, "{}", "║"),
            Connection::LeftRight => write!(f, "{}", "═"),
            Connection::UpRight => write!(f, "{}", "╚"),
            Connection::UpLeft => write!(f, "{}", "╝"),
            Connection::DownLeft => write!(f, "{}", "╗"),
            Connection::DownRight => write!(f, "{}", "╔"),
            Connection::None => write!(f, "░"),
            Connection::Start => write!(f, "▣"),
        }
    }
}

struct SewerMap {
    pub tiles: Vec<Vec<Connection>>,
    pub dist: Vec<Vec<i32>>,
    pub start: (usize, usize),
}

// TODO: Use Option<i32> rather than 999999999 to represent the 'no distance'
impl SewerMap {
    pub fn new() -> SewerMap {
        SewerMap {
            tiles: vec![],
            dist: vec![],
            start: (0, 0),
        }
    }

    fn walk_sewer(&mut self) -> i32 {
        // Create our distances array
        self.dist = vec![vec![999999999; self.tiles[0].len()]; self.tiles.len()];

        let (start_x, start_y) = self.start;
        // Mark the start cell 0
        self.dist[start_y][start_x] = 0;

        // Push the adjacent to start cells to a queue
        let mut walk_queue = vec![];

        // Check the four cells next to start, push them into the queue
        if start_y > 0 {
            if let Connection::UpDown | Connection::DownLeft | Connection::DownRight =
                self.tiles[start_y - 1][start_x]
            {
                walk_queue.push((start_x, start_y - 1, 0));
            }
        }
        if start_y < self.tiles.len() - 1 {
            if let Connection::UpDown | Connection::UpLeft | Connection::UpRight =
                self.tiles[start_y + 1][start_x]
            {
                walk_queue.push((start_x, start_y + 1, 0));
            }
        }
        if start_x > 0 {
            if let Connection::LeftRight | Connection::UpRight | Connection::DownRight =
                self.tiles[start_y][start_x - 1]
            {
                walk_queue.push((start_x - 1, start_y, 0));
            }
        }
        if start_x < self.tiles[0].len() - 1 {
            if let Connection::LeftRight | Connection::UpLeft | Connection::DownLeft =
                self.tiles[start_y][start_x + 1]
            {
                walk_queue.push((start_x + 1, start_y, 0));
            }
        }

        let mut max_dist = 0;

        while let Some((current_x, current_y, prev_score)) = walk_queue.pop() {
            let cell = &self.tiles[current_y][current_x];

            // Mark this cell's distance
            let dist = prev_score + 1;
            self.dist[current_y][current_x] = dist;

            // println!("Found: {},{} - {} ({})", current_x, current_y, cell, dist);
            max_dist = max_dist.max(prev_score);

            // Check both neighbors of this cell, if they haven't been explored, add them to the queue
            if let Connection::UpDown | Connection::UpRight | Connection::UpLeft = cell {
                if current_y > 0 && self.dist[current_y - 1][current_x] > dist {
                    walk_queue.push((current_x, current_y - 1, dist));
                }
            }
            if let Connection::UpDown | Connection::DownRight | Connection::DownLeft = cell {
                if current_y < self.tiles.len() - 1 && self.dist[current_y + 1][current_x] > dist {
                    walk_queue.push((current_x, current_y + 1, dist));
                }
            }
            if let Connection::LeftRight | Connection::UpLeft | Connection::DownLeft = cell {
                if current_x > 0 && self.dist[current_y][current_x - 1] > dist {
                    walk_queue.push((current_x - 1, current_y, dist));
                }
            }
            if let Connection::LeftRight | Connection::UpRight | Connection::DownRight = cell {
                if current_x < self.tiles[0].len() - 1 && self.dist[current_y][current_x + 1] > dist
                {
                    walk_queue.push((current_x + 1, current_y, dist));
                }
            }
        }

        self.dist.iter().fold(-1, |acc, row| {
            acc.max(row.iter().fold(0, |row_max, cell| {
                if *cell == 999999999 {
                    return row_max;
                }
                row_max.max(*cell)
            }))
        })
    }

    fn check_interior_count(&self) -> i32 {
        self.tiles.iter().enumerate().fold(0, |acc, (y, row)| {
            let mut is_interior = false;

            acc + row.iter().enumerate().fold(0, |row_count, (x, cell)| {
                if let Connection::UpDown | Connection::UpLeft | Connection::UpRight = cell {
                    if self.dist[y][x] != 999999999 {
                        is_interior = !is_interior;
                    }
                }

                if self.dist[y][x] == 999999999 && is_interior {
                    return row_count + 1;
                }

                row_count
            })
        })
    }
}

impl std::fmt::Display for SewerMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.tiles.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, item)| {
                if self.dist[y][x] == 999999999 {
                    write!(f, "░").unwrap();
                } else {
                    write!(f, "{}", item).unwrap();
                }
            });
            write!(f, "\n").unwrap();
        });

        write!(f, "\nStart Position: {:?}", self.start).unwrap();
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut sewer_map = SewerMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut temp = vec![];

        line.chars().enumerate().for_each(|(x, ch)| {
            match ch {
                '|' => temp.push(Connection::UpDown),
                '-' => temp.push(Connection::LeftRight),
                'L' => temp.push(Connection::UpRight),
                'J' => temp.push(Connection::UpLeft),
                '7' => temp.push(Connection::DownLeft),
                'F' => temp.push(Connection::DownRight),
                '.' => temp.push(Connection::None),
                'S' => {
                    temp.push(Connection::Start);
                    sewer_map.start = (x, y);
                }
                _ => println!("Unknown character in map: {}", ch),
            };
        });
        sewer_map.tiles.push(temp);
    });

    let result = sewer_map.walk_sewer();

    println!("{}", sewer_map);
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sewer_map = SewerMap::new();

    input.lines().enumerate().for_each(|(y, line)| {
        let mut temp = vec![];

        line.chars().enumerate().for_each(|(x, ch)| {
            match ch {
                '|' => temp.push(Connection::UpDown),
                '-' => temp.push(Connection::LeftRight),
                'L' => temp.push(Connection::UpRight),
                'J' => temp.push(Connection::UpLeft),
                '7' => temp.push(Connection::DownLeft),
                'F' => temp.push(Connection::DownRight),
                '.' => temp.push(Connection::None),
                'S' => {
                    temp.push(Connection::Start);
                    sewer_map.start = (x, y);
                }
                _ => println!("Unknown character in map: {}", ch),
            };
        });
        sewer_map.tiles.push(temp);
    });

    sewer_map.walk_sewer();

    Some(sewer_map.check_interior_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(567));
    }
}
