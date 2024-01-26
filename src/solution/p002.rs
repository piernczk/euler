pub fn solution() -> String {
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;

    for i in 0.. {
        // If you sum any of the subsequent numbers in the sequence, you get:
        // odd + even -> odd
        // even + odd -> odd
        // odd + odd -> even
        // Then the cycle repeats. This means that every third number in the
        // Fibonacci sequence is even and the first even number is 2.
        if i % 3 == 0 {
            sum += b;
        }
        (a, b) = (b, a + b);
        if b > 4_000_000 {
            break;
        }
    }

    sum.to_string()
}
