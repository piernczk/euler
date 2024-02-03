pub fn solution() -> String {
    let mut count = 0;
    'outer: for n in 1..=100 {
        let half = n >> 1;
        for k in 1..=half {
            let mut fact = 1.0f64;
            for i in 2..=k {
                fact /= i as f64;
            }

            for i in n - k + 1..=n {
                fact *= i as f64;
                if fact > 1_000_000.0 {
                    count += 2 * (half - k) + 1 + u64::from(half << 1 != n);
                    continue 'outer;
                }
            }
        }
    }

    count.to_string()
}
