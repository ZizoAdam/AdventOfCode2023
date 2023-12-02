use anyhow::Context;
use phf::phf_map;

static COLORS: phf::Map<&'static str, CubeColor> = phf_map! {
    "red" => CubeColor::Red,
    "green" => CubeColor::Green,
    "blue" => CubeColor::Blue,
};

pub struct GameHistory {
    pub games: Vec<Game>,
}

impl GameHistory {
    pub fn new(log: &str) -> anyhow::Result<GameHistory> {
        let games = log
            .lines()
            .map(Game::new)
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();
        Ok(GameHistory { games })
    }

    pub fn sum_game_ids_with_cubes(&self, red: u32, green: u32, blue: u32) -> u32 {
        self.games
            .iter()
            .filter(|game| game.has_enough_cubes(red, green, blue))
            .map(|game| game.id)
            .sum()
    }

    pub fn sum_min_power(&self) -> u32 {
        self.games.iter().map(Game::min_power).sum()
    }
}

pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn new(log: &str) -> anyhow::Result<Game> {
        let split = log.split(':').collect::<Vec<_>>();
        let id = Game::get_id(split[0])?;

        let draws = split[1]
            .split(';')
            .map(Draw::new)
            .map(|result| result.unwrap())
            .collect::<Vec<_>>();

        Ok(Game { id, draws })
    }

    fn get_id(id_split: &str) -> anyhow::Result<u32> {
        let id = id_split[5..].parse::<u32>()?;
        Ok(id)
    }

    pub fn has_enough_cubes(&self, red: u32, green: u32, blue: u32) -> bool {
        self.draws
            .iter()
            .all(|draw| draw.has_enough_cubes(red, green, blue))
    }

    pub fn minimum_cubes(&self) -> MinCubes {
        self.draws
            .iter()
            .fold(MinCubes::default(), |mut min_cubes, draw| {
                if draw.red > min_cubes.red {
                    min_cubes.red = draw.red;
                }
                if draw.green > min_cubes.green {
                    min_cubes.green = draw.green;
                }
                if draw.blue > min_cubes.blue {
                    min_cubes.blue = draw.blue;
                }
                min_cubes
            })
    }

    pub fn min_power(&self) -> u32 {
        let min_cubes = self.minimum_cubes();
        min_cubes.red * min_cubes.green * min_cubes.blue
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct MinCubes {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Draw {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl Draw {
    pub fn new(draw_split: &str) -> anyhow::Result<Draw> {
        let split = draw_split.split(',').collect::<Vec<_>>();
        let mut red = 0;
        let mut blue = 0;
        let mut green = 0;

        split
            .into_iter()
            .map(Draw::get_color)
            .map(|result| result.unwrap())
            .for_each(|(amount, color)| match color {
                CubeColor::Red => red += amount,
                CubeColor::Green => green += amount,
                CubeColor::Blue => blue += amount,
            });

        Ok(Draw { red, green, blue })
    }

    fn get_color(color: &str) -> anyhow::Result<(u32, CubeColor)> {
        let split = color.split(' ').collect::<Vec<_>>();

        let amount = split[1].parse()?;

        let color = COLORS.get(split[2]).context("Invalid color")?;

        Ok((amount, *color))
    }

    pub fn has_enough_cubes(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }
}