use std::collections::BinaryHeap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    len: u32,
    y: u8,
    x: u8,
}

pub fn solution() -> String {
    let mut queue = BinaryHeap::new();
    let mut visited = vec![0; NUMBERS.len()];
    let mut max = 0;
    queue.push(Node {
        len: NUMBERS[0],
        y: 0,
        x: 0,
    });

    while let Some(Node { len, y, x }) = queue.pop() {
        let index = (y as usize) * (y as usize + 1) / 2 + x as usize;
        if len < visited[index] || len + 99 * (HEIGHT as u32 - y as u32) < max {
            continue;
        }

        visited[index] = len;
        if y < HEIGHT as u8 - 1 {
            queue.push(Node {
                len: len + NUMBERS[(y as usize + 1) * (y as usize + 2) / 2 + x as usize],
                y: y + 1,
                x,
            });

            queue.push(Node {
                len: len + NUMBERS[(y as usize + 1) * (y as usize + 2) / 2 + x as usize + 1],
                y: y + 1,
                x: x + 1,
            });
        } else if len > max {
            max = len;
        }
    }

    max.to_string()
}

const HEIGHT: usize = 15;
#[rustfmt::skip]
const NUMBERS: [u32; HEIGHT * (HEIGHT + 1) / 2] = [
                                75,
                              95, 64,
                            17, 47, 82,
                          18, 35, 87, 10,
                        20,  4, 82, 47, 65,
                      19,  1, 23, 75,  3, 34,
                    88,  2, 77, 73,  7, 63, 67,
                  99, 65,  4, 28,  6, 16, 70, 92,
                41, 41, 26, 56, 83, 40, 80, 70, 33,
              41, 48, 72, 33, 47, 32, 37, 16, 94, 29,
            53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14,
          70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57,
        91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48,
      63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31,
     4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23,
];
