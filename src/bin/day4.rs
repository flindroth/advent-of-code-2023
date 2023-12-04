use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 4).expect("Could not get input");
    let sum: u32 = input
        .iter()
        .map(|s| parse_card(s))
        .map(|c| score_card(&c))
        .sum();

    println!("Star 1: {sum}")
}

#[derive(Debug)]
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

fn score_card(card: &Card) -> u32 {
    let winning_scratches = card.scratches
        .iter()
        .filter(|scratch| card.winning_numbers.contains(&scratch))
        .count();
    if winning_scratches == 0 {
        return 0;
    }

    2u32.pow((winning_scratches - 1) as u32)
}