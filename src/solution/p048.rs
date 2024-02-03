pub fn solution() -> String {
    let mut sum = 0;
    for n in 1..=1000u64 {
        let mut pow = 1;
        for _ in 0..n {
            pow = (pow * n) % 10_000_000_000;
        }
        sum += pow;
        // can't go higher than 19_999_999_998
        if sum >= 10_000_000_000 {
            sum -= 10_000_000_000;
        }
    }

    sum.to_string()
}
