use std::{collections::HashSet, fs};

fn calculate_points(numbers: &[u32], winning_numbers: &[u32]) -> u32 {
    let numbers: HashSet<u32> = HashSet::from_iter(numbers.iter().cloned());
    let winning_numbers: HashSet<u32> = HashSet::from_iter(winning_numbers.iter().cloned());

    numbers.intersection(&winning_numbers).count() as u32
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum: u32 = 0;
    let lines: Vec<&str> = contents.lines().collect();

    let mut matches_per_card: Vec<u32> = vec![];

    for line in lines.iter() {
        let (_, line) = line.split_once(":").unwrap();
        let line = line.split_once('|').unwrap();
        let winning_numbers: Vec<u32> = line
            .0
            .split_whitespace()
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let numbers: Vec<u32> = line
            .1
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let matches: u32 = calculate_points(&numbers, &winning_numbers);
        if matches != 0 {
            sum += 2_u32.pow(matches - 1);
        }

        matches_per_card.push(matches);
    }
    println!("Sum of all of the part numbers: {sum}");

    let mut card_instances = vec![1; matches_per_card.len()];
    for (i, matches) in matches_per_card.iter().enumerate() {
        for j in 1..*matches + 1 {
            card_instances[i + j as usize] += card_instances[i];
        }
    }

    let scratchcards_sum: u32 = card_instances.iter().sum();

    println!("Sum of all scratchcards: {scratchcards_sum}");
}
