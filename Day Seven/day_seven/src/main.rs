use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Card {
    Ace = 12,
    King = 11,
    Queen = 10,
    Jack = 9,
    Ten = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandState {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

impl HandState {
    fn from_cards(cards: &[Card]) -> HandState {
        let mut counts = [0; 13];
        for card in cards {
            counts[*card as usize] += 1;
        }

        let mut max_count: u8 = 0;
        let mut second_max_count: u8 = 0;

        for count in counts.iter() {
            if *count > max_count {
                second_max_count = max_count;
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }

        match max_count {
            5 => HandState::FiveOfAKind,
            4 => HandState::FourOfAKind,
            3 => match second_max_count {
                2 => HandState::FullHouse,
                _ => HandState::ThreeOfAKind,
            },
            2 => match second_max_count {
                2 => HandState::TwoPair,
                _ => HandState::OnePair,
            },
            _ => HandState::HighCard,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
    state: HandState,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.state.cmp(&other.state) {
            std::cmp::Ordering::Equal => match self.cards[0].cmp(&other.cards[0]) {
                std::cmp::Ordering::Equal => match self.cards[1].cmp(&other.cards[1]) {
                    std::cmp::Ordering::Equal => match self.cards[2].cmp(&other.cards[2]) {
                        std::cmp::Ordering::Equal => match self.cards[3].cmp(&other.cards[3]) {
                            std::cmp::Ordering::Equal => self.cards[4].cmp(&other.cards[4]),
                            ordering => ordering,
                        },
                        ordering => ordering,
                    },
                    ordering => ordering,
                },
                ordering => ordering,
            },
            ordering => ordering,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn from_str(s: &str) -> Option<Hand> {
        let (card_1, card_2, card_3, card_4, card_5) = s
            .chars()
            .chunks(5)
            .into_iter()
            .take(1)
            .map(|chunk| {
                let (card_1, card_2, card_3, card_4, card_5) =
                    chunk.map(Card::from_char).collect_tuple().unwrap();
                (
                    card_1.unwrap(),
                    card_2.unwrap(),
                    card_3.unwrap(),
                    card_4.unwrap(),
                    card_5.unwrap(),
                )
            })
            .next()?;

        let cards = [card_1, card_2, card_3, card_4, card_5];

        let state = HandState::from_cards(&cards);

        Some(Hand { cards, state })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Bet {
    amount: u32,
    hand: Hand,
}

impl Ord for Bet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand.cmp(&other.hand)
    }
}

impl PartialOrd for Bet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Bet {
    fn from_str(s: &str) -> Bet {
        let (hand, amount) = s.split_at(6);
        let amount = amount.parse::<u32>().unwrap();
        let hand = Hand::from_str(hand).unwrap();
        Bet { amount, hand }
    }
}

struct Game(Vec<Bet>);

impl Game {
    fn from_str(s: &str) -> Game {
        let mut bets = s.lines().map(Bet::from_str).collect_vec();
        bets.sort();
        Game(bets)
    }

    fn winnings(&self) -> u32 {
        //self.0.sort();
        self.0
            .iter()
            .enumerate()
            .map(|(index, bet)| bet.amount * (index as u32 + 1))
            .sum()
    }
}

fn main() {
    let part_1_start = std::time::Instant::now();
    let game = Game::from_str(include_str!("../input.txt"));
    let part_1_elapsed = part_1_start.elapsed();
    println!("Part 1: {} ({:?})", game.winnings(), part_1_elapsed);
    let part_2_start = std::time::Instant::now();
    let result = part_2::solve_part_2(include_str!("../input.txt"));
    let part_2_elapsed = part_2_start.elapsed();
    println!("Part 2: {} ({:?})", result, part_2_elapsed);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_full_parse() {
        let input = include_str!("../input.txt");
        let game = super::Game::from_str(input);
    }

    #[test]
    fn test() {
        let input = include_str!("../test_input.txt");
        let game = super::Game::from_str(input);
        assert_eq!(game.winnings(), 6440);
    }
}

pub mod part_2;
