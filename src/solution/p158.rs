pub fn solution() -> String {
    const N: u64 = 26;

    // p(n) = \binom{N}{n} * (2^n - n - 1)
    (1..=N).scan(1, |binom, n| { 
        *binom = *binom * (N + 1 - n) / n;
        Some(*binom * ((1 << n) - n - 1))
    }).max().unwrap().to_string()
}
