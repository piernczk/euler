pub fn solution() -> String {
    const MAX: u128 = 10u128.pow(12);
    const STEP: u128 = 10u128.pow(10);

    let mut sum = 1;
    let mut nums = Vec::with_capacity(STEP as usize / 10);
    for c in (0..MAX / STEP).map(|i| i * STEP) {
        nums.clear();
        let mut b = 2u128;
        while b.pow(3) <= (b - 1) * (c + STEP) + 1 {
            let mut n = ((b - 1) * c + 1).ilog(b).max(3);
            if b.pow(n) <= (b - 1) * c + 1 {
                n += 1;
            }
            while b.pow(n) <= (b - 1) * (c + STEP) + 1 {
                let x = (b.pow(n) - 1) / (b - 1);
                if let Err(i) = nums.binary_search(&x) {
                    nums.insert(i, x);
                }
                n += 1;
            }
            b += 1;
        }

        sum += nums.iter().sum::<u128>();
    }

    sum.to_string()
}
