use std::{collections::HashMap, fs};

#[derive(Clone, Copy)]
struct Part {
    symbol: char,
    x: usize,
    y: usize,
}

fn get_adjacent_part(i: usize, j: usize, lines: &[&str]) -> Option<Part> {
    // println!("({i},{j})");
    let start_i = i.clamp(1, lines.len() - 1) - 1;
    let end_i = i.clamp(0, lines.len() - 2) + 2;
    let start_j = j.clamp(1, lines[0].len() - 1) - 1;
    let end_j = j.clamp(0, lines[0].len() - 2) + 2;
    // println!("{}", lines.len());
    // println!("{}", lines[0].len());
    // println!("{:?}, {:?}", start_i..end_i, start_j ..end_j);
    // print!("Neighbours: ");
    for a in start_i..end_i {
        for b in start_j..end_j {
            // print!("{a}, {b}: ");
            if a == i && b == j {
                continue;
            }

            let neighbour = lines[a].chars().nth(b).unwrap();
            // print!("{}; ", neighbour);
            if !neighbour.is_ascii_digit() && neighbour != '.' {
                print!("{a}, {b}: ");
                println!("Neighbour: {}", neighbour);
                return Some(Part {
                    symbol: neighbour,
                    x: a,
                    y: b,
                });
            }
        }
    }
    // println!();

    None
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let lines: Vec<&str> = contents.lines().collect();

    let mut gears_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let mut sum: u32 = 0;
    let mut number = String::new();
    let mut part = None;
    for (i, line) in lines.iter().enumerate() {
        for (j, c) in line.char_indices() {
            // println!("Number: {}", number);
            if c.is_ascii_digit() {
                // println!("Symbol: {c}");
                number.push(c);
                part = get_adjacent_part(i, j, &lines).or(part);
            } else if let Some(p) = part {
                println!("Number: {}", number);
                let n = number.parse::<u32>().unwrap();
                sum += n;
                if p.symbol == '*' {
                    gears_map.entry((p.x, p.y)).or_default().push(n);
                }
                number = String::new();
                part = None;
            } else {
                part = None;
                number = String::new();
            }
        }
    }

    let mut gear_ratio_sum: u32 = 0;
    for (_, part_numbers) in gears_map.iter() {
        println!("{:?}", part_numbers);
        if part_numbers.len() == 2 {
            gear_ratio_sum += part_numbers[0] * part_numbers[1];
        }
    }

    println!("Sum of all of the part numbers: {sum}");
    println!("Sum of all of the gear ratios: {gear_ratio_sum}");
}
