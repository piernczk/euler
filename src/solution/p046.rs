pub fn solution() -> String {
    let mut p = Vec::new();
    const PRIMES_STEP: u32 = 3000;
    gen_primes(&mut p, PRIMES_STEP);

    let mut pi = 0;
    'main: for n in (1..).map(|i| 2 * i + 1) {
        let mut new_pi = p[pi..].iter().position(|&p| n < p);
        while new_pi.is_none() {
            pi = p.len();
            gen_primes(&mut p, PRIMES_STEP);
            new_pi = p[pi..].iter().position(|&p| n < p);
        }

        pi += new_pi.unwrap();
        // println!("{} {:?}", n, &p[..pi]);
        if n == p[pi - 1] {
            continue;
        }

        for p in &p[..pi] {
            // println!("{} {}", n, p);
            if is_perfect_square((n - p) / 2) {
                continue 'main;
            }
        }

        return n.to_string();
    }

    "".to_string()
}

fn is_perfect_square(n: u32) -> bool {
    let sqrt = (n as f32).sqrt() as u32;
    sqrt.pow(2) == n
}

fn gen_primes(p: &mut Vec<u32>, mut max: u32) {
    if p.is_empty() {
        p.extend(&[2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        if max <= 30 {
            return;
        }
        max -= 30;
    }

    if max % 30 > 0 {
        max += 30 - (max % 30);
    }

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

    let mut start = *p.last().unwrap();
    start += 30 - (start % 30);
    for (n, i) in (start..(start + max)).zip((0..30).cycle()) {
        if (sieve >> i) & 1 == 0 && is_prime(n) {
            p.push(n);
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
