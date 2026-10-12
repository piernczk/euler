pub fn comp(count: u64, mut dims: &[u64], must_be_even: bool) -> u64 {
    let dim;
    (dim, dims) = match dims.split_first() {
        Some(v) => v,
        None => return count,
    };

    let child_count = 1 << dim;
    if must_be_even {
        count * comp(child_count, dims, true)
    } else {
        count * (comp(child_count.pow(2) - child_count, dims, true) + comp(child_count, dims, false))
    }
}

pub fn solution() -> String {
    comp(24, &[8, 9, 10], false).to_string()
}
