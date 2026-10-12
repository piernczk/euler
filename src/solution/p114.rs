pub fn solution() -> String {
    const GREY: usize = 50;
    let mut pos = vec![1, 1, 1];
    pos.reserve_exact(GREY - 2);
    for g in 3..=GREY {
        // yeah whatever it works
        pos.push(
            pos.iter()
                .rev()
                .skip(2)
                .enumerate()
                .map(|(i, &p)| i as u64 * p)
                .sum::<u64>()
                + g as u64
                - 1,
        );
    }

    pos.last().unwrap().to_string()
}
