pub fn solution() -> String {
    const MAX: u64 = 999966663333;

    let mut c = count_semdiv(2, 3, None);
    let mut p = 3;
    let mut q = 5;
    for i in 1.. {
        q = 6 * i - 1;
        if is_prime(q) {
            if q * q > MAX {
                break;
            }
            c += count_semdiv(p, q, None);
            p = q;
        }

        q = 6 * i + 1;
        if is_prime(q) {
            if q * q > MAX {
                break;
            }
            c += count_semdiv(p, q, None);
            p = q;
        }
    }

    c += count_semdiv(p, q, Some(MAX + 1));

    c.to_string()
}

fn count_semdiv(p: u64, q: u64, max: Option<u64>) -> u64 {
    let p2 = p * p;
    let q2 = q * q;
    let pq = p * q;

    let max = max.unwrap_or(q2);
    let firstq = p2 + q - p2 % q;

    let divp = aprog_sum(p2 + p, p, max);
    let divq = aprog_sum(firstq, q, max);
    let divpq = u64::from(max > pq) * pq;

    divp + divq - 2 * divpq
}

fn aprog_sum(first: u64, diff: u64, max: u64) -> u64 {
    let count = (max - first + diff - 1) / diff;
    count * (count - 1) / 2 * diff + first * count
}

fn is_prime(n: u64) -> bool {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }

    true
}
