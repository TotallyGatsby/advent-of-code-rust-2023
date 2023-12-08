use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(7);
#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

const CARD_VALUE: [(char, u32); 13] = [
    ('A', 14),
    ('K', 13),
    ('Q', 12),
    ('J', 11),
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
];

const CARD_VALUE_WITH_JOKERS: [(char, u32); 13] = [
    ('A', 14),
    ('K', 13),
    ('Q', 12),
    ('T', 10),
    ('9', 9),
    ('8', 8),
    ('7', 7),
    ('6', 6),
    ('5', 5),
    ('4', 4),
    ('3', 3),
    ('2', 2),
    ('J', 1),
];

fn get_card_value(char: &char, use_jokers: bool) -> u32 {
    if use_jokers {
        return CARD_VALUE_WITH_JOKERS
            .iter()
            .find_map(|(card_char, card_val)| {
                if char == card_char {
                    return Some(*card_val);
                }
                return None;
            })
            .unwrap();
    }
    CARD_VALUE
        .iter()
        .find_map(|(card_char, card_val)| {
            if char == card_char {
                return Some(*card_val);
            }
            return None;
        })
        .unwrap()
}

#[derive(Debug, Eq, Ord)]
struct ScoredHand {
    pub hand_type: HandType,
    pub hand_str: String,
    pub bet: u32,
    pub use_jokers: bool,
}

impl ScoredHand {
    fn new(line: &str, use_jokers: bool) -> ScoredHand {
        let (cards, bet) = line.split_ascii_whitespace().collect_tuple().unwrap();

        let mut char_counts = HashMap::new();
        let mut joker_count = 0;
        cards.chars().for_each(|char| {
            if char == 'J' && use_jokers {
                joker_count += 1;
                return;
            }

            if !char_counts.contains_key(&char) {
                char_counts.insert(char, 0);
            }
            *char_counts.get_mut(&char).unwrap() += 1;
        });

        let mut hand_type = HandType::HighCard;

        if char_counts.len() == 1 {
            hand_type = HandType::FiveOfAKind;
        } else if char_counts.len() == 2 {
            // Could be a full house or 4 of a kind
            let test = *char_counts.values().max().unwrap();
            if test == 4 {
                hand_type = HandType::FourOfAKind;
            } else if joker_count == 0 {
                hand_type = HandType::FullHouse;
            } else if test == 3 {
                hand_type = HandType::ThreeOfAKind;
            } else if test == 2 {
                if *char_counts.values().min().unwrap() == 1 {
                    hand_type = HandType::OnePair;
                } else {
                    hand_type = HandType::TwoPair;
                }
            }
        } else if char_counts.len() == 3 {
            // Three of a kind or two pair
            let test = *char_counts.values().max().unwrap();
            if test == 3 {
                hand_type = HandType::ThreeOfAKind;
            } else if test == 2 && joker_count == 0 {
                hand_type = HandType::TwoPair;
            } else if test == 2 {
                hand_type = HandType::OnePair;
            }
        } else if char_counts.len() == 4 {
            let test = *char_counts.values().max().unwrap();
            if test == 2 {
                hand_type = HandType::OnePair;
            }
        }

        if use_jokers && joker_count > 0 {
            match hand_type {
                HandType::FiveOfAKind => {
                    println!("Matched 5 of a kind with jokers: {}", cards);
                } // This shouldn't be possible
                HandType::FourOfAKind => hand_type = HandType::FiveOfAKind,
                HandType::FullHouse => {
                    println!("Matched full house with jokers: {}", cards);
                } // This shouldn't be possible
                HandType::ThreeOfAKind => {
                    if joker_count == 1 {
                        hand_type = HandType::FourOfAKind;
                    } else {
                        hand_type = HandType::FiveOfAKind;
                    }
                }
                HandType::TwoPair => hand_type = HandType::FullHouse,
                HandType::OnePair => {
                    if joker_count == 1 {
                        hand_type = HandType::ThreeOfAKind;
                    } else if joker_count == 2 {
                        hand_type = HandType::FourOfAKind;
                    } else {
                        hand_type = HandType::FiveOfAKind;
                    }
                }
                HandType::HighCard => {
                    if joker_count == 1 {
                        hand_type = HandType::OnePair;
                    } else if joker_count == 2 {
                        hand_type = HandType::ThreeOfAKind;
                    } else if joker_count == 3 {
                        hand_type = HandType::FourOfAKind;
                    } else {
                        hand_type = HandType::FiveOfAKind;
                    }
                }
            }
        }

        // Determine the hand type
        ScoredHand {
            hand_type,
            hand_str: cards.to_string(),
            bet: bet.parse::<u32>().unwrap(),
            use_jokers,
        }
    }
}

impl PartialOrd for ScoredHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type == other.hand_type {
            return self.hand_str.chars().zip(other.hand_str.chars()).find_map(
                |(self_char, other_char)| {
                    if self_char == other_char {
                        return None;
                    }
                    return Some(
                        get_card_value(&self_char, self.use_jokers)
                            .cmp(&get_card_value(&other_char, self.use_jokers)),
                    );
                },
            );
        }

        self.hand_type.partial_cmp(&other.hand_type)
    }
}

impl PartialEq for ScoredHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_str == other.hand_str
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| ScoredHand::new(line, false))
        .collect();

    hands.sort();

    Some(hands.iter().enumerate().fold(0, |acc, (idx, hand)| {
        println!(
            "{}\t{} \t({})\t{:?}",
            hand.hand_str, hand.bet, acc, hand.hand_type
        );
        acc + (1 + idx) as u32 * hand.bet
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<_> = input
        .lines()
        .map(|line| ScoredHand::new(line, true))
        .collect();

    hands.sort();

    Some(hands.iter().enumerate().fold(0, |acc, (idx, hand)| {
        println!(
            "{}\t{} \t({})\t{:?}",
            hand.hand_str, hand.bet, acc, hand.hand_type
        );
        acc + (1 + idx) as u32 * hand.bet
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
