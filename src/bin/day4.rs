use std::collections::HashMap;

use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 4).expect("Could not get input");
    //let input = aoc::get_sample_input("samples/day4.txt").unwrap();
    let cards: Vec<Card> = input
        .iter()
        .map(|s| parse_card(s))
        .collect();

    let sum: u32 = cards
        .iter()
        .map(|c| score_card(&c))
        .sum();

    println!("Star 1: {sum}");

    
    let mut card_piles: HashMap<u32, usize> = HashMap::new();
    for card in &cards {
        card_piles.insert(card.card_nr, 1);
    }

    for card_nr in 1..cards.len()+1 {
        let card = get_card(&cards, card_nr as u32).unwrap();
        let amount_in_pile = card_piles.get(&(card_nr as u32)).unwrap();
        let prize = cash_in(&cards, &card);
        for _ in 0..*amount_in_pile {
            for prize_card in &prize {
                let cards_in_pile = card_piles.get(&prize_card.card_nr).unwrap();
                card_piles.insert(prize_card.card_nr, cards_in_pile + 1);
            }
        }
    }

    let card_count: u32 = card_piles
        .iter()
        .map(|(_, pile)| *pile as u32)
        .sum();

    println!("Star 2: {}", card_count);

}

#[derive(Debug, Clone)]
struct Card {
    card_nr: u32,
    winning_numbers: Vec<u32>,
    scratches: Vec<u32>
}

fn parse_card(line: &String) -> Card {
    let mut line_pipe_parts = line.split("|");
    let left_part = line_pipe_parts.next().unwrap();
    let right_part = line_pipe_parts.next().unwrap();

    let card_nr_str = left_part.split(":")
        .next()
        .unwrap()
        .strip_prefix("Card ")
        .unwrap()
        .trim();
    let card_nr = str::parse::<u32>(card_nr_str).unwrap();

    let winning_numbers = parse_num_series(left_part.split(":").skip(1).next().unwrap().trim());
    let scratches = parse_num_series(right_part.trim());

    Card { card_nr, winning_numbers, scratches }

}

fn parse_num_series(input: &str) -> Vec<u32> {
    let commafied = input.replace("  ", ",").replace(" ", ",");
    commafied.split(",")
        .map(|s| str::parse::<u32>(s).unwrap())
        .collect()
}

fn count_winning_scratches(card: &Card) -> u32 {
    card.scratches
        .iter()
        .filter(|scratch| card.winning_numbers.contains(&scratch))
        .count() as u32
}

fn score_card(card: &Card) -> u32 {
    let winning_scratches = count_winning_scratches(card);
    if winning_scratches == 0 {
        return 0;
    }

    2u32.pow((winning_scratches - 1) as u32)
}

fn cash_in(stack: &Vec<Card>, card: &Card) -> Vec<Card> {
    let winning = count_winning_scratches(card);
    if winning == 0 {
        return vec![];
    }
    let new_cards = (card.card_nr+1..card.card_nr+winning+1)
        .flat_map(|n| get_card(stack, n))
        .collect();
    new_cards
}

fn get_card(cards: &Vec<Card>, number: u32) -> Option<Card> {
    let card = cards.iter().find(|c| c.card_nr == number);

    match card {
        Some(card) => Some(card.clone()),
        None => None
    }
}