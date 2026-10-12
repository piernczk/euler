pub fn solution() -> String {
    // pretty darn slow, but works
    const N: u64 = 10u64.pow(8);
    let primes = gen_primes(N);

    let mut count = 0;
    for (i, &p) in primes.iter().enumerate() {
        let d = N / p;
        let j = match primes.binary_search(&d) {
            Ok(j) => j,
            Err(0) => break,
            Err(j) => j - 1,
        };

        if j >= i {
            count += j - i + 1;
        }
    }

    count.to_string()
}

fn gen_primes(mut max: u64) -> Vec<u64> {
    let mut p = Vec::new();
    p.extend(&[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    if max <= 30 {
        return p;
    }

    // a simple wheel sieve with the basis {2, 3, 5}
    let mut sieve = 1u64;
    for i in (1..30 / 2).map(|x| x * 2) {
        sieve |= 1 << i;
    }
    for i in (1..30 / 3).map(|x| x * 3) {
        sieve |= 1 << i;
    }
    for i in (1..30 / 5).map(|x| x * 5) {
        sieve |= 1 << i;
    }

    if max % 30 > 0 {
        max += 30 - (max % 30);
    }

    for (n, i) in (30..max).zip((0..30).cycle()) {
        if (sieve >> i) & 1 == 0 && is_prime(n) {
            p.push(n);
        }
    }

    p
}

fn is_prime(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }

    true
}
