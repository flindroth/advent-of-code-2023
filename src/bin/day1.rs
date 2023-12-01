use aoc_2023::aoc;
fn main() {
    let input = aoc::get_input(2023,1).expect("Could not get input");

    let sum: u32 = input.iter()
        .filter(|line| line.to_owned() != "\n")
        .map(|line| {
            let nums: Vec<u32> = line.chars().into_iter()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_digit(10).unwrap())
                .collect();
            (nums.get(0).unwrap().to_owned(), nums.get(nums.len()-1).unwrap().to_owned())
        })
        .map(|(left, right)| left * 10 + right)
        .fold(0, |acc, x| acc + x);

    println!("First star: {sum}");

   
    let sum2: u32 = input.iter()
        .filter(|line| line.to_owned() != "\n")
        .map(|line| extract_numbers_star_two(line))
        .map(|(left, right)| left * 10 + right)
        .fold(0, |acc, x| acc + x);

    println!("Second star: {sum2}");


}

fn extract_numbers_star_two(line: &String) -> (u32, u32) {
    let mut remainder = line.clone();
    let tokens = vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut read: Vec<&str> = vec![];

    while remainder.len() > 0 {
        for t in tokens.iter() {
            if remainder.starts_with(t) {
                read.push(t);
            }
        }
        remainder = remainder[1..].to_string();
    }

    let numbers: Vec<u32> = read.iter()
        .map(|s| match s.to_owned() {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => str::parse::<u32>(s).unwrap()
        })
        .collect();


    (numbers.get(0).unwrap().to_owned(), numbers.get(numbers.len()-1).unwrap().to_owned())
}