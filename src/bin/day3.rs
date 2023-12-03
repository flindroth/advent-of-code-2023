use core::num;
use std::{collections::{HashSet, HashMap}, str::Chars};

use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 3).expect("Could not get input");
    //let input = aoc::get_sample_input("samples/day3.txt").expect("Could not get input");
    let symbols = parse_symbols(&input);
    println!("Found {} symbols", symbols.len());
    let part_numbers = parse_part_numbers(&input, &symbols);
    println!("Found {} part numbers", part_numbers.len());

    let part_number_sum = part_numbers
        .iter()
        .fold(0, |acc, x| acc + x);

    println!("Star 1: {part_number_sum}");
}

fn parse_part_numbers(input: &Vec<String>, symbols: &HashSet<(usize, usize)>) -> Vec<u32> {
    let mut part_numbers: Vec<u32> = vec![];
    let mut row_num: usize = 0;
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
                if is_part_number((start, idx), row_num, symbols) {
                    let num_str = &row[start..idx];
                    println!("{row_num},{start}: {num_str}");
                    part_numbers.push(str::parse::<u32>(num_str).unwrap());
                }
            }
            num_start = None;
        }
        if let Some(start) = num_start {
            if is_part_number((start, row.len()), row_num, symbols) {
                let num_str = &row[start..row.len()];
                println!("{row_num},{start}: {num_str}");
                part_numbers.push(str::parse::<u32>(num_str).unwrap());
            }
        }
        row_num += 1;
    }
    part_numbers
}

fn is_part_number(num_start_end: (usize, usize), row_num: usize, symbols: &HashSet<(usize, usize)>) -> bool {
    for idx in num_start_end.0..num_start_end.1 {
        if char_is_adjacent_to_symbol((row_num, idx), symbols) {
            return true;
        }
    }

    false
}

fn char_is_adjacent_to_symbol(coords: (usize, usize), symbols: &HashSet<(usize, usize)>) -> bool {
    let row = coords.0;
    let col = coords.1;

    //println!("Adj check {row},{col}");

    let mut checks: Vec<(usize, usize)> = vec![(row, col+1),(row+1, col), (row+1,col+1)];

    if row > 0 {
        checks.push((row-1, col));
        checks.push((row-1, col+1));
        if col > 0 {
            checks.push((row-1, col-1));
            checks.push((row, col-1));
            checks.push((row+1, col-1));
        }
    }
    if col > 0 {
        checks.push((row, col-1));
        checks.push((row+1, col-1));
    }

    for check in checks {
        if let Some(_) = symbols.get(&(check.0, check.1)) {
            //println!("TRUE at {},{}", check.0, check.1);
            return true;
        }
    }
    //println!("FALSE");
    false
}

fn parse_symbols(input: &Vec<String>) -> HashSet<(usize, usize)> {
    let mut symbols = HashSet::<(usize, usize)>::new();

    let mut row_num: usize = 0;
    for row in input {
        for (idx, c) in row.char_indices() {
            if c.is_numeric() || c == '.' {
                continue;
            }
            symbols.insert((row_num, idx));
        }
        row_num += 1;
    }

    symbols
}