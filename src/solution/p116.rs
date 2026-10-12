pub fn solution() -> String {
    const GREY: u128 = 50;
    let mut result = 0;
    for len in 2..=4 {
        for count in 1..=GREY / len {
            result += choose(GREY - count * (len - 1), count);
        }
    }

    result.to_string()
}

fn choose(n: u128, k: u128) -> u128 {
    if k << 1 >= n {
        ((k + 1)..=n).product::<u128>() / (2..=n - k).product::<u128>()
    } else {
        ((n - k + 1)..=n).product::<u128>() / (2..=k).product::<u128>()
    }
}
