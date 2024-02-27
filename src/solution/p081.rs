use std::{cmp::Ordering, collections::BinaryHeap};

use crate::get_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    distance: u32,
    x: u8,
    y: u8,
}

pub fn solution() -> String {
    let data = get_data(81).unwrap();
    let mut matrix = Vec::with_capacity(80 * 80);
    for line in data.lines() {
        for s in line.split(',') {
            matrix.push(s.parse().unwrap())
        }
    }

    let mut distances = vec![u32::MAX; 80 * 80];
    let mut unvisited = BinaryHeap::new();
    unvisited.push(Point {
        distance: matrix[0],
        x: 0,
        y: 0,
    });

    while let Some(Point { distance, x, y }) = unvisited.pop() {
        if x == 79 && y == 79 {
            break;
        }

        let this = y as usize * 80 + x as usize;
        let right = this + 1;
        let down = this + 80;
        if x < 79 && distances[right] > distance + matrix[right] {
            distances[right] = distance + matrix[right];
            unvisited.push(Point {
                distance: distance + matrix[right],
                x: x + 1,
                y,
            })
        }

        if y < 79 && distances[down] > distance + matrix[down] {
            distances[down] = distance + matrix[down];
            unvisited.push(Point {
                distance: distance + matrix[down],
                x,
                y: y + 1,
            })
        }
    }

    distances.last().unwrap().to_string()
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}
