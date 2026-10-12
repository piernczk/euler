pub fn solution() -> String {
    // Heron's formula for `(a, a, a \pm 1)` yields
    // \[S^2 = \frac{1}{16} (a^2 - 1)(3a^2 \pm 4a + 1).\]
    // For S to be an integer, `(a^2 - 1)(3a^2 \pm 4a + 1)` must be a perfect square divisible
    // by 16. Direct substitution of a = 0, 1, ..., 15 shows that this number is divisible by 16 iff
    // a is odd.

    // we use u128 because we need ~30 bits for a and thus ~120 bits for a^4.
    const LIMIT: u128 = 1_000_000_000;

    let mut sum = 0;

    // check (a, a, a + 1)
    for a in (3..=(LIMIT - 1) / 3).step_by(2) {
        let a2 = a * a;
        let s = (a2 - 1) * (3 * a2 + 4 * a + 1);
        let sqrt = s.isqrt();
        sum += (sqrt * sqrt == s) as u128 * (3 * a + 1);
    }

    // check (a, a, a - 1)
    for a in (3..=(LIMIT + 1) / 3).step_by(2) {
        let a2 = a * a;
        let s = (a2 - 1) * (3 * a2 - 4 * a + 1);
        let sqrt = s.isqrt();
        sum += (sqrt * sqrt == s) as u128 * (3 * a - 1);
    }

    sum.to_string()
}
