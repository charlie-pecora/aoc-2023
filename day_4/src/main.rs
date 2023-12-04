use anyhow::{bail, Result};
use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() -> Result<()> {
    let filename = "input.txt";
    let mut card_score_sum: u32 = 0;
    let mut total_cards: u32 = 0;
    let mut card_copies_queue = VecDeque::<u32>::new();
    for line in fs::read_to_string(filename)
        .expect("Could not open input file")
        .lines()
    {
        match Card::parse_from_str(line) {
            Ok(card) => {
                let card_copies = match card_copies_queue.pop_front() {
                    Some(v) => v,
                    None => 0,
                };
                total_cards += 1 + card_copies;
                card_score_sum += card.score();
                for _ in 0..(card_copies + 1) {
                    for i in 0..card.matches() {
                        match card_copies_queue.get_mut(i) {
                            Some(v) => *v += 1,
                            None => card_copies_queue.push_back(1),
                        };
                    }
                }
            }
            Err(e) => println!("Couldn't parse line {:?} {}", e, line),
        }
    }
    println!("card score sum: {}", card_score_sum);
    println!("total number of cards {}", total_cards);
    Ok(())
}

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn matches(&self) -> usize {
        let mut num_matches: usize = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(n) {
                num_matches += 1
            }
        }
        num_matches
    }
    fn score(&self) -> u32 {
        let mut num_matches: u32 = 0;
        for n in &self.my_numbers {
            if self.winning_numbers.contains(n) {
                num_matches += 1
            }
        }
        if num_matches > 0 {
            2_u32.pow(num_matches - 1)
        } else {
            0
        }
    }

    fn parse_from_str(s: &str) -> Result<Card> {
        let card_split = s.split(':').collect::<Vec<&str>>();
        if card_split.len() != 2 {
            bail!("couldn't parse Card");
        }
        let numbers_split = card_split[1].split('|').collect::<Vec<&str>>();
        if numbers_split.len() != 2 {
            bail!("couldn't parse Card");
        }
        let mut winning_numbers = HashSet::<u32>::new();
        for n in numbers_split[0].trim().split(' ') {
            if n.len() != 0 {
                let v = n.parse::<u32>()?;
                winning_numbers.insert(v);
            }
        }
        let mut my_numbers = Vec::<u32>::new();
        for n in numbers_split[1].trim().split(' ') {
            if n.len() != 0 {
                let v = n.parse::<u32>()?;
                my_numbers.push(v);
            }
        }
        Ok(Card {
            winning_numbers,
            my_numbers,
        })
    }
}
