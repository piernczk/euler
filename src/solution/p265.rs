pub fn solution() -> String {
    const N: u64 = 5;
    let bit_count = 1 << N;
    let mut mask = 0;
    for i in 0..N {
        mask |= 1 << i;
    }

    let root = 1 << (bit_count - N - 1) | 1;
    let mut tree = vec![root];
    let mut next = Vec::new();
    for i in (2..=bit_count - N - 1).rev() {
        next.clear();
        for &bits in &tree {
            let current = (bits >> i) & mask;
            let new = (current << 1) & mask;
            if new == 0 {
                continue;
            }

            if new != 1 << (N - 1) {
                let mut present = false;
                for j in i + 1..=bit_count - N - 1 {
                    let previous = (bits >> j) & mask;
                    if new == previous {
                        present = true;
                        break;
                    }
                }

                if !present {
                    next.push(bits);
                }
            }

            let new = new | 1;
            let mut present = false;
            for j in i..=bit_count - N {
                let previous = (bits >> j) & mask;
                if new == previous {
                    present = true;
                    break;
                }
            }

            if !present {
                next.push(bits | (1 << (i - 1)));
            }
        }

        std::mem::swap(&mut tree, &mut next);
    }

    let mut sum = 0u64;
    'leaf: for &bits in &tree {
        for j in 1..=bit_count - N - 1 {
            let previous = (bits >> j) & mask;
            for i in 0..N {
                if previous == (bits << i) & mask {
                    continue 'leaf;
                }
            }
        }

        sum += bits as u64;
    }

    sum.to_string()
}
