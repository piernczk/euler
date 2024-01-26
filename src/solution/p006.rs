pub fn solution() -> String {
    const N: u32 = 100;
    // the sum of the squares is a pyramidal number:
    let a = N * (N + 1) * (2 * N + 1) / 6;
    // the square of the partial sum of the arithmetic progression:
    let b = (N * (N + 1) / 2).pow(2);
    (b - a).to_string()
}
