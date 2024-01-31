pub fn solution() -> String {
    // a simple wheel sieve with the basis {2, 3, 5}
    let mut sieve = 1u32;
    for i in (1..30 / 2).map(|x| x * 2) {
        sieve |= 1 << i;
    }
    for i in (1..30 / 3).map(|x| x * 3) {
        sieve |= 1 << i;
    }
    for i in (1..30 / 5).map(|x| x * 5) {
        sieve |= 1 << i;
    }

    // we're starting from 30
    let mut n = 30;
    let mut count = 10;
    loop {
        for i in 0..30 {
            if (sieve >> i) & 1 == 0 && is_prime(n) {
                count += 1;
                if count == 10001 {
                    return n.to_string();
                }
            }
            n += 1;
        }
    }
}

fn is_prime(n: u32) -> bool {
    let sqrt = (n as f32).sqrt() as u32;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }

    true
}
