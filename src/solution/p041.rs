const DIGITS: [u8; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solution() -> String {
    for i in (1..=9).rev() {
        let mut digits = Digits::new(i);
        if let Some(value) = digits.permute(i, &is_prime) {
            return value.to_string();
        }
    }

    "".to_string()
}

fn is_prime(n: u64) -> bool {
    let sqrt = (n as f64).sqrt() as u64;
    for i in 2..=sqrt {
        if n % i == 0 {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Digits(Vec<u8>);

impl Digits {
    pub fn new(digits: usize) -> Self {
        Self(Vec::from_iter(DIGITS.iter().take(digits).cloned()))
    }

    pub fn mov(&mut self, a: usize, b: usize) {
        let tmp = self.0[a];
        for i in a..b {
            self.0[i] = self.0[i + 1];
        }
        self.0[b] = tmp;
    }

    pub fn sort(&mut self, to: usize) {
        self.0[0..=to].sort();
    }

    pub fn value(&self) -> u64 {
        let mut pow: u64 = 1;
        let mut res: u64 = 0;
        for d in &self.0 {
            res += pow * *d as u64;
            pow *= 10;
        }
        res
    }

    pub fn permute<F>(&mut self, to: usize, until: &F) -> Option<u64>
    where
        F: Fn(u64) -> bool,
    {
        for i in 1..to {
            for j in (0..i).rev() {
                self.sort(i);
                self.mov(j, i);
                let value = self.value();
                if until(value) {
                    return Some(value);
                }
                if let Some(value) = self.permute(i - 1, until) {
                    return Some(value);
                }
            }
        }

        None
    }
}
