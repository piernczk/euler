use crate::get_data;

pub fn solution() -> String {
    let mut n = 1;
    let mut tn = 1;
    let mut triangles = vec![1];

    let data = get_data(42).unwrap();
    let mut count = 0;
    for mut word in data.split(',') {
        word = word.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
        let mut value = 0;
        for c in word.bytes() {
            value += (c - b'A' + 1) as u32;
        }

        while value > tn {
            n += 1;
            tn += n;
            triangles.push(tn);
        }

        count += u32::from(triangles.iter().any(|&t| value == t));
    }

    count.to_string()
}
