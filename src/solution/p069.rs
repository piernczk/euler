pub fn solution() -> String {
    const MAX_N: u32 = 1_000_000;

    let mut totient = vec![1; MAX_N as usize - 1];

    for n in 2..=MAX_N {
        if totient[n as usize - 2] == 1 {
            totient[n as usize - 2] = n - 1;
            for mut m in (2 * n..=MAX_N).step_by(n as usize) {
                let phi = &mut totient[m as usize - 2];
                *phi *= n - 1;
                m /= n;
                while m % n == 0 {
                    *phi *= n;
                    m /= n;
                }
            }
        }
    }

    totient.into_iter()
        .enumerate()
        .map(|(n, phi)| (n as u64 + 2, phi as u64))
        .max_by(|(n1, phi1), (n2, phi2)| (n1 * phi2).cmp(&(n2 * phi1)))
        .unwrap()
        .0
        .to_string()
}
