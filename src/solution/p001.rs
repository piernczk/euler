pub fn solution() -> String {
    (count(3) + count(5) - count(15)).to_string()
}

fn count(n: u32) -> u32 {
    // naturals smaller than 1000 = naturals not greater than 999
    let m = 999 / n;
    n * (m * (m + 1)) / 2
}
