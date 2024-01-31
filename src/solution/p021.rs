pub fn solution() -> String {
    let mut d = Vec::with_capacity(10000);
    d.push(0);
    d.push(0);
    for n in 2..10000 {
        d.push(get_sum(n));
    }

    let mut sum = 0;
    for (a, b) in d.iter().cloned().enumerate() {
        if a != b as usize
            && (((b as usize) < d.len() && a == d[b as usize] as usize)
                || ((b as usize) >= d.len() && a == get_sum(b) as usize))
        {
            // if b < d.len() then b will be added later
            sum += a as u32;
        }
    }

    sum.to_string()
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
