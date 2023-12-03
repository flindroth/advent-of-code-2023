use std::collections::{HashMap, HashSet};

use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 3).expect("Could not get input");

    let symbols = parse_symbols(&input);
    let (part_numbers, ratios) = parse_part_numbers_and_ratios(&input, &symbols);

    let part_number_sum = part_numbers.iter().fold(0, |acc, x| acc + x);

    println!("Star 1: {part_number_sum}");

    let ratio_sum: u32 = ratios.iter().sum();

    println!("Star 2: {ratio_sum}");
}

type SymbolMap = HashMap<(usize, usize), char>;
type GearRatioMap = HashMap<(usize, usize), (u32, u32)>;

fn parse_part_numbers_and_ratios(input: &Vec<String>, symbols: &SymbolMap) -> (Vec<u32>, Vec<u32>) {
    let mut part_numbers: Vec<u32> = vec![];
    let mut row_num: usize = 0;
    let mut gear_ratios: GearRatioMap = HashMap::new();
    for row in input {
        let mut num_start: Option<usize> = None;
        for (idx, c) in row.char_indices() {
            if c.is_numeric() {
                if num_start == None {
                    num_start = Some(idx);
                }
                continue;
            }
            // No longer a number
            if let Some(start) = num_start {
                handle_number_substring(
                    &mut gear_ratios,
                    &mut part_numbers,
                    row,
                    row_num,
                    symbols,
                    (start, idx),
                )
            }
            num_start = None;
        }
        if let Some(start) = num_start {
            handle_number_substring(
                &mut gear_ratios,
                &mut part_numbers,
                row,
                row_num,
                symbols,
                (start, row.len()),
            );
        }
        row_num += 1;
    }
    let mut ratio_vec: Vec<u32> = vec![];
    for gear in gear_ratios.values() {
        ratio_vec.push(gear.0 * gear.1);
    }
    (part_numbers, ratio_vec)
}

fn handle_number_substring(
    gear_ratios: &mut GearRatioMap,
    part_numbers: &mut Vec<u32>,
    row: &String,
    row_number: usize,
    symbol_map: &SymbolMap,
    num_start_end: (usize, usize),
) {
    let (part_number, gear_coords) =
        is_part_number((num_start_end.0, num_start_end.1), row_number, symbol_map);
    if part_number {
        let num_str = &row[num_start_end.0..num_start_end.1];
        let num = str::parse::<u32>(num_str).unwrap();
        part_numbers.push(num);

        for coords in gear_coords {
            update_gear_ratios(gear_ratios, &coords, num);
        }
    }
}

fn is_part_number(
    num_start_end: (usize, usize),
    row_num: usize,
    symbols: &SymbolMap,
) -> (bool, HashSet<(usize, usize)>) {
    let mut gear_coords: HashSet<(usize, usize)> = HashSet::new();
    let mut part_number = false;

    for idx in num_start_end.0..num_start_end.1 {
        let adjacent_symbols = get_adjacent_symbols((row_num, idx), symbols);

        if adjacent_symbols.len() > 0 {
            part_number = true;
        }

        for symbol in adjacent_symbols {
            if let Some(map_symbol) = symbols.get(&symbol) {
                if map_symbol == &'*' {
                    gear_coords.insert(symbol);
                }
            }
        }
    }

    (part_number, gear_coords)
}

fn get_adjacent_symbols(coords: (usize, usize), symbols: &SymbolMap) -> HashSet<(usize, usize)> {
    let row = coords.0;
    let col = coords.1;
    let mut symbol_locations: HashSet<(usize, usize)> = HashSet::new();

    let mut checks: Vec<(usize, usize)> = vec![(row, col + 1), (row + 1, col), (row + 1, col + 1)];

    if row > 0 {
        checks.push((row - 1, col));
        checks.push((row - 1, col + 1));
        if col > 0 {
            checks.push((row - 1, col - 1));
            checks.push((row, col - 1));
            checks.push((row + 1, col - 1));
        }
    }
    if col > 0 {
        checks.push((row, col - 1));
        checks.push((row + 1, col - 1));
    }

    for check in checks {
        if let Some(_) = symbols.get(&(check.0, check.1)) {
            symbol_locations.insert((check.0, check.1));
        }
    }
    symbol_locations
    //println!("FALSE");
}

fn parse_symbols(input: &Vec<String>) -> SymbolMap {
    let mut symbols: SymbolMap = HashMap::new();

    let mut row_num: usize = 0;
    for row in input {
        for (idx, c) in row.char_indices() {
            if c.is_numeric() || c == '.' {
                continue;
            }
            symbols.insert((row_num, idx), c);
        }
        row_num += 1;
    }

    symbols
}

fn update_gear_ratios(
    gear_map: &mut HashMap<(usize, usize), (u32, u32)>,
    gear_coords: &(usize, usize),
    add_value: u32,
) {
    let new_ratio = match gear_map.get(gear_coords) {
        Some((0, 0)) => (add_value, 0),
        Some((left, 0)) => (*left, add_value),
        Some((left, right)) => (*left, *right),
        /*
        Some((left, _)) => {
            if *left == 0 {
                (add_value, 0)
            } else {
                (*left, add_value)
            }
        }
        */
        None => (add_value, 0),
    };
    gear_map.insert(*gear_coords, new_ratio);
}
