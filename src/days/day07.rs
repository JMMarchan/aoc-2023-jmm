use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input = read_to_string("input/day07.txt").expect("Day 7 input file should be present.");
    let input: Vec<&str> = input.lines().collect();
    let sol1: u64 = poker_total_winnings(&input, false);
    let sol2: u64 = poker_total_winnings(&input, true);

    (Solution::from(sol1), Solution::from(sol2))
}

// The input is a list of hands of five cards and their corresponding bids
// Hands are ordered by type, and then if type is the same, compare first card, then second card, etc.
// We rank each hand, weakest as 1, then 2, etc.
// The total winnings is equal to the sum of the bid amounts for each hand multiplied by the rank of the hand
// In the second part, J instead of being a jack is a joker, which can be any card for the purposes of determining hand type but is always treated as less than any other card
fn poker_total_winnings(input: &[&str], joker: bool) -> u64 {
    let mut hands_with_bids = input
        .iter()
        .map(|line| parse_hand_and_bid(line, joker))
        .collect::<Vec<_>>();

    hands_with_bids.sort_by(|(a_hand, _), (b_hand, _)| compare_hands(a_hand, b_hand));

    hands_with_bids
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank as u64 + 1) * bid)
        .sum()
}

fn parse_hand_and_bid(input: &str, joker: bool) -> (Hand, u64) {
    let (hand_str, bid_str) = input.split_at(5);
    let hand = {
        let cards = hand_str
            .chars()
            .map(|c| match c {
                'A' => CardValue::Ace,
                'K' => CardValue::King,
                'Q' => CardValue::Queen,
                'J' => {
                    if joker {
                        CardValue::Joker
                    } else {
                        CardValue::Jack
                    }
                }
                'T' => CardValue::Ten,
                n => CardValue::Number(n.to_digit(10).unwrap() as u8),
            })
            .collect::<Vec<_>>();

        let hand_type = if joker {
            determine_hand_type_with_joker(&cards)
        } else {
            determine_hand_type(&cards)
        };

        Hand {
            cards, // Original order of cards
            hand_type,
        }
    };
    let bid = bid_str.trim().parse::<u64>().unwrap();

    (hand, bid)
}

fn determine_hand_type_with_joker(cards: &[CardValue]) -> HandType {
    let mut counts = std::collections::HashMap::new();
    let mut joker_count = 0;
    for card in cards {
        if *card == CardValue::Joker {
            joker_count += 1;
        } else {
            *counts.entry(card).or_insert(0) += 1;
        }
    }

    if joker_count == cards.len() {
        return HandType::FiveOfAKind;
    }

    let mut best_hand = HandType::HighCard;
    for (value, _) in &counts {
        for joker_combination in 0..=joker_count {
            let mut cards_with_jokers = cards.to_vec();
            for _ in 0..joker_combination {
                if let Some(joker_index) = cards_with_jokers
                    .iter()
                    .position(|&card| card == CardValue::Joker)
                {
                    cards_with_jokers[joker_index] = **value;
                }
            }
            // println!("Cards with jokers: {:?}", cards_with_jokers);
            let hand_type = determine_hand_type(&cards_with_jokers);
            if hand_type > best_hand {
                best_hand = hand_type;
            }
        }
    }
    best_hand
}

fn determine_hand_type(cards: &[CardValue]) -> HandType {
    let mut counts = std::collections::HashMap::new();
    for card in cards {
        *counts.entry(card).or_insert(0) += 1;
    }

    match counts.len() {
        5 => HandType::HighCard, // All cards are different
        4 => HandType::OnePair,  // One pair, three different cards
        3 => {
            // Either two pairs or three of a kind
            if counts.values().any(|&v| v == 3) {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        2 => {
            // Either full house or four of a kind
            if counts.values().any(|&v| v == 4) {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        1 => HandType::FiveOfAKind, // All cards are the same
        _ => panic!("Invalid number of cards in hand"),
    }
}

// Custom comparison function for hands
fn compare_hands(a_hand: &Hand, b_hand: &Hand) -> std::cmp::Ordering {
    // Compare first by hand type
    match a_hand.hand_type.cmp(&b_hand.hand_type) {
        std::cmp::Ordering::Equal => {
            // If hand types are equal, compare the individual cards in their original order
            for (a, b) in a_hand.cards.iter().zip(&b_hand.cards) {
                match a.cmp(b) {
                    std::cmp::Ordering::Equal => continue,
                    other => return other,
                }
            }
            std::cmp::Ordering::Equal
        }
        other => other,
    }
}

// Structures to represent a hand of poker
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Hand {
    cards: Vec<CardValue>,
    hand_type: HandType,
}

// we can create enums with macro derive sorts for the cards
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
// And then for the types
// Five of a kind, Four of a kind, Full house, Three of a kind, Two pair, One pair, High card
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum CardValue {
    Joker, // J can be a joker, which is treated as less than any other card
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day07_test_input() -> &'static str {
        "32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483"
    }

    #[test]
    fn test_parse_hand_and_bid() {
        let input = day07_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(poker_total_winnings(&input, false), 6440);
    }

    #[test]
    fn test_parse_hand_and_bid_with_joker() {
        let input = day07_test_input();
        // split into lines
        let input: Vec<&str> = input.lines().collect();
        assert_eq!(poker_total_winnings(&input, true), 5905);
    }
}
