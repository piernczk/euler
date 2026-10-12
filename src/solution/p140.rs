pub fn solution() -> String {
    // One can prove that `A_G(x) = (x + 3x^2) / (1 - x - x^2)`. Setting `A_G(x) = p` gives us the
    // quadratic equation `(p + 3)x^2 + (p + 1)x - p = 0`. Its positive solution is rational if and
    // only if the discriminant, `\Delta = 5p^2 + 14p + 1`, is a perfect square. Equating
    // `\Delta = b^2` gives us the Pell-like equation `a^2 - 5b^2 = 44`, where `a = 5p + 7`.

    const COUNT: u64 = 30;

    let mut count = 0;
    let mut sum: u64 = 0;
    let mut ab = vec![(7, 1), (8, 2), (13, 5), (17, 7), (32, 14), (43, 19)];

    // we could get rid of this loop by swapping the order of operations in the following one, but
    // that would needlessly lead to a quicker integer overflow
    for (a, _) in &ab {
        if *a % 5 == 2 {
            let p = *a / 5 - 1;
            if p > 0 {
                count += 1;
                sum += p;
            }
        }
    }

    while count < COUNT {
        let (a, b) = ab.iter_mut().min().unwrap();
        // a' + b'\sqrt 5 = (a + b\sqrt 5)(9 + 4\sqrt 5)
        (*a, *b) = (9 * *a + 20 * *b, 4 * *a + 9 * *b);
        if *a % 5 == 2 {
            let p = *a / 5 - 1;
            count += 1;
            sum += p;
        }
    }

    sum.to_string()
}
