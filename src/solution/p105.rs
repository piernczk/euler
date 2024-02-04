use bitvec::{order::Lsb0, vec::BitVec};

use crate::get_data;

pub fn solution() -> String {
    let data = get_data(105).unwrap();
    let mut set_sum = 0;
    let mut sums: BitVec<usize, Lsb0> = BitVec::new();
    let mut highest = Vec::new();
    let mut subset = Vec::new();
    'sets: for line in data.lines() {
        let mut set: Vec<u32> = line.split(',').filter_map(|s| s.parse().ok()).collect();
        set.sort();
        let sum: u32 = set.iter().sum();

        sums.fill(false);
        sums.resize(sum as usize, false);
        highest.clear();

        for size in 1..=set.len() {
            subset.resize(size, 0);
            for (i, v) in subset.iter_mut().enumerate() {
                *v = i;
            }

            let mut first = true;
            highest.push('subsets: loop {
                let mut subsum = 0;
                for i in &subset {
                    subsum += set[*i];
                }

                if sums[subsum as usize - 1] {
                    // two different subsets have the same sum
                    continue 'sets;
                }
                sums.set(subsum as usize - 1, true);

                if first && highest.iter().any(|&l| l > subsum) {
                    // a smaller subset has a bigger sum than this one
                    continue 'sets;
                }

                first = false;
                for i in 0..size - 1 {
                    if subset[i] + 1 < subset[i + 1] {
                        subset[i] += 1;
                        for (j, v) in subset[..i].iter_mut().enumerate() {
                            *v = j;
                        }
                        continue 'subsets;
                    }
                }

                if subset[size - 1] + 1 < set.len() {
                    for (i, v) in subset[..size - 1].iter_mut().enumerate() {
                        *v = i;
                    }
                    subset[size - 1] += 1;
                } else {
                    break subsum;
                }
            });
        }

        set_sum += sum;
    }

    set_sum.to_string()
}
