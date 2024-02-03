use std::ops::Mul;

use nalgebra::{DMatrix, DVector};

pub fn solution() -> String {
    let polynomial = [1, -1, 1, -1, 1, -1, 1, -1, 1, -1, 1];
    let mut sequence = [0; 11];
    for (n, s) in sequence.iter_mut().enumerate() {
        let mut pow = 1;
        for c in &polynomial {
            *s += c * pow;
            pow *= n as i64 + 1;
        }
    }

    let mut sum = 0;
    for k in 1..polynomial.len() as u32 {
        let mut x = DMatrix::from_iterator(
            k as usize,
            k as usize,
            (0..k * k).map(|i| (i % k + 1).pow(i / k) as f64),
        );
        let y = DVector::from_iterator(
            k as usize,
            sequence.iter().take(k as usize).map(|&s| s as f64),
        );
        if !x.try_inverse_mut() {
            panic!();
        }

        let a: Vec<_> = x.mul(&y).into_iter().map(|&a| a.round() as i64).collect();

        for (n, s) in sequence.iter().enumerate().skip(k as usize) {
            let mut val = 0;
            let mut pow = 1;
            for c in &a {
                val += c * pow;
                pow *= n as i64 + 1;
            }

            if val != *s {
                sum += val;
                break;
            }
        }
    }

    sum.to_string()
}
