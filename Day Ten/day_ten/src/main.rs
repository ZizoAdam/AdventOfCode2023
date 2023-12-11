use colored::*;
use itertools::Itertools;
use std::fmt::Display;

const VERTICAL_PIPE: char = '|';
const HORIZONTAL_PIPE: char = '-';
const NORTH_EAST_PIPE: char = 'L';
const NORTH_WEST_PIPE: char = 'J';
const SOUTH_WEST_PIPE: char = '7';
const SOUTH_EAST_PIPE: char = 'F';
const GROUND: char = '.';
const ANIMAL_START: char = 'S';

#[derive(Clone, Copy, Debug, PartialEq)]
enum TileState {
    Undecided,
    External,
    Internal,
    MainLoop,
}

#[derive(Clone, Debug)]
struct Tile {
    char: char,
    state: TileState,
    connection_a: Option<usize>,
    connection_b: Option<usize>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = match self.state {
            TileState::Undecided => Color::White,
            TileState::External => Color::Red,
            TileState::Internal => Color::Blue,
            TileState::MainLoop => Color::Green,
        };

        write!(f, "{}", self.char.to_string().color(color))
    }
}

// Phantom types to ensure the correct order of operations
struct Unfilled {}
struct MainFilled {}
struct InternalFilled {}
struct Filled {}

struct Map<State = Unfilled> {
    tiles: Vec<Tile>,
    width: usize,
    animal_idx: usize,
    _state: State,
}

impl<State> Display for Map<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, tile) in self.tiles.iter().enumerate() {
            write!(f, "{}", tile)?;
            if idx % self.width == self.width - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

impl<State> Map<State> {
    fn get_tile_mut(&mut self, x: isize, y: isize) -> &mut Tile {
        &mut self.tiles[(y * self.width as isize + x) as usize]
    }

    fn get_state(&self, x: usize, y: usize) -> TileState {
        self.tiles[y * self.width + x].state
    }
}

impl Map<Unfilled> {
    fn from_str(s: &str) -> Map {
        let lines = s
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = lines[0].len();

        let mut tiles = Vec::new();

        let upper_conn = |x_idx: usize, y_idx: usize| -> Option<usize> {
            if y_idx == 0 {
                None
            } else {
                Some((y_idx - 1) * width + x_idx)
            }
        };

        let left_conn = |x_idx: usize, y_idx: usize| -> Option<usize> {
            if x_idx == 0 {
                None
            } else {
                Some(y_idx * width + (x_idx - 1))
            }
        };

        let right_conn = |x_idx: usize, y_idx: usize| -> Option<usize> {
            if x_idx == width - 1 {
                None
            } else {
                Some(y_idx * width + (x_idx + 1))
            }
        };

        let lower_conn = |x_idx: usize, y_idx: usize| -> Option<usize> {
            if y_idx == lines.len() - 1 {
                None
            } else {
                Some((y_idx + 1) * width + x_idx)
            }
        };

        let mut animal_idx = 0;
        for (y_idx, y_vec) in lines.iter().enumerate() {
            for (x_idx, char) in y_vec.iter().enumerate() {
                let tile = match *char {
                    VERTICAL_PIPE => Tile {
                        char: '|',
                        state: TileState::Undecided,
                        connection_a: upper_conn(x_idx, y_idx),
                        connection_b: lower_conn(x_idx, y_idx),
                    },
                    HORIZONTAL_PIPE => Tile {
                        char: '-',
                        state: TileState::Undecided,
                        connection_a: right_conn(x_idx, y_idx),
                        connection_b: left_conn(x_idx, y_idx),
                    },
                    NORTH_EAST_PIPE => Tile {
                        char: 'L',
                        state: TileState::Undecided,
                        connection_a: upper_conn(x_idx, y_idx),
                        connection_b: right_conn(x_idx, y_idx),
                    },
                    NORTH_WEST_PIPE => Tile {
                        char: 'J',
                        state: TileState::Undecided,
                        connection_a: upper_conn(x_idx, y_idx),
                        connection_b: left_conn(x_idx, y_idx),
                    },
                    SOUTH_WEST_PIPE => Tile {
                        char: '7',
                        state: TileState::Undecided,
                        connection_a: lower_conn(x_idx, y_idx),
                        connection_b: left_conn(x_idx, y_idx),
                    },
                    SOUTH_EAST_PIPE => Tile {
                        char: 'F',
                        state: TileState::Undecided,
                        connection_a: lower_conn(x_idx, y_idx),
                        connection_b: right_conn(x_idx, y_idx),
                    },
                    GROUND => Tile {
                        char: '.',
                        state: TileState::Undecided,
                        connection_a: None,
                        connection_b: None,
                    },
                    ANIMAL_START => {
                        animal_idx = y_idx * width + x_idx;
                        Tile {
                            char: 'S',
                            state: TileState::MainLoop,
                            connection_a: None,
                            connection_b: None,
                        }
                    }
                    _ => panic!("Invalid character in map: {char}"),
                };

                tiles.push(tile);
            }
        }

        let animal_connections = tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| {
                tile.connection_a == Some(animal_idx) || tile.connection_b == Some(animal_idx)
            })
            .map(|(index, _)| index)
            .collect_vec();

