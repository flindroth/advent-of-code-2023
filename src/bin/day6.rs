use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 6).expect("Could not read input");
    let races = parse_races(&input);

    let result = races
        .iter()
        .map(|race| race.ways_to_win())
        .fold(1, |acc, x| acc * x);

    println!{"Star 1: {result}"}

    let race = parse_one_big_race(&input);

    println!{"Star 2: {}", race.ways_to_win()};
}

#[derive(Debug)]
struct Race {
    duration: u64,
    distance: u64
}

impl Race {
    fn ways_to_win(&self) -> u32 {
        let mut count = 0u32;
        for hold_time in 1..self.duration {
            let distance = hold_time * (self.duration - hold_time);
            if distance > self.distance {
                count += 1;
            }
        }
        count
    }
}

fn parse_races(input: &Vec<String>) -> Vec<Race> {
    let lines: Vec<Vec<u64>> = input
        .iter()
        .map(|line| line.split(":").skip(1).next().unwrap().trim())
        .map(|right| parse_spaced_numbers(right))
        .collect();

    let race_times = lines.get(0).unwrap();
    let race_distances = lines.get(1).unwrap();

    race_times
        .iter()
        .zip(race_distances)
        .map(|(duration, distance)| Race { duration: duration.clone(), distance: distance.clone() })
        .collect()
}

fn parse_spaced_numbers(input: &str) -> Vec<u64> {
    input
        .replace(" ", ",")
        .split(",")
        .filter(|s| *s != "")
        .map(|s| str::parse::<u64>(s).unwrap())
        .collect()
}

fn parse_one_big_race(input: &Vec<String>) -> Race {
    let lines: Vec<u64> = input
        .iter()
        .map(|line| line.split(":").skip(1).next().unwrap().trim())
        .map(|right| str::parse::<u64>(right.replace(" ", "").as_str()).unwrap())
        .collect();

    Race { duration: lines.get(0).unwrap().clone(), distance: lines.get(1).unwrap().clone() }
}

