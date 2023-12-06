struct Races(Vec<Race<u32>>);

impl Races {
    fn from_str(s: &str) -> Races {
        let mut lines = s.lines();
        let times = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(str::parse)
            .map(Result::unwrap);
        let dists = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .map(str::parse)
            .map(Result::unwrap);

        Races(
            times
                .zip(dists)
                .map(|(time_ms, distance_record_mm)| Race {
                    time_ms,
                    distance_record_mm,
                })
                .collect(),
        )
    }

    fn part_1(&self) -> u32 {
        self.0.iter().map(Race::<u32>::win_count).product()
    }
}

#[derive(Debug, Clone, Copy)]
struct Race<T> {
    time_ms: T,
    distance_record_mm: T,
}
impl<T> Race<T>
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy + PartialOrd + Ord,
{
    fn does_win(&self, hold_time_ms: T) -> bool {
        ready_set_go(hold_time_ms, self.time_ms) > self.distance_record_mm
    }
}

impl Race<u32> {
    fn win_count(&self) -> u32 {
        let lowest = (1..self.time_ms).find(|&hold_time_ms| self.does_win(hold_time_ms));
        let highest = (1..self.time_ms)
            .rev()
            .find(|&hold_time_ms| self.does_win(hold_time_ms));

        match (lowest, highest) {
            (Some(low), Some(high)) => high - low + 1,
            _ => 0,
        }
    }
}

impl Race<u64> {
    fn part_2_from_str(s: &str) -> Race<u64> {
        let mut lines = s.lines();
        let time_ms = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s);
                acc
            })
            .parse()
            .unwrap();

        let distance_record_mm = lines
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            // Concatenate multi-digit numbers
            .fold(String::new(), |mut acc, s| {
                acc.push_str(s);
                acc
            })
            .parse()
            .unwrap();
        Race {
            time_ms,
            distance_record_mm,
        }
    }

    fn win_count(&self) -> u64 {
        let lowest = (1..self.time_ms).find(|&hold_time_ms| self.does_win(hold_time_ms));
        let highest = (1..self.time_ms)
            .rev()
            .find(|&hold_time_ms| self.does_win(hold_time_ms));

        match (lowest, highest) {
            (Some(low), Some(high)) => high - low + 1,
            _ => 0,
        }
    }
}

fn ready_set_go<T>(hold_time_ms: T, time_limit_ms: T) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    hold_time_ms * (time_limit_ms - hold_time_ms)
}

fn main() {
    let part_1_start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let races = Races::from_str(input);
    let part_1 = races.part_1();
    let part_1_elapsed = part_1_start.elapsed();
    println!("Part 1: {} ({:?})", part_1, part_1_elapsed);
    let part_2_start = std::time::Instant::now();
    let part_2 = Race::<u64>::part_2_from_str(input).win_count();
    let part_2_elapsed = part_2_start.elapsed();
    println!("Part 2: {} ({:?})", part_2, part_2_elapsed);
}

#[cfg(test)]
mod test {
    use crate::{Race, Races};

    #[test]
    fn parse() {
        let input = include_str!("../test_input.txt");
        let races = Races::from_str(input);
        let first = races.0.first().unwrap();
        assert_eq!(first.time_ms, 7);
        assert_eq!(first.distance_record_mm, 9);
        assert_eq!(first.win_count(), 4);

        let second = &races.0[1];
        assert_eq!(second.time_ms, 15);
        assert_eq!(second.distance_record_mm, 40);
        assert_eq!(second.win_count(), 8);

        let third = races.0.last().unwrap();
        assert_eq!(third.time_ms, 30);
        assert_eq!(third.distance_record_mm, 200);
        assert_eq!(third.win_count(), 9);

        assert_eq!(races.part_1(), 4 * 8 * 9)
    }

    #[test]
    fn ready_set_go() {
        assert_eq!(super::ready_set_go(1, 7), 6);
        assert_eq!(super::ready_set_go(2, 7), 10);
    }

    #[test]
    fn test_part2_parse() {
        let race = Race::part_2_from_str(include_str!("../test_input.txt"));

        assert_eq!(race.distance_record_mm, 940200);
        assert_eq!(race.time_ms, 71530);
        assert_eq!(race.win_count(), 71503)
    }
}
