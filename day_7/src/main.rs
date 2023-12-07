use std::fs;
use std::cmp::Ordering;

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
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    _9,
    _8,
    _7,
    _6,
    _5,
    _4,
    _3,
    _2,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Eq)]
struct Hand {
    bid: u32,
    cards: Vec<char>,
    hand_type: HandType,
}

impl Hand {
    fn parse(s: &str) -> Option<Hand> {
        let split = s.split(' ').collect::<Vec<&str>>();
        if split.len() != 2 {
            return None;
        }
        return Some(Hand {
            bid: split[1].parse::<u32>().expect("invalid bid on hand {s}"),
            cards: split[0].chars().collect::<Vec<char>>(),
            hand_type: HandType::HighCard,
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
