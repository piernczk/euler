pub fn solution() -> String {
    let mut number = 600851475143u64;
    let mut i = 2;
    let mut maxp = 1;
    while number > 1 {
        if number % i == 0 {
            number /= i;
            maxp = i;
        } else {
            i += 1;
        }
    }

    maxp.to_string()
}