        // A quick bodge to connect up the animal to the rest of the map
        tiles[animal_idx].connection_a = Some(animal_connections[0]);
        tiles[animal_idx].connection_b = Some(animal_connections[1]);

        Map {
            tiles,
            width,
            animal_idx,
            _state: Unfilled {},
        }
    }

    fn flood_fill_main_loop(mut self) -> Map<MainFilled> {
        let next_idx = self.tiles[self.animal_idx].connection_a.unwrap();
        self.flood_fill_main_loop_(next_idx, self.animal_idx);

        Map {
            tiles: self.tiles,
            width: self.width,
            animal_idx: self.animal_idx,
            _state: MainFilled {},
        }
    }

    fn flood_fill_main_loop_(&mut self, cur_idx: usize, prev_idx: usize) {
        let cur_tile = &mut self.tiles[cur_idx];
        if cur_tile.state == TileState::MainLoop {
            return;
        }

        cur_tile.state = TileState::MainLoop;

        let next_idx = if cur_tile.connection_a == Some(prev_idx) {
            cur_tile.connection_b
        } else {
            cur_tile.connection_a
        };

        // Due to tail recursion this does not run in debug mode
        self.flood_fill_main_loop_(next_idx.unwrap(), cur_idx);
    }
}

impl Map<MainFilled> {
    fn main_loop_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::MainLoop)
            .count()
    }

    // If a ray is fired from a point within a 2d boundary and it crosses the boundary an odd number of times it's inside the boundary
    fn fill_internal(mut self) -> Map<InternalFilled> {
        let width = self.width;
        let height = self.tiles.len() / width;

        for (x, y) in (0..height).cartesian_product(0..width) {
            if self.get_state(x, y) == TileState::MainLoop {
                continue;
            }
            let mut crosses = 0u32;
            let (mut x2, mut y2) = (x, y);

            while x2 < width && y2 < height {
                let c2 = self.tiles[y2 * width + x2].char;
                if self.get_state(x2, y2) == TileState::MainLoop && c2 != 'L' && c2 != '7' {
                    crosses += 1;
                }
                x2 += 1;
                y2 += 1;
            }

            if crosses % 2 == 1 {
                self.get_tile_mut(x as isize, y as isize).state = TileState::Internal;
            }
        }

        Map {
            tiles: self.tiles,
            width: self.width,
            animal_idx: self.animal_idx,
            _state: InternalFilled {},
        }
    }
}

impl Map<InternalFilled> {
    fn main_loop_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::MainLoop)
            .count()
    }

    fn internal_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::Internal)
            .count()
    }

    fn fill_external(mut self) -> Map<Filled> {
        self.tiles.iter_mut().for_each(|tile| {
            if tile.state == TileState::Undecided {
                tile.state = TileState::External;
            }
        });

        Map {
            tiles: self.tiles,
            width: self.width,
            animal_idx: self.animal_idx,
            _state: Filled {},
        }
    }
}

impl Map<Filled> {
    fn main_loop_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::MainLoop)
            .count()
    }

    fn external_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::External)
            .count()
    }

    fn internal_count(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| tile.state == TileState::Internal)
            .count()
    }
}

fn main() {
    // Part 1: 6831 (777.1µs)
    let part1_start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let map = Map::from_str(input);
    let map = map.flood_fill_main_loop();
    let track_max = map.main_loop_count() / 2;
    let part1_elapsed = part1_start.elapsed();

    // Part 2: 305 (390.3µs)
    let part2_start = std::time::Instant::now();
    let map = map.fill_internal();
    let internal_count = map.internal_count();
    let part2_elapsed = part2_start.elapsed();

    // Filling the rest of the map for display purposes
    let map = map.fill_external();
    println!("{}", map);
    println!("Part 1: {track_max} ({part1_elapsed:?})");
    println!("Part 2: {internal_count} ({part2_elapsed:?})");
    println!(
        "External Tiles : {external_count}",
        external_count = map.external_count()
    );
    // Total time is 1.1674ms
    println!(
        "Total: {total_elapsed:?}",
        total_elapsed = part1_elapsed + part2_elapsed
    )
}
