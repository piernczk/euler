use std::collections::HashMap;

pub fn solution() -> String {
    let mut max_start = 1;
    let mut max_len = 0;
    for i in 2..1_000_000u64 {
        let mut n = i;
        let mut l = 0;
        while n > 1 {
            l += 1;
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }

        if l > max_len {
            max_start = i;
            max_len = l;
        }
    }

    max_start.to_string()
}

#[allow(dead_code)]
pub fn solution_cache() -> String {
    // this approach is slower and consumes more memory, but maybe it has potential?
    let mut cache: HashMap<u64, u32> = HashMap::new();
    cache.insert(1, 0);
    let mut max_start = 1;
    let mut max_len = 0;
    let mut current = Vec::new();
    for i in 2..1_000_000u64 {
        current.clear();
        let mut n = i;
        loop {
            current.push(n);
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
            if let Some(len) = cache.get(&n).cloned() {
                for (i, v) in current.iter().rev().enumerate() {
                    cache.insert(*v, len + i as u32);
                }
                if current.len() as u32 + len > max_len {
                    max_start = i;
                    max_len = current.len() as u32 + len;
                }
                break;
            }
        }
    }

    max_start.to_string()
}
