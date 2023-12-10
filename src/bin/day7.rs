use std::{collections::HashMap, cmp::Ordering};

use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 7).expect("Could not get input");
    let mut hands = parse_hands(&input);

    hands.sort_by(|a, b| a.cards.cmp(&b.cards));

    let ranks: Vec<usize> = (1..=hands.len()).into_iter().collect();

    let sum: u64 = hands
        .iter()
        .zip(ranks)
        .map(|(hand, rank)| {
            println!("Hand: {hand:?} with rank {rank}: {}", hand.bid as u64 * rank as u64);

            hand.bid as u64 * rank as u64 
        })
        .sum();

    println!("Star 1: {sum}");


}

#[derive(Debug)]
struct Hand {
    cards: CardsInHand,
    bid: u64
}

#[derive(Debug)]
struct CardsInHand {
    card_str: String,
    value: u8
}

impl PartialOrd for CardsInHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value.cmp(&other.value) != Ordering::Equal {
            // value (hand kind) differs, go for that
            return Some(self.value.cmp(&other.value));
        }
        // Else, compare the hands themselves
        Some(compare_cards_in_order(&self.card_str, &other.card_str))
    }
}

impl Ord for CardsInHand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialEq for CardsInHand {
    fn eq(&self, other: &Self) -> bool {
        self.card_str == other.card_str && self.value == other.value
    }
}

impl Eq for CardsInHand {

}

fn parse_hands(lines: &Vec<String>) -> Vec<Hand> {
    lines
        .iter()
        .map(|line| parse_hand(line))
        .collect()
}

fn parse_hand(line: &String) -> Hand {
    let mut iter = line.split(" ");
    let cards = iter.next().unwrap();
    let bid = str::parse::<u64>(iter.next().unwrap()).unwrap();

    let kind = get_kind(cards);

    Hand { cards: CardsInHand { card_str: cards.to_string(), value: kind }, bid }
}

fn get_kind(cards: &str) -> u8 {
    let mut counts: HashMap<char, u8> = HashMap::new();
    for c in cards.chars() {
        let count = match counts.get(&c) {
            Some(i) => i + 1,
            None => 1
        };
        counts.insert(c, count);        
    }
    let mut max: Vec<&u8> = counts.values().collect();
    max.sort();
    max.reverse();

    if max.get(0).unwrap() == &&5 {
        return 7; // Five of a kind
    }

    if max.get(0).unwrap() == &&4 {
        return 6; // Four of a kind
    }

    if max.get(0).unwrap() == &&3 {
        if max.get(1).unwrap() == && 2 {
            return 5; // Full house
        }
        return 4; // Three of a kind
    }

    if max.get(0).unwrap() == &&2 {
        if max.get(1).unwrap() == &&2 {
            return 3; // Two pairs
        }
        return 2; // One pair
    }

    1 // All cards different
}

fn compare_cards_in_order(cards: &String, other: &String) -> Ordering {
    let first_different_card = cards.chars()
        .zip(other.chars())
        .skip_while(|(a, b)| a == b)
        .next();

    match first_different_card {
        Some((a, b)) => value_of_card(a).cmp(&value_of_card(b)),
        None => Ordering::Equal
    }
}

fn value_of_card(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        c => str::parse::<u8>(c.to_string().as_str()).unwrap()
    }
}