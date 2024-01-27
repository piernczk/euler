use std::collections::BinaryHeap;

use crate::get_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Node {
    len: u32,
    y: u8,
    x: u8,
}

pub fn solution() -> String {
    let mut triangle: Vec<u32> = Vec::new();
    let data = get_data(67).unwrap();
    let mut height = 0;
    let mut x = 0;
    for s in data.split_ascii_whitespace() {
        triangle.push(s.parse().unwrap());
        if x < height {
            x += 1;
        } else {
            height += 1;
            x = 0;
        }
    }

    let mut queue = BinaryHeap::new();
    let mut visited = vec![0; triangle.len()];
    let mut max = 0;
    queue.push(Node {
        len: triangle[0],
        y: 0,
        x: 0,
    });

    while let Some(Node { len, y, x }) = queue.pop() {
        let index = y as usize * (y as usize + 1) / 2 + x as usize;
        if len < visited[index] || len + 99 * (height as u32 - y as u32) < max {
            continue;
        }

        visited[index] = len;
        if y < height as u8 - 1 {
            queue.push(Node {
                len: len + triangle[(y as usize + 1) * (y as usize + 2) / 2 + x as usize],
                y: y + 1,
                x,
            });

            queue.push(Node {
                len: len + triangle[(y as usize + 1) * (y as usize + 2) / 2 + x as usize + 1],
                y: y + 1,
                x: x + 1,
            });
        } else if len > max {
            max = len;
        }
    }

    max.to_string()
}
