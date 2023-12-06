use std::fs;

const NUMBER_LITERALS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn get_digit(s: &str, window: usize) -> u32 {
    for i in 0..(s.len() - window + 1) {
        // println!("{i}");
        let w = &s.as_bytes()[i..i + window];
        // println!("{:?}", w.iter().map(|&b| b as char).collect::<String>());
        // println!("{}, {}", w[0], w[0] as char);
        if w[0].is_ascii_digit() {
            return (w[0] as char).to_digit(10).unwrap();
        }
        for (i, lit) in NUMBER_LITERALS.iter().enumerate() {
            if w.len() >= lit.len()
                && (&w[0..lit.len()] == lit.as_bytes()
                    || w[0..lit.len()].iter().eq(lit.as_bytes().iter().rev()))
            {
                // println!(
                //     "{:?}",
                //     w[0..lit.len()]
                //         .iter()
                //         .map(|&b| b as char)
                //         .collect::<String>()
                // );
                return i as u32 + 1;
            }
        }
    }
    0
}

fn get_calibration_value(s: &str) -> u32 {
    let window: usize = 5;
    let mut s = s.to_owned();
    let padding: String = String::from_utf8(vec![b'a'; window]).unwrap();
    s.insert_str(s.len(), &padding);
    s.insert_str(0, &padding);
    // println!("{s}");

    // println!("first digit");
    let mut first_digit = get_digit(&s, window);
    // println!("second_digit");
    let mut second_digit = get_digit(&s.chars().rev().collect::<String>(), window);
    // println!("{}, {}", first_digit, second_digit);

    if second_digit == 0 {
        second_digit = first_digit;
    } else if first_digit == 0 {
        first_digit = second_digit;
    }
    println!("{}", first_digit * 10 + second_digit);
    first_digit * 10 + second_digit
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut sum: u32 = 0;
    for word in contents.lines() {
        sum += get_calibration_value(word);
    }

    println!("Sum of all of the calibration values: {sum}");
}
