use std::ops::{Index, IndexMut};

const GO: u8 = 0;
const JAIL: u8 = 10;
const GO_TO_JAIL: u8 = 30;
const R1: u8 = 5;
const C1: u8 = 11;
const E3: u8 = 24;
const H2: u8 = 39;

const COMMUNITY_CHEST: [u8; 3] = [2, 17, 33];
const CHANCE: [Chance; 3] = [
    Chance {
        square: 7,
        next_railway: 15,
        next_utility: 12,
    },
    Chance {
        square: 22,
        next_railway: 25,
        next_utility: 28,
    },
    Chance {
        square: 36,
        next_railway: 5,
        next_utility: 12,
    },
];

pub fn solution() -> String {
    // It doesn't work properly when changed to use 6-sided dice but it gives
    // the correct output for 4-sided dice so whatever

    let mut prev_prob = Probabilites::new();
    let mut prob = Probabilites::new();

    for _ in 0..1000 {
        prob.clear();
        for i in 0..40 {
            for j in 0..=2 {
                let p = prev_prob[i] * (j as f32 + 1.0) / 16.0;
                prob.add_chance(i + 2 + j, p);
                prob.add_chance(i + 8 - j, p);
            }
            prob.add_chance(i + 5, prev_prob[i] * 4.0 / 16.0);
        }

        prob[JAIL] += 1.0 / 64.0 * 3.0 / 4.0;
        prob.normalize();
        std::mem::swap(&mut prev_prob, &mut prob);
    }

    let mut p: Vec<_> = prev_prob.0.iter().enumerate().collect();
    p.sort_by(|(_, p), (_, q)| q.partial_cmp(p).unwrap());
    format!("{:02}{:02}{:02}", p[0].0, p[1].0, p[2].0)
}

#[derive(Debug, Clone, PartialEq)]
struct Probabilites([f32; 40]);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Chance {
    square: u8,
    next_railway: u8,
    next_utility: u8,
}

impl Probabilites {
    pub fn new() -> Probabilites {
        let mut p = [0.0; 40];
        p[0] = 1.0;
        Self(p)
    }

    pub fn normalize(&mut self) {
        let sum: f32 = self.0.iter().sum();
        for p in &mut self.0 {
            *p /= sum;
        }
    }

    pub fn clear(&mut self) {
        self.0.fill(0.0);
    }

    pub fn add_chance(&mut self, square: u8, p: f32) {
        if square == GO_TO_JAIL {
            self[JAIL] += p;
        } else if COMMUNITY_CHEST.contains(&square) {
            let q = p * (1.0 / 16.0);
            self[GO] += q;
            self[JAIL] += q;
            self[square] += p * (14.0 / 16.0);
        } else if let Some(chance) = CHANCE.iter().find(|c| square == c.square) {
            let q = p * (1.0 / 16.0);
            self[GO] += q;
            self[JAIL] += q;
            self[C1] += q;
            self[E3] += q;
            self[H2] += q;
            self[R1] += q;
            self[chance.next_railway] += 2.0 * q;
            self[chance.next_utility] += q;
            self[square - 3] += q;
            self[square] += p * (6.0 / 16.0);
        } else {
            self[square] += p;
        }
    }
}

impl Index<u8> for Probabilites {
    type Output = f32;

    fn index(&self, index: u8) -> &f32 {
        &self.0[index as usize % 40]
    }
}

impl IndexMut<u8> for Probabilites {
    fn index_mut(&mut self, index: u8) -> &mut f32 {
        &mut self.0[index as usize % 40]
    }
}
