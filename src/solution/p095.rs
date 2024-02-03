pub fn solution() -> String {
    let mut d = Vec::with_capacity(1_000_000);
    d.push(0);
    d.push(0);
    for n in 2..1_000_000 {
        d.push(get_sum(n));
    }

    let mut current = Vec::new();
    let mut chains = vec![None; 1_000_000];
    for mut n in 2..1_000_000u32 {
        current.clear();
        while n < 1_000_000 && chains[n as usize].is_none() && !current.contains(&n) {
            current.push(n);
            n = d[n as usize];
        }

        let i = current
            .iter()
            .position(|&m| n == m)
            .unwrap_or(current.len());
        for m in &current[0..i] {
            chains[*m as usize] = Some(0);
        }
        for m in &current[i..] {
            chains[*m as usize] = Some(current.len() - i);
        }
    }

    chains
        .iter()
        .enumerate()
        .flat_map(|(n, l)| l.map(|x| (n, x)))
        .rev()
        .max_by_key(|(_, l)| *l)
        .map(|(n, _)| n)
        .unwrap()
        .to_string()
}

fn get_sum(n: u32) -> u32 {
    let sqrt = (n as f32).sqrt().ceil() as u32;
    let mut sum = 1;
    for a in 2..sqrt {
        if n % a == 0 {
            sum += a + n / a;
        }
    }

    // we're treating perfect squares differently to not include the square
    // root as two different divisors
    if sqrt * sqrt == n {
        sum += sqrt;
    }

    sum
}
