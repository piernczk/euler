use crate::get_data;

pub fn solution() -> String {
    let data = get_data(102).unwrap();
    let mut count = 0;
    for line in data.lines() {
        let mut iter = line.split(',').flat_map(|s| s.parse::<i32>().ok());
        let (ax, mut ay, bx, mut by, cx, mut cy) = (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        );

        // TODO use a u64 and count the negative bits
        let mut over_zero_count = 0;
        if ay > 0 {
            over_zero_count += 1;
        }
        if by > 0 {
            over_zero_count += 1;
        }
        if cy > 0 {
            over_zero_count += 1;
        }

        if over_zero_count == 0 || over_zero_count == 3 {
            continue;
        } else if over_zero_count == 2 {
            (ay, by, cy) = (-ay, -by, -cy);
        }

        let (x0, y0, x1, y1, x2, y2) = if ay > 0 {
            (ax, ay, bx, by, cx, cy)
        } else if by > 0 {
            (bx, by, ax, ay, cx, cy)
        } else {
            (cx, cy, ax, ay, bx, by)
        };

        over_zero_count = 0;
        if -y0 * (x1 - x0) / (y1 - y0) + x0 > 0 {
            over_zero_count += 1;
        }
        if -y0 * (x2 - x0) / (y2 - y0) + x0 > 0 {
            over_zero_count += 1;
        }

        count += u32::from(over_zero_count == 1);
    }

    count.to_string()
}
