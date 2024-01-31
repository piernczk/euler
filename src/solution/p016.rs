pub fn solution() -> String {
    // We just take the logarithm:
    //   10^x <= 2^1000 < 10^(x + 1)
    //   x <= [log(2^1000) = 1000log(2)] < x + 1
    // The result is x rounded down.

    ((1000.0 * 2.0f32.log10()) as u32).to_string()
}
