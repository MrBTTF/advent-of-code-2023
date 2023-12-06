fn main() {
    let races: [(u64, u64); 1] = [(49979494, 263153213781851)];
    // let races: [(u64, u64); 1] = [(71530, 940200)];

    let mut number = 1;
    for race in races.iter() {
        let t = race.0 as f64;
        let d = race.1 as f64;

        let D = (t * t - 4.0 * d).sqrt();
        let x1 = (t - D) / 2.0;
        let x2 = (t + D) / 2.0;

        let mut ways_to_win = x2.floor() - x1.floor();
        if x2.floor() == x2 {
            ways_to_win -= 1.0
        }
        println!("{ways_to_win}");
        number *= ways_to_win as u64;
    }

    println!("Product of number of ways to win: {number}");
}
