use std::cmp::Ordering;

pub fn solution() -> String {
    const NUMERATOR: u32 = 99;
    const DENOMINATOR: u32 = 100;

    let mut digits = Vec::new();
    digits.push(2);

    let mut all = NUMERATOR;
    let mut bouncy = 0;
    'outer: while bouncy < all {
        bouncy += DENOMINATOR * u32::from(is_bouncy(&digits));
        all += NUMERATOR;

        for d in digits.iter_mut() {
            if *d < 9 {
                *d += 1;
                continue 'outer;
            }

            *d = 0;
        }
        digits.push(1);
    }

    (all / NUMERATOR).to_string()
}

fn is_bouncy(s: &[u8]) -> bool {
    let mut ord = Ordering::Equal;
    let mut last = s.first().unwrap();
    for d in &s[1..] {
        let o = d.cmp(last);
        if o == Ordering::Equal {
            continue;
        }

        if ord == o.reverse() {
            return true;
        }

        last = d;
        ord = o;
    }

    false
}
