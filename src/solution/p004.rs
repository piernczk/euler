pub fn solution() -> String {
    let mut max = 0;
    for a in (100..=999).rev() {
        if a * 999 <= max {
            break;
        }

        for b in (a..=999).rev() {
            let num = a * b;
            if num <= max {
                break;
            }
            if is_palindrome(num) {
                max = num;
            }
        }
    }

    max.to_string()
}

fn is_palindrome(n: u32) -> bool {
    (n >= 100_000
        && n % 10 == n / 100_000
        && (n / 10) % 10 == (n / 10_000) % 10
        && (n / 100) % 10 == (n / 1000) % 10)
        || (n < 100_000 && n % 10 == n / 10_000 && (n / 10) % 10 == (n / 1000) % 10)
}
