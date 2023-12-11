use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Ace = 13,
    King = 12,
    Queen = 11,
    Ten = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
    // When determining hand state, wild cards are always the best card possible
    // When determining a draw, wild cards are always the lowest value
    Wildcard = 0,
}

impl Card {
    fn from_char<T>(c: T) -> Card
    where
        T: Eq + Copy + Into<char>,
    {
        match c.into() {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            'J' => Card::Wildcard,
            _ => unreachable!(),
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
    fn from_cards(cards: &[Card; 5]) -> HandState {
        let mut counts = [0; 14];
        for card in cards {
            counts[*card as usize] += 1;
        }

        let wild_card_count = counts[0];

        // We don't count wild cards towards the max until we determine the state
        let mut max_count: u8 = 0;
        let mut second_max_count: u8 = 0;

        for count in counts.iter().skip(1) {
            if *count > max_count {
                second_max_count = max_count;
                max_count = *count;
            } else if *count > second_max_count {
                second_max_count = *count;
            }
        }

        // Wild cards are always the best card possible
        match (max_count, second_max_count, wild_card_count) {
            (_, _, 5) => HandState::FiveOfAKind, // JJJJJ becomes JJJJJ
            (_, _, 4) => HandState::FiveOfAKind, // QJJJJ becomes QQQQQ
            (_, _, 3) => match max_count {
                2 => HandState::FiveOfAKind, // QQJJJ becomes QQQQQ
                _ => HandState::FourOfAKind, // QJJJK becomes QQQQK
            },
            (_, _, 2) => match max_count {
                3 => HandState::FiveOfAKind,  // QQQJJ becomes QQQQQ
                2 => HandState::FourOfAKind,  // QQKJJ becomes QQQQK
                _ => HandState::ThreeOfAKind, // QAKJJ becomes QQQAK
            },
            (_, _, 1) => match max_count {
                4 => HandState::FiveOfAKind, // QQQQJ becomes QQQQQ
                3 => HandState::FourOfAKind, // QQQAJ becomes QQQQA
                2 => match second_max_count {
                    2 => HandState::FullHouse,    // QQKKJ becomes QQKKQ
                    _ => HandState::ThreeOfAKind, // QQKAJ becomes QQKAQ
                },
                _ => HandState::OnePair, // QA12J becomes QA12Q
            },
            (_, _, 0) => match max_count {
                // If there are no wild cards, we can just use the normal logic
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
            },
            _ => unreachable!(), // This would mean there are more than 5 wild cards in the hand but a hand is only 5 cards
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
                (card_1, card_2, card_3, card_4, card_5)
            })
            .next()?;

        let cards = [card_1, card_2, card_3, card_4, card_5];

        let state = HandState::from_cards(&cards);

        Some(Hand { cards, state })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Bet {
    hand: Hand,
    amount: u32,
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

pub fn solve_part_2(s: &str) -> u32 {
    let game = Game::from_str(s);
    game.winnings()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_full_parse() {
        let input = super::Game::from_str(include_str!("../input.txt"));
        // This shouldn't panic
        assert_eq!(input.0.len(), 1000);
    }

    #[test]
    fn test_solve_part_2() {
        let input = super::Game::from_str(include_str!("../test_input.txt"));
        assert_eq!(input.winnings(), 5905);
    }

    // 32T3K One pair
    // T55J5 Four of a kind
    // KK677 Two pair
    // KTJJT Four of a kind
    // QQQJA Four of a kind
    #[test]
    fn test_wild_card() {
        let hand = super::Hand::from_str("32T3K").unwrap();
        assert_eq!(hand.state, super::HandState::OnePair);

        let hand = super::Hand::from_str("T55J5").unwrap();
        assert_eq!(hand.state, super::HandState::FourOfAKind);

        let hand = super::Hand::from_str("KK677").unwrap();
        assert_eq!(hand.state, super::HandState::TwoPair);

        let hand = super::Hand::from_str("KTJJT").unwrap();
        assert_eq!(hand.state, super::HandState::FourOfAKind);

        let hand = super::Hand::from_str("QQQJA").unwrap();
        assert_eq!(hand.state, super::HandState::FourOfAKind);
    }
}
