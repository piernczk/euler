use num_bigint::ToBigUint;

pub fn solution() -> String {
    // There are always 20 right and 20 down steps, they are just ordered
    // differently. This means we need to choose 20 steps from all 40, and
    // that's just a simple combination: `40! / (20! * 20!)`. This can be
    // simplified to:

    let mut result = 2.to_biguint().unwrap();
    for i in 21..=39usize {
        result *= i;
    }
    for i in 1..=19usize {
        result /= i;
    }

    result.to_string()
}
