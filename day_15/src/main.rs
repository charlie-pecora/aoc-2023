use anyhow::Result;
use std::io;

fn main() -> Result<()> {
    let input = io::read_to_string(io::stdin())?;
    println!("{}", input);
    let mut hash_sum = 0_usize;
    let mut lens_boxes = LensBox::new();
    for step in input.trim().split(',') {
        let hash = calculate_hash(step);
        println!("{}, hash: {}", step, hash);
        hash_sum += hash;
        let command = Command::parse(step);
        lens_boxes.apply_command(command);
        println!("{:?}", lens_boxes);
    }
    println!("Part 1 hash sum: {}", hash_sum);
    println!("Part 2 score: {}", lens_boxes.score());

    Ok(())
}

fn calculate_hash(s: &str) -> usize {
    let mut current_value = 0_usize;
    for c in s.chars() {
        let ascii_code = c as usize;
        current_value += ascii_code;
        current_value *= 17;
        current_value = current_value % 256;
    }
    current_value
}

#[derive(Debug)]
struct LensBox {
    boxes: Vec<Vec<(String, u32)>>,
}

impl LensBox {
    fn new() -> LensBox {
        let boxes: Vec<Vec<(String, u32)>> =
            (0..256).map(|_| Vec::<(String, u32)>::new()).collect();
        LensBox { boxes }
    }

    fn apply_command(&mut self, command: Command) {
        match command {
            Command::Remove(label) => {
                let hash = calculate_hash(&label);
                let mut b = self.boxes[hash].clone();
                let mut lens_index: Option<usize> = None;
                for (i, lens) in b.iter().enumerate() {
                    if lens.0 == label {
                        lens_index = Some(i);
                        break;
                    }
                }
                match lens_index {
                    Some(i) => {
                        b.remove(i);
                    }
                    None => {}
                }
                self.boxes[hash] = b;
            }
            Command::Add(label, v) => {
                let hash = calculate_hash(&label);
                let mut b = self.boxes[hash].clone();
                let mut lens_index: Option<usize> = None;
                for (i, lens) in b.iter().enumerate() {
                    if lens.0 == label {
                        lens_index = Some(i);
                        break;
                    }
                }
                match lens_index {
                    Some(i) => {
                        b[i] = (label, v);
                    }
                    None => {
                        b.push((label, v));
                    }
                }
                self.boxes[hash] = b;
            }
            _ => {}
        }
    }

    fn score(&self) -> usize {
        let score = self
            .boxes
            .iter()
            .enumerate()
            .map(|(box_number, lens_box)| {
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(lens_number, lens)| {
                        (box_number + 1) * (lens_number + 1) * lens.1 as usize
                    })
                    .sum::<usize>()
            })
            .sum::<usize>();
        score
    }
}

enum Command {
    Remove(String),
    Add(String, u32),
    None,
}

impl Command {
    fn parse(s: &str) -> Command {
        if let Some(i) = s.find('-') {
            let label = &s[..i];
            return Command::Remove(label.to_string());
        } else if let Some(i) = s.find('=') {
            let label = &s[..i];
            let value = s[(i + 1)..].parse::<u32>().unwrap();
            return Command::Add(label.to_string(), value);
        } else {
            return Command::None;
        }
    }
}
