pub fn solution() -> String {
    const GREY: usize = 50;
    let mut pos = vec![1, 1, 2, 4];
    pos.reserve_exact(GREY - 3);

    for g in 4..=GREY {
        pos.push(3 * pos.iter().take(g - 3).sum::<u64>() + 2 * pos[g - 3] + pos[g - 2] + 1);
    }

    pos.last().unwrap().to_string()
}
