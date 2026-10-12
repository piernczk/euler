pub fn solution() -> String {
    const MAX: u64 = 7u64.pow(17);
    const ODDS: u64 = (MAX + 1) / 2;
    let mut pow = 2;
    let mut l = 0;
    while pow <= MAX + 1 {
        let h = pow - 1;
        let count = (ODDS + (pow >> 1)) / pow;
        l += 2 * count * h - 1;
        pow <<= 1;
    }

    l.to_string()
}
