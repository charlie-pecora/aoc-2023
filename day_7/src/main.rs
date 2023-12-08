use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    let mut hands: Vec<Hand> = vec![];
    for line in fs::read_to_string(filename)
        .expect("Could not open input file")
        .lines()
    {
        match Hand::parse(line) {
            Some(hand) => {
                println!("{hand:?}");
                hands.push(hand);
            }
            None => println!("Couldn't parse line {}", line),
        }
    }
    hands.sort();
    let mut score: u32 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let ranking = u32::try_from(i).expect("error parsing enumerate") + 1;
        score += ranking * hand.bid
    }
    println!("part 1 score: {score}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn parse(card_counts: HashMap<Card, u32>) -> HandType {
        let mut counts = card_counts.into_values().collect::<Vec<u32>>();
        counts.sort();
        counts.reverse();
        if counts[0] == 5 {
            return HandType::FiveOfAKind;
        } else if counts[0] == 4 {
            return HandType::FourOfAKind;
        } else if counts[0] == 3 {
            if counts[1] == 2 {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        } else if counts[0] == 2 {
            if counts[1] == 2 {
                return HandType::TwoPair;
            } else {
                return HandType::OnePair;
            }
        } else {
            return HandType::HighCard;
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    bid: u32,
    cards: Vec<Card>,
    hand_type: HandType,
}

impl Hand {
    fn parse(s: &str) -> Option<Hand> {
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }
        let mut cards: Vec<Card> = vec![];
        let mut card_counts = HashMap::<Card, u32>::new();
        for c in split[0].chars() {
            let card = match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::_9,
                '8' => Card::_8,
                '7' => Card::_7,
                '6' => Card::_6,
                '5' => Card::_5,
                '4' => Card::_4,
                '3' => Card::_3,
                '2' => Card::_2,
                _ => panic!("no such card {c}"),
            };
            cards.push(card);
            let count = card_counts.entry(card).or_insert(0);
            *count += 1;
        }
        return Some(Hand {
            bid: split[1].parse::<u32>().expect("invalid bid on hand {s}"),
            cards: cards,
            hand_type: HandType::parse(card_counts),
        });
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp != Ordering::Equal {
            return hand_type_cmp;
        }
        for i in 0..self.cards.len() {
            let card_ordering = self.cards[i].cmp(&other.cards[i]);
            if card_ordering != Ordering::Equal {
                return card_ordering;
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}
