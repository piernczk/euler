pub fn solution() -> String {
    const PRIMES: &[u32] = &[2, 3, 5, 7, 11, 13, 17, 19];
    let mut result = vec![0; PRIMES.len()];
    let mut current = result.clone();
    for mut a in 1..=20 {
        current.fill(0);
        let mut i = 0;
        while a > 1 {
            if a % PRIMES[i] == 0 {
                a /= PRIMES[i];
                current[i] += 1;
            } else {
                i += 1;
            }
        }

        for (c, r) in current.iter().zip(result.iter_mut()) {
            if c > r {
                *r = *c;
            }
        }
    }

    result
        .iter()
        .zip(PRIMES)
        .map(|(r, p)| p.pow(*r))
        .product::<u32>()
        .to_string()
}
