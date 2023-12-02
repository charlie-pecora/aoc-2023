use anyhow::Result;
use std::fs;

#[derive(Debug)]
struct CubeSelection {
    blue: u32,
    green: u32,
    red: u32,
}

impl CubeSelection {
    fn is_possible(&self, given: &CubeSelection) -> bool {
        if (self.blue <= given.blue) && (self.green <= given.green) && (self.red <= given.red) {
            return true;
        } else {
            return false;
        }
    }

    fn parse_from_str(s: &str) -> Result<CubeSelection> {
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        let mut red: u32 = 0;
        for color in s.split(',') {
            let splits: Vec<&str> = color.trim().split(' ').collect();
            let color_name = splits[1];
            let color_value = splits[0].parse::<u32>()?;
            match color_name {
                "blue" => blue = color_value,
                "green" => green = color_value,
                "red" => red = color_value,
                _ => println!("something went wrong"),
            }
        }
        Ok(CubeSelection {
            blue: blue,
            green: green,
            red: red,
        })
    }
}

const GIVEN_CUBES: CubeSelection = CubeSelection {
    blue: 14,
    green: 13,
    red: 12,
};

#[derive(Debug)]
struct Game {
    id: u32,
    cube_selections: Vec<CubeSelection>,
}

impl Game {
    fn parse_from_str(s: &str) -> Result<Game> {
        let game_splits: Vec<&str> = s.split(':').collect();
        if game_splits.len() != 2 {
            anyhow::bail!("unexpected input");
        }
        let game_str = game_splits[0];
        let game_id = game_str.split(' ').collect::<Vec<&str>>()[1].parse::<u32>()?;
        let selections_str = game_splits[1];
        let cube_selections = selections_str
            .split(';')
            .map(|x| CubeSelection::parse_from_str(x).expect("Failed to parse cube selection"))
            .collect();
        Ok(Game {
            id: game_id,
            cube_selections: cube_selections,
        })
    }

    fn is_possible(&self, given: &CubeSelection) -> bool {
        for selection in &self.cube_selections {
            if !selection.is_possible(given) {
                return false;
            }
        }
        return true;
    }

    fn calculate_power(&self) -> u32 {
        let mut blue: u32 = 0;
        let mut green: u32 = 0;
        let mut red: u32 = 0;
        for cube_selection in &self.cube_selections {
            if cube_selection.blue > blue {
                blue = cube_selection.blue;
            }
            if cube_selection.green > green {
                green = cube_selection.green;
            }
            if cube_selection.red > red {
                red = cube_selection.red;
            }
        }
        return blue * green * red;
    }
}

fn main() -> Result<()> {
    let filename = "input.txt";
    let mut game_id_sum: u32 = 0;
    let mut power_sum: u32 = 0;
    for line in fs::read_to_string(filename)
        .expect("Could not open input file")
        .lines()
    {
        match Game::parse_from_str(line) {
            Ok(g) => {
                let is_possible = g.is_possible(&GIVEN_CUBES);
                let power = g.calculate_power();
                println!(
                    "power: {:?}, is possible: {:?}, {:?}",
                    power, is_possible, g
                );
                if is_possible {
                    game_id_sum += g.id;
                }
                power_sum += power;
            }
            Err(e) => println!("Couldn't parse line {:?}", e),
        }
    }
    println!("possible game id sum: {:?}", game_id_sum);
    println!("sum of game powers: {:?}", power_sum);
    Ok(())
}
