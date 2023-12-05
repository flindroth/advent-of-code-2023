use aoc_2023::aoc;

fn main() {
    let input = aoc::get_input(2023, 5).unwrap();
    //let input = aoc::get_sample_input("samples/day5.txt").expect("Could not read input");
    let almanac = Almanac::new(input);

    let min = almanac.seeds
        .iter()
        .map(|s| almanac.map_to_end(*s))
        .min()
        .unwrap();

    println!("Star 1: {min}");
        //.for_each(|s| println!("Seed {s} mapped to {}", almanac.map_to_end(*s)));
}

struct Almanac {
    seeds: Vec<u64>,
    mappers: Vec<Mapper>
}

impl Almanac {
    fn new(input: Vec<String>) -> Self {
        let mut iter = input.iter();

        let seeds = iter
            .next()
            .unwrap().split(": ")
            .skip(1)
            .next()
            .unwrap()
            .split(" ")
            .map(|s| str::parse::<u64>(s).unwrap())
            .collect();

        let mapper_lines = iter
            .map(|s| s.clone())
            .collect();

        let mappers = Mapper::mappers_from_lines(mapper_lines);



        Almanac { seeds, mappers}
    }

    fn map_to_end(&self, seed: u64) -> u64 {
        let mut value = seed;
        for mapper in &self.mappers {
            value = mapper.map(value);
        }
        value
    }
}

#[derive(Debug)]
struct Mapper {
    ranges: Vec<Range>
}

impl Mapper {
    fn map(&self, value: u64) -> u64 {
        for range in &self.ranges {
            if value >= range.source_start && value <= range.source_start + range.length {
                let mapped = value as i64 + (range.dest_start as i64) - (range.source_start as i64);
                return mapped as u64;
            }
        }
        return value;
    }
}

#[derive(Clone, Debug)]
struct Range {
    source_start: u64,
    dest_start: u64,
    length: u64
}

impl Mapper {
    fn mappers_from_lines(lines: Vec<String>) -> Vec<Self> {
        let mut mappers = vec![];
        
        let mut curr_ranges: Vec<Range> = vec![];
        for line in lines {
            if line.contains(":") {
                continue;
            }
            if line == "" {
                if curr_ranges.len() > 0 {
                    mappers.push(Mapper { ranges: curr_ranges.clone() });
                    curr_ranges.clear();
                }
                continue;
            }
            let numbers: Vec<u64> = line.split(" ")
                .map(|s| str::parse::<u64>(s).unwrap())
                .collect();
            curr_ranges.push(Range { dest_start: numbers.get(0).unwrap().clone(), source_start: numbers.get(1).unwrap().clone(), length: numbers.get(2).unwrap().clone() })
        }
        if curr_ranges.len() > 0 {
            mappers.push(Mapper { ranges: curr_ranges.clone()});
        }

        mappers
    }
}