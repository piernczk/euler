pub fn solution() -> String {
    const DIGITS: u32 = 1000;
    const EXP: u32 = DIGITS - 1; // 10^(n-1) hast n digits

    let sqrt5 = 5.0f32.sqrt();
    let phi = (1.0 + sqrt5) / 2.0;
    let log10sqrt5 = sqrt5.log10();
    let log10phi = phi.log10();
    let index = ((EXP as f32 + log10sqrt5) / log10phi).floor() as u32 + 1;

    index.to_string()
}
