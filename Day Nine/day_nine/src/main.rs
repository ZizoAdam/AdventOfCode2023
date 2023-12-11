use itertools::Itertools;

struct HistoryGrid {
    grid: Vec<Vec<i64>>,
}

impl HistoryGrid {
    fn from_str(input: &str) -> Self {
        let grid = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(str::parse::<i64>)
                    .map(Result::unwrap)
                    .collect_vec()
            })
            .collect_vec();
        Self { grid }
    }
    fn from_input() -> Self {
        let input = include_str!("../input.txt");
        Self::from_str(input)
    }

    fn sum_all_next(&self) -> i64 {
        self.grid
            .iter()
            .map(|row| generate_next_in_sequence(row))
            .sum()
    }

    fn sum_all_prev(&self) -> i64 {
        self.grid
            .iter()
            .map(|row| generate_next_in_sequence(&row.iter().rev().copied().collect_vec()))
            .sum()
    }
}

fn generate_next_in_sequence(seq: &[i64]) -> i64 {
    if seq.iter().all(|x| seq.first().unwrap() == x) {
        *seq.first().unwrap()
    } else {
        let diffs = seq.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
        let next = generate_next_in_sequence(&diffs);
        seq.last().unwrap() + next
    }
}

fn main() {
    let start = std::time::Instant::now();
    let grid = HistoryGrid::from_input();
    let sum_next = grid.sum_all_next();
    let elapsed = start.elapsed();
    println!(
        "Sum of all next numbers in sequence: {} ({:?})",
        sum_next, elapsed
    );
    let start = std::time::Instant::now();
    let sum_prev = grid.sum_all_prev();
    let elapsed = start.elapsed();
    println!(
        "Sum of all prev numbers in sequence: {} ({:?})",
        sum_prev, elapsed
    );
}

#[cfg(test)]
mod test {
    #[test]
    fn gen_next() {
        let seq = vec![0, 3, 6];
        assert_eq!(super::generate_next_in_sequence(&seq), 9);

        let seq = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(super::generate_next_in_sequence(&seq), 68);
    }

    #[test]
    fn gen_prev() {
        let seq = [0, 3, 6].iter().rev().copied().collect::<Vec<_>>();
        assert_eq!(super::generate_next_in_sequence(&seq), -3);

        let seq = [10, 13, 16, 21, 30, 45]
            .iter()
            .rev()
            .copied()
            .collect::<Vec<_>>();
        assert_eq!(super::generate_next_in_sequence(&seq), 5);
    }
}
