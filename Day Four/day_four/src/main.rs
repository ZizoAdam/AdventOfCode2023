use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct ScratchCard {
    old_score: u32,
    matches: usize,
}

fn from_str(s: &str) -> ScratchCard {
    let split_off_id = s.split(':').collect_vec();

    let all_numbers = split_off_id[1].split('|').collect_vec();

    let numbers = all_numbers[0]
        .split_whitespace()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect_vec();

    let winning_numbers = all_numbers[1]
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect_vec();

    let matches = numbers
        .iter()
        .filter(|&x| winning_numbers.contains(x))
        .count();

    let old_score = if matches == 0 {
        0
    } else {
        u32::pow(2, matches as u32 - 1)
    };

    ScratchCard { old_score, matches }
}

struct ScoreGenerator(Vec<ScratchCard>);

impl ScoreGenerator {
    fn get_card_number(&self, card_number: usize) -> &ScratchCard {
        unsafe { self.0.get_unchecked(card_number - 1) }
    }

    fn old_score(&self) -> u32 {
        self.0.iter().map(|x| x.old_score).sum()
    }

    fn new_score(&self) -> usize {
        let total = self.0.len();
        let cards = (0..self.0.len()).collect_vec();
        self.recurse(cards, total)
    }

    fn recurse(&self, cards: Vec<usize>, mut total: usize) -> usize {
        if cards.is_empty() {
            return total;
        }

        let new_cards = cards
            .into_iter()
            .filter_map(|index| {
                let matches = self.get_card_number(index).matches;
                if matches == 0 {
                    None
                } else {
                    Some((index + 1..index + matches + 1).collect_vec())
                }
            })
            .flatten()
            .collect_vec();
        total += new_cards.len();

        self.recurse(new_cards, total)
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let cards = ScoreGenerator(input.lines().map(from_str).collect_vec());
    let old_score = cards.old_score();
    let elapsed = start.elapsed();
    println!(
        "Old score: {} in {} micro seconds",
        old_score,
        elapsed.as_micros()
    );
    let start = std::time::Instant::now();
    let new_score = cards.new_score();
    let elapsed = start.elapsed();
    println!("New score: {} in {}ms", new_score, elapsed.as_millis());
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let card_6 = super::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(card_6.old_score, 0);
        assert_eq!(card_6.matches, 0);

        let card_1 = super::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(card_1.old_score, 8);
        assert_eq!(card_1.matches, 4);
    }
}
