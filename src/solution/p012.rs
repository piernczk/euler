pub fn solution() -> String {
    let mut n = 1;
    for i in 2.. {
        n += i;
        let mut div = 0;
        let mut sqrt = (n as f32).sqrt() as u32;

        // we only need to check up to sqrt of n
        // for any j | n there are two new divisors: j and (n / j)
        // we don't care about sqrt itself now because it would add the same
        // divisor two times.
        sqrt -= u32::from(sqrt * sqrt == n);

        for j in 1..=sqrt {
            if n % j == 0 {
                div += 2;
                if div > 500 {
                    return n.to_string();
                }
            }
        }

        // if n is a perfect square and it has exactly 250 divisors smaller than
        // sqrt, then it has 501 divisors
        if sqrt * sqrt == n && div == 500 {
            return n.to_string();
        }
    }
    String::new()
}
