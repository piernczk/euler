pub fn solution() -> String {
    // This algorithm starts with a set D of 9 digits. At every step it
    // partitions D into a unique set of disjoint subsets. For any subset A it
    // counts how many of its permutations make up a prime number. Then these
    // counts are multiplied to get a total number of sets of prime numbers with
    // such partition of digits.
    let mut count = 0;
    let mut part = vec![0; 9];
    loop {
        let mut combinations = 1;
        for number in 0..=*part.iter().max().unwrap() {
            let mut digits: Vec<u8> = part
                .iter()
                .cloned()
                .enumerate()
                .filter(|&(_, x)| x == number)
                .map(|(i, _)| i as u8 + 1)
                .collect();

            let mut this_primes = 0;
            gen_permutations(&mut digits, |s| this_primes += u32::from(is_prime(s)));

            combinations *= this_primes;
            if this_primes == 0 {
                break;
            }
        }

        count += combinations;
        if !next_partition(&mut part) {
            break;
        }
    }

    count.to_string()
}

fn next_partition(part: &mut [u8]) -> bool {
    // here's a nice explanation
    // https://stackoverflow.com/a/30898130
    for i in 0..part.len() - 1 {
        if part[i + 1..].iter().rev().any(|&p| part[i] == p) {
            part[i] += 1;
            part[..i].fill(0);
            return true;
        }
    }

    false
}

fn gen_permutations<F>(digits: &mut [u8], mut f: F)
where
    F: FnMut(&[u8]),
{
    f(digits);

    let mut c = vec![0; digits.len()];
    let mut i = 0;
    while i < digits.len() {
        if c[i] < i {
            if i & 1 == 0 {
                digits.swap(0, i);
            } else {
                digits.swap(c[i], i);
            }

            f(digits);

            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
}

fn is_prime(digits: &[u8]) -> bool {
    let mut p = 1;
    let mut v = 0;
    for d in digits {
        v += *d as u32 * p;
        p *= 10u32;
    }

    for n in 2..=(v as f32).sqrt() as u32 {
        if v % n == 0 {
            return false;
        }
    }

    v >= 2
}
