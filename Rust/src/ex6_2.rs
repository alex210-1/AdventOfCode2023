pub fn run() {
    // (time, distance)
    // let races = [(41, 214), (96, 1789), (88, 1127), (94, 1055)];
    let races = [(41968894u64, 214178911271055u64)];

    let ways: i64 = races
        .iter()
        .map(|(t, d)| {
            let t_half = (*t as f64) / 2.;
            let common = (t_half * t_half - *d as f64).sqrt();

            let min = (t_half - common).ceil() as i64;
            let max = (t_half + common).floor() as i64;

            println!("race t: {t}, d: {d} => min: {min}, max: {max}");

            max - min + 1
        })
        .product();

    println!("ways: {ways}");
}
