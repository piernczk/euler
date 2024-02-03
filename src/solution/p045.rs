pub fn solution() -> String {
    // every hexagonal number is triangular, so we can ignore these
    let mut n = 144;
    let mut hn = n * (2 * n - 1);
    let mut m = 166;
    let mut pm = m * (3 * m - 1) / 2;
    loop {
        while pm > hn {
            // we know that:
            //     (n + 1)(2[n + 1] - 1) - n(2n - 1)
            //   = (n + 1)(2n + 1) - n(2n - 1)
            //   = 2n^2 + 3n + 1 - 2n^2 + n
            //   = 4n + 1
            // so instead of computing hn from scratch, we just add 4n + 1 to it
            hn += 4 * n + 1;
            n += 1;
        }

        if pm == hn {
            return pm.to_string();
        }

        // we do the same trick as above
        pm += 3 * m + 1;
        m += 1;
    }
}
