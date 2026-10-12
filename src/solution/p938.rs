pub fn solution() -> String {
    const R_IN: u32 = 24690;
    const B_IN: u32 = 12345;
    const R: u32 = R_IN / 2;
    const T: u32 = R_IN % 2;
    const B: u32 = B_IN;

    let mut arr = vec![0.0f64; R as usize + 1];
    arr[0] = 1.0;
    for b in 1..=B {
        for r in 1..=R {
            let t1 = (2 * r + T) * (2 * r + T - 1);
            let t2 = 2 * (2 * r + T) * b;
            let t = (t1 + t2) as f64;
            let p1 = t1 as f64 / t * arr[r as usize - 1];
            let p2 = t2 as f64 / t * arr[r as usize];
            arr[r as usize] = p1 + p2;
        }
    }

    format!("{:.10}", arr[R as usize])
}
