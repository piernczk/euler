use std::collections::VecDeque;

fn is_square(num: u32) -> bool {
    let sqrt = num.isqrt();
    sqrt * sqrt == num
}

pub fn solution() -> String {
    const SOLUTIONS: u32 = 1_000_000;

    let mut solutions = 0;
    let mut bc_less = VecDeque::new();
    for m in 2.. {
        bc_less.push_back(0);
        bc_less.push_back(0);

        let a = m;
        let a2 = a * a;

        solutions += bc_less.pop_front().unwrap();
        if solutions > SOLUTIONS {
            return m.to_string();
        }

        for bc in 2..m / 2 {
            if is_square(a2 + bc * bc) {
                solutions += bc / 2;
                if solutions > SOLUTIONS {
                    return m.to_string();
                }
            }
        }

        for bc in m + 1..2 * m {
            if is_square(a2 + bc * bc) {
                bc_less[(bc - m - 1) as usize] += a / 2;
                solutions += (2 * m - bc) / 2 + 1;
                if solutions > SOLUTIONS {
                    return m.to_string();
                }
            }
        }
    }

    unreachable!()
}
