pub fn solution() -> String {
    const NUMERATOR: u64 = 1;
    const DENOMINATOR: u64 = 12345;

    let mut n = 1;
    let mut p = 1;
    loop {
        n = (n << 1) | 1;
        if p * DENOMINATOR < n * NUMERATOR {
            break;
        }
        p += 1;
    }

    n = p * DENOMINATOR / NUMERATOR + 2;
    (n * (n - 1)).to_string()
}
