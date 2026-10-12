pub fn solution() -> String {
    let mut peter = Dice::new(4, 9);
    let mut colin = Dice::new(6, 6);

    peter.gen();
    colin.gen();

    peter.beats(&colin).to_string()
}

#[derive(Debug, Clone, PartialEq)]
struct Dice {
    prob: Vec<f64>,
    faces: u32,
    count: u32,
}

impl Dice {
    fn new(faces: u32, count: u32) -> Self {
        let mut prob = vec![0.0; faces as usize * count as usize + 1];
        prob[0] = 1.0;
        Self { prob, faces, count }
    }

    fn gen(&mut self) {
        let mut new_prob = vec![0.0; self.faces as usize * self.count as usize + 1];
        for _ in 0..self.count {
            let mut sum = 0.0;
            for (new, &current) in new_prob
                .iter_mut()
                .zip(&self.prob)
                .take(self.faces as usize)
            {
                *new = sum / self.faces as f64;
                sum += current;
            }

            for (i, new) in new_prob.iter_mut().enumerate().skip(self.faces as usize) {
                *new = sum / self.faces as f64;
                sum += self.prob[i] - self.prob[i - self.faces as usize];
            }

            std::mem::swap(&mut new_prob, &mut self.prob);
        }
    }

    fn beats(&self, other: &Self) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.prob.len().min(other.prob.len()) {
            sum += other.prob[0..i].iter().sum::<f64>() * self.prob[i];
        }
        sum
    }
}
