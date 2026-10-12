pub fn solution() -> String {
    const POW: usize = 100;
    let mut combinations: Vec<Vec<u64>> = Vec::with_capacity(POW - 1);
    combinations.push((1..=10).rev().collect());

    for p in 1..POW - 1 {
        let mut current = vec![0; 10];
        let mut v = 0;
        for (n, m) in combinations[p - 1]
            .iter()
            .rev()
            .zip(current.iter_mut().rev())
        {
            v += n;
            *m = v;
        }

        combinations.push(current);
    }

    let mut count = 9;
    for i in combinations {
        count += 2 * i[1..=8].iter().sum::<u64>() + i[9] + i[0] - 9;
    }

    count.to_string()
}
