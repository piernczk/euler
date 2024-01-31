pub fn solution() -> String {
    for b in 1..1000u32 {
        for a in 1..=b {
            let c = ((a.pow(2) + b.pow(2)) as f32).sqrt() as u32;
            // double check the float -> int conversion
            if a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000 {
                return (a * b * c).to_string();
            }
        }
    }

    "".to_string()
}
