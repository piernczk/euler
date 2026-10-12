use crate::get_data;

pub fn solution() -> String {
    let mut sum = Vec::with_capacity(50);
    let nums_str = get_data(13).expect("no input data");
    let mut nums = Vec::with_capacity(100);
    for n in nums_str.split('\n') {
        if n.len() >= 50 {
            nums.push(n);
        }
    }

    let mut c = 0;
    for i in (0..50).rev() {
        let mut s = c + nums
            .iter()
            .map(|&s| (s.as_bytes()[i] - b'0') as u32)
            .sum::<u32>();
        c = s / 10;
        s %= 10;
        sum.push(b'0' + s as u8)
    }

    while c > 0 {
        sum.push(b'0' + (c % 10) as u8);
        c /= 10;
    }

    String::from_utf8(sum.into_iter().rev().take(10).collect()).unwrap()
}
