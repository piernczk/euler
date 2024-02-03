const K: u32 = 12;

pub fn solution() -> String {
    let mut digits = Digits::new();
    let mut s = 0;
    loop {
        // For any K-tuple of sorted digits from 0 to 9 (with at least a single
        // 1), we pick one of the non-zero digits as the first one. Then we pick
        // K - 1 other digits. If there are any duplicates, we have to divide
        // this count by the product of factorials of numbers of digits in the
        // tuple. For example, for:
        //   (9, 9, 7, 4, 1, 1, 1, 0, 0)
        // Here we have 2 9s, 3 1s and 2 0s. We compute:
        //   (9 - 2) * (9 - 1)! / (2! * 3! * 2!)
        // The number we get is the number of unique permutations of the digits
        // that form a K-digit number.
        let p = (K - digits.zeros()) * fact(K - 1) / digits.dup_fact();

        // Suppose there are m K-digit numbers with the same digits. They form
        // a m-tuple which is sorted descending. For the nth number in the tuple
        // (1 <= n <= m), there are exactly n - 1 numbers in the tuple which are
        // greater than this number. These counts form a simple arithmetic
        // progression from 0 to m - 1.
        s += (p as u64) * (p as u64 - 1) / 2;

        // Take the next tuple of digits.
        if !digits.next() {
            // We have reached the tuple (9, 9, 9, ...).
            break;
        }
    }

    s.to_string()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Digits([u8; K as usize]);

impl Digits {
    pub fn new() -> Self {
        let mut arr = [0; K as usize];
        arr[0] = 1;
        Self(arr)
    }

    pub fn next(&mut self) -> bool {
        for i in (1..K as usize).rev() {
            if self.0[i] < self.0[i - 1] {
                self.0[i] += 1;
                self.0[i + 1..].fill(0);
                return true;
            }
        }

        if self.0[0] < 9 {
            self.0[0] += 1;
            self.0[1..].fill(0);
            true
        } else {
            false
        }
    }

    pub fn zeros(&self) -> u32 {
        self.0.iter().filter(|&&d| d == 0).count() as u32
    }

    pub fn dup_fact(&self) -> u32 {
        let mut count = [0; 10];
        for (i, d) in self.0.iter().enumerate() {
            if *d == 0 {
                count[0] = K - i as u32;
                break;
            }
            count[*d as usize] += 1;
        }
        count.iter().map(|&c| fact(c)).product()
    }
}

fn fact(k: u32) -> u32 {
    (1..=k).product()
}
