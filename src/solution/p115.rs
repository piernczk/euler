pub fn solution() -> String {
    const MIN_LENGTH: usize = 50;
    let mut pos = vec![1; MIN_LENGTH];
    for g in MIN_LENGTH.. {
        // uuugh..
        let n = pos
            .iter()
            .rev()
            .skip(MIN_LENGTH - 1)
            .enumerate()
            .map(|(i, &p)| i as u64 * p)
            .sum::<u64>()
            + g as u64
            - MIN_LENGTH as u64
            + 2;
        if n < 1_000_000 {
            pos.push(n);
        } else {
            return g.to_string();
        }
    }

    unreachable!()
}
