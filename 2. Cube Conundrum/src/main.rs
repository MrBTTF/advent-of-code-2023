use std::fs;

const RED: usize = 0;
const GREEN: usize = 1;
const BLUE: usize = 2;

fn color_to_idx(color: &str) -> usize {
    match color {
        "red" => RED,
        "green" => GREEN,
        "blue" => BLUE,
        _ => unreachable!(),
    }
}

fn get_cubes_used(game_results: &str) -> [u32; 3] {
    let mut r = [0; 3];
    for result in game_results.split("; ") {
        for throw in result.split(", ") {
            let throw = throw.trim();
            // println!("{throw}");
            let mut s = throw.split(' ');
            let number: u32 = s.next().unwrap().parse().unwrap();
            let color = color_to_idx(s.next().unwrap());
            if number > r[color] {
                r[color] = number;
            }
        }
        // println!();
    }
    r
}

fn is_game_valid(total_cubes: [u32; 3], cubes_used: [u32; 3]) -> bool {
    total_cubes[0] >= cubes_used[0]
        && total_cubes[1] >= cubes_used[1]
        && total_cubes[2] >= cubes_used[2]
}
fn get_set_power(cubes_used: [u32; 3]) -> u32 {
    cubes_used[0] * cubes_used[1] * cubes_used[2]
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let total_cubes: [u32; 3] = [12, 13, 14];

    let mut sum: u32 = 0;
    let mut power_sum: u32 = 0;
    for line in contents.lines() {
        let mut game = line.split(":");
        let game_id: u32 = game
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let game_results = game.next().unwrap();
        let cubes_used = get_cubes_used(game_results);
        if is_game_valid(total_cubes, cubes_used) {
            sum += game_id;
        }
        let set_power = get_set_power(cubes_used);
        println!("Game {}: {:?}", game_id, set_power);
        power_sum += set_power;
    }

    println!("Sum of IDs of valid games: {sum}");
    println!("Sum of powers of miminum: {power_sum}");
}
