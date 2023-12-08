use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPairs,
    OnePair,
    HighCard,
}

struct Hand {
    cards: Vec<u8>,
    card_type: Type,
    bid: i64,
}

lazy_static! {
    static ref CARD_VALUES: HashMap<char, u8> = [
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
    ]
    .iter()
    .cloned()
    .collect();
}

fn hand_to_cards(hand: &str) -> Vec<u8> {
    hand.chars()
        .filter_map(|c| CARD_VALUES.get(&c))
        .cloned()
        .collect()
}

fn cards_to_type(cards: &Vec<u8>) -> Type {
    let mut card_counts_map: HashMap<u8, u8> = HashMap::new();
    for card in cards {
        let count = card_counts_map.entry(*card).or_insert(0);
        *count += 1;
    }
    let mut card_counts = card_counts_map.values().collect::<Vec<_>>();
    card_counts.sort();
    card_counts.reverse();

    match card_counts.as_slice() {
        [5] => Type::FiveOfAKind,
        [4, ..] => Type::FourOfAKind,
        [3, 2] => Type::FullHouse,
        [3, ..] => Type::ThreeOfAKind,
        [2, 2, ..] => Type::TwoPairs,
        [2, ..] => Type::OnePair,
        _ => Type::HighCard,
    }
}

fn compare_lists(list1: &[u8], list2: &[u8]) -> std::cmp::Ordering {
    for (item1, item2) in list1.iter().zip(list2) {
        match item1.cmp(item2) {
            std::cmp::Ordering::Equal => continue,
            non_equal => return non_equal,
        }
    }
    std::cmp::Ordering::Equal
}

fn line_to_hand(line: &str) -> Hand {
    let mut split = line.split(" ");
    let hand = split.next().unwrap();
    let bid = split.next().unwrap().parse::<i64>().unwrap();

    let cards = hand_to_cards(hand);
    let card_type = cards_to_type(&cards);
    Hand {
        cards,
        card_type,
        bid,
    }
}

pub fn solve1(lines: Vec<String>) -> i64 {
    let mut hands = lines
        .iter()
        .map(|line| line_to_hand(line))
        .collect::<Vec<_>>();

    hands.sort_by(|hand1, hand2| match hand1.card_type.cmp(&hand2.card_type) {
        std::cmp::Ordering::Equal => compare_lists(&hand2.cards, &hand1.cards),
        non_equal => non_equal,
    });
    hands.reverse();

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + (hand.bid * (i as i64 + 1)))
}

#[cfg(test)]
mod tests {
    use advent_of_code_2023::solution_lines;

    use super::*;

    #[test]
    fn test_hand_to_cards() {
        let test = hand_to_cards("53A4J");
        let hand_type = cards_to_type(&test);
        assert_eq!(hand_type, Type::HighCard);
    }

    #[test]
    fn test_compare_lists() {
        let list1 = vec![4, 5, 3];
        let list2 = vec![4, 2, 3];
        assert_eq!(compare_lists(&list1, &list2), std::cmp::Ordering::Greater);
    }

    #[test]
    fn test_hand_str_to_hand() {
        let test = line_to_hand("53A4J 231");
        assert_eq!(test.cards, vec![5, 3, 14, 4, 11]);
        assert_eq!(test.card_type, Type::HighCard);
        assert_eq!(test.bid, 231);
    }

    #[test]
    fn test_solve1_test() {
        let result = solution_lines("day7_test", solve1, 6440);
        assert!(result)
    }

    #[test]
    fn test_solve1() {
        let result = solution_lines("day7", solve1, 251121738);
        assert!(result)
    }
}
