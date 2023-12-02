use aoc_2023::aoc;
fn main() {
    let input = aoc::get_input(2023,2).expect("Could not get input");
    let games = parse_games(&input);
    let possible_sum: u32 = games.iter()
        .filter(|game| game_is_possible(game, 12, 13, 14))
        .fold(0, |acc, x| acc + x.number);

    println!("Star 1: {possible_sum}");
}

#[derive(Debug)]
struct Game {
    number: u32,
    pick_groups: Vec<Vec<(Color, u32)>>
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue
}

fn parse_games(input: &Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];
    for line in input {
        let game_nr_str = line
            .split(":").next().unwrap()
            .split(" ").skip(1).next().unwrap();
        let game_nr = str::parse::<u32>(game_nr_str).unwrap();
        let picks_str = line.split(":").skip(1).next().unwrap().trim();
        let groups_strs = picks_str.split(";");
        let mut groups: Vec<Vec<(Color, u32)>> = vec![];
        for group_str in groups_strs {
            let mut curr_group: Vec<(Color, u32)> = vec![];
            let picks_str = group_str.split(",");
            for pick_str in picks_str {
                let trimmed = pick_str.trim();
                let mut pick_components = trimmed.split(" ");
                let amount = str::parse::<u32>(pick_components.next().unwrap()).unwrap();
                let color = match pick_components.next().unwrap() {
                    "red" => Color::Red,
                    "green" => Color::Green,
                    "blue" => Color::Blue,
                    other => panic!("Invalid color: {other}")
                };
                curr_group.push((color, amount))
            }
            groups.push(curr_group);
        }
        games.push(Game { number: game_nr, pick_groups: groups});
    }
    games
}

fn game_is_possible(game: &Game, max_red: u32, max_green: u32, max_blue: u32) -> bool {
    for group in &game.pick_groups {
        for pick in group {
            if match pick.0 {
                Color::Red => pick.1 > max_red,
                Color::Green => pick.1 > max_green,
                Color::Blue => pick.1 > max_blue
            } {
                return false;
            }
        } 
    }
    true
}