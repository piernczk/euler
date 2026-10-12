pub fn solution() -> String {
    // omg this is so terrible i hate it
    const N: usize = 30;
    let mut sequence = Vec::with_capacity(N);

    let max;
    let maxa;
    let mut i = 2u32;
    'outer: loop {
        for a in 1..i {
            let x = match (a as u64).checked_pow(i) {
                Some(x) => x,
                None => break,
            };
            if x >= 10 && a == digits_sum(x) {
                sequence.push(x);
                if sequence.len() == N {
                    max = x;
                    maxa = a;
                    break 'outer;
                }
            }
        }

        for n in 2..=i {
            let x = match (i as u64).checked_pow(n) {
                Some(x) => x,
                None => break,
            };
            if x >= 10 && i == digits_sum(x) {
                sequence.push(x);
                if sequence.len() == N {
                    max = x;
                    maxa = i;
                    break 'outer;
                }
            }
        }

        i += 1;
    }

    for a in 2..maxa {
        let mut x = a as u64;
        for _ in 2..=i {
            x = match x.checked_mul(a as u64) {
                Some(x) => x,
                None => break,
            };
            if x > max {
                break;
            }
            if x >= 10 && a == digits_sum(x) {
                sequence.push(x);
                break;
            }
        }
    }

    for a in i + u32::from(maxa == i)..(max as f32).sqrt().floor() as u32 {
        let mut x = a as u64;
        for _ in 2..=i {
            x = match x.checked_mul(a as u64) {
                Some(x) => x,
                None => break,
            };
            if x > max {
                break;
            }
            if x >= 10 && a == digits_sum(x) {
                sequence.push(x);
                break;
            }
        }
    }

    sequence.sort();
    sequence.dedup();
    sequence[N - 1].to_string()
}

fn digits_sum(mut n: u64) -> u32 {
    let mut sum = 0;
    while n > 0 {
        sum += (n % 10) as u32;
        n /= 10;
    }
    sum
}
