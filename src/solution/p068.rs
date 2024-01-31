pub fn solution() -> String {
    // For a magic n-gon filled with numbers {1, 2, .. 2n}, the maximal sum m in
    // a line must lie between
    //   2n + 1 + 2 = 2n + 3
    // Which we get by putting the 3 smallest numbers inside, and
    //   1 + 2n + 2n - 1 = 4n
    // Which we get by putting the 3 largest numbers inside.
    //
    // In order to express m as a sum of three different positive integers, we
    // need to choose 2 elements from a set with cardinality m - 1, so there are
    //   (m - 1)! / (2! * (m - 1 - 2)!) = (m - 2)(m - 1) / 2
    // possible triples of unique positive numbers that sum up to m. This means
    // that we need to check up to
    //   (n / 2) * \sum{m = 2n + 3}{4n} (m - 2)(m - 1)
    // triples of numbers for a magic n-gon. This equals 4600 for n = 5, so we
    // could just brute force it at this point.
    //
    // Once we have x such that there exist some magic n-gons with x being the
    // smallest outer number, one of these ngons is the solution to our problem.
    //
    // There is a way to express a magic n-gon as a n-by-2n bit-matrix whose
    // rows are the lines of the n-gon and the columns are the numbers from 1 to
    // 2n. The matrix must satisfy the following properties:
    //   - all n rows have exactly 3 ones each
    //   - n columns have 2 ones (these are the inner numbers which are always
    //     an intersection of two lines),
    //   - the other n columns have a single one (they are the outer numbers).
    // As I found out, this idea is a bit over-engineered and doesn't lead to
    // anything other than what this algorithm does.
    //
    // Also, this algorithm is pretty fast for odd n, but extremely slow for
    // even n. It doesn't necessarily work, but it does its job for n = 5.

    const N: usize = 49;
    const MAX: usize = 2 * N;
    let mut outer: Vec<_> = (N + 1..=MAX).rev().collect();
    let mut inner = vec![1; N];
    let mut sum = 2 * N + 3;
    'main: loop {
        inner.fill(1);
        let mut rowi = 2;
        if outer.contains(&1) || sum - outer[0] - 1 > MAX || outer.contains(&(sum - outer[0] - 1)) {
            rowi = 0;
        } else if sum - outer[1] - 1 > MAX || outer.contains(&(sum - outer[1] - 1)) {
            rowi = 1;
        }

        while rowi < N {
            if 2 * inner[rowi] + 2 < sum - outer[rowi] && inner[rowi] < MAX {
                inner[rowi] += 1;
                if sum - outer[rowi] - inner[rowi] <= MAX
                    && !outer.contains(&inner[rowi])
                    && !outer.contains(&(sum - outer[rowi] - inner[rowi]))
                    && two_in_col(&inner, &outer, MAX, rowi, sum)
                {
                    rowi += 1;
                }
            } else if rowi > 0 {
                rowi -= 1;
            } else if sum < 4 * N {
                sum += 1;
                continue 'main;
            } else {
                sum = 2 * N + 3;
                for i in 0..N {
                    if (i < N - 1 && outer[i] > outer[i + 1] + 1) || (i == N - 1 && outer[i] > 1) {
                        outer[i] -= 1;
                        outer
                            .iter_mut()
                            .enumerate()
                            .take(i)
                            .map(|(j, o)| *o = MAX - j)
                            .count();
                        continue 'main;
                    }
                }

                return "wrong".to_string();
            }
        }

        let mut rowi = N - 1;
        let mut current_farther = MAX + 1;
        let mut first = None;
        let mut output = String::with_capacity(3 * (N + 1));
        for _ in 0..N {
            let mut farther = inner[rowi];
            let mut closer = sum - outer[rowi] - farther;
            if closer == current_farther {
                (farther, closer) = (closer, farther);
            }

            let to_push = String::from_utf8_lossy(
                &format!("{}{}{}", outer[rowi], closer, farther)
                    .bytes()
                    .rev()
                    .collect::<Vec<_>>(),
            )
            .to_string();
            output.push_str(&to_push);
            first.get_or_insert(to_push);

            for (j, inner_no) in inner.iter().enumerate() {
                if closer == *inner_no || closer == sum - outer[j] - inner_no {
                    rowi = j;
                    current_farther = closer;
                    break;
                }
            }
        }

        let first = first.unwrap();
        output.push_str(&first);
        return String::from_utf8_lossy(
            &output.bytes().skip(first.len()).rev().collect::<Vec<_>>(),
        )
        .into_owned();
    }
}

fn two_in_col(inner: &[usize], outer: &[usize], max: usize, rowi: usize, sum: usize) -> bool {
    let mut counts = vec![0; max];
    for (a, outer_no) in inner.iter().zip(outer.iter()).take(rowi + 1) {
        if counts[*a - 1] == 2 || counts[sum - outer_no - *a - 1] == 2 {
            return false;
        }
        counts[*a - 1] += 1;
        counts[sum - outer_no - *a - 1] += 1;
    }

    true
}
