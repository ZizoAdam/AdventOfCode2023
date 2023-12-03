use itertools::Itertools;
const GEAR: char = '*';

struct CharGrid {
    // The grid is stored as a single vector of chars instead of a 2d Vec<char> to make my life more difficult
    grid: Vec<char>,
    width: usize,
}

impl CharGrid {
    fn from_str(s: &str) -> CharGrid {
        let width = s.lines().next().unwrap().chars().count();

        let grid: Vec<char> = s
            .lines()
            .flat_map(|l| l.chars().collect::<Vec<_>>())
            .collect();

        CharGrid { grid, width }
    }
    fn len(&self) -> usize {
        self.grid.len()
    }

    fn get_gear_ratio(&self) -> u32 {
        // Find all the gear symbols
        let gear_indices = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(i, c)| if *c == GEAR { Some(i) } else { None })
            .collect::<Vec<_>>();

        // Get the numbers adjacent to the gear symbols and determine the ratio
        gear_indices
            .into_iter()
            .map(|i| self.get_adjacent_numbers(i))
            .filter(|n| n.len() == 2)
            .map(|n| n.iter().map(|n| n.number).product::<u32>())
            .sum()
    }

    fn get_numbers(&self) -> Vec<GridNumber> {
        // Find all the symbols
        let symbol_indices = self
            .grid
            .iter()
            .enumerate()
            .filter_map(|(i, c)| {
                if c.is_ascii_digit() || c == &'.' {
                    None
                } else {
                    Some(i)
                }
            })
            .collect::<Vec<_>>();

        // Get the numbers adjacent to the symbols
        symbol_indices
            .into_iter()
            .flat_map(|i| self.get_adjacent_numbers(i))
            // Some numbers may be adjacent to multiple symbols so dedupe them
            .unique()
            .collect()
    }

    fn get_number(&self, index: usize) -> Option<GridNumber> {
        // If the value at the index isn't a number then there's no number
        if !self.grid[index].is_ascii_digit() {
            return None;
        }
        // Walk left and right from the index until we hit a non-number
        let walk_left = (0..index)
            .rev()
            // If we walk past the left edge stop
            .take_while(|i| !self.index_left_edge(*i + 1))
            .map(|i| (self.grid[i], i))
            .take_while(|(c, _)| c.is_ascii_digit())
            .collect_vec();

        // Reverse the walk left
        let walk_left = walk_left
            .into_iter()
            .rev()
            .map(|(c, i)| (c.to_digit(10).unwrap(), i))
            .collect_vec();

        // Walk right from the index until we hit a non-number
        let walk_right = (index..self.grid.len())
            // If we walk past the right edge stop
            .take_while(|i| !self.index_right_edge(*i - 1))
            .map(|i| (self.grid[i], i))
            .take_while(|(c, _)| c.is_ascii_digit())
            .map(|(c, i)| (c.to_digit(10).unwrap(), i))
            .collect_vec();

        // Combine the walks
        let combined = walk_left.into_iter().chain(walk_right).collect_vec();

        // If we didn't find any numbers then return None
        if combined.is_empty() {
            None
        } else {
            let start_index = combined.first().unwrap().1;
            let end_index = combined.last().unwrap().1;
            // Combine the digits into a number
            let number = combined
                .into_iter()
                .map(|(c, _)| c)
                .fold(0, |acc, n| acc * 10 + n);

            // Return the number and where it came from
            Some(GridNumber {
                number,
                start_index,
                end_index,
            })
        }
    }

    // Check if the index is on the edge of the grid so we don't treat the grid as wrapping
    fn index_left_edge(&self, index: usize) -> bool {
        index % self.width == 0
    }

    fn index_right_edge(&self, index: usize) -> bool {
        index % self.width == self.width - 1
    }

    fn get_adjacent_index(&self, index: usize) -> Vec<usize> {
        let width = self.width as isize;
        let offsets = if self.index_left_edge(index) {
            vec![
                width,      // Below
                -width,     // Above
                1,          // Right
                width + 1,  // Below Right
                -width + 1, // Above Right
            ]
        } else if self.index_right_edge(index) {
            vec![
                width,      // Below
                -width,     // Above
                -1,         // Left
                width - 1,  // Below Left
                -width - 1, // Above Left
            ]
        } else {
            vec![
                width,      // Below
                -width,     // Above
                -1,         // Left
                1,          // Right
                width + 1,  // Below Right
                width - 1,  // Below Left
                -width + 1, // Above Right
                -width - 1, // Above Left
            ]
        };

        offsets
            .into_iter()
            .map(|o| index as isize + o)
            .filter(|i| *i >= 0 && *i < self.len() as isize)
            .map(|i| i as usize)
            .collect_vec()
    }

    fn get_adjacent_numbers(&self, index: usize) -> Vec<GridNumber> {
        // If the value at the index isn't a symbol then there's no adjacent numbers to care about
        if index >= self.grid.len() {
            let val = self.grid[index];
            if val.is_ascii_digit() || val == '.' {
                return vec![];
            }
        }
        let adjacent_indices = self.get_adjacent_index(index);

        // Get the numbers adjacent to the symbols
        adjacent_indices
            .into_iter()
            .filter_map(|i| self.get_number(i))
            // Some numbers may be adjacent in multiple directions so dedupe them
            .unique()
            .collect()
    }
}

// A number and where we got it from in the grid so we can determine duplicates
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct GridNumber {
    number: u32,
    start_index: usize,
    end_index: usize,
}

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../input.txt");
    let grid = CharGrid::from_str(input);
    let numbers = grid.get_numbers();
    let sum = numbers.iter().map(|n| n.number).sum::<u32>();

    println!(
        "Sum: {} in {} micro seconds",
        sum,
        start.elapsed().as_micros()
    );
    let part_2_start = std::time::Instant::now();
    let gear_ratio = grid.get_gear_ratio();
    println!(
        "Gear Ratio: {} in {} micro seconds",
        gear_ratio,
        part_2_start.elapsed().as_micros()
    );
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    #[test]
    fn test() {
        let input = include_str!("../test_input.txt");

        let grid = super::CharGrid::from_str(input);
        assert!(grid.grid.first().is_some());
        assert_eq!(grid.grid.first().unwrap(), &'4');

        let eight_four = grid
            .get_adjacent_numbers(grid.width * 8 + 3)
            .into_iter()
            .unique()
            .collect_vec();
        println!("{:?}", eight_four);
        assert_eq!(eight_four.len(), 1);
        assert_eq!(eight_four[0].number, 664);

        let one_four = grid
            .get_adjacent_numbers(grid.width + 3)
            .into_iter()
            .unique()
            .collect_vec();
        println!("{:?}", one_four);
        assert_eq!(one_four.len(), 2);
        assert_eq!(one_four[1].number, 467);
        assert_eq!(one_four[0].number, 35);

        let sum = grid.get_numbers().iter().map(|n| n.number).sum::<u32>();
        assert_eq!(sum, 4361);

        let gear_ratio = grid.get_gear_ratio();
        assert_eq!(gear_ratio, 467835);
    }
}
