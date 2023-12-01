use phf::phf_map;

// Map of the words to their numbers
static NUMBERS: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

// Map of characters to numbers or an array of word lengths to check
static USEFUL_CHARS: phf::Map<char, UsefulChar> = phf_map! {
    '0' => UsefulChar::Digit(0),
    '1' => UsefulChar::Digit(1),
    '2' => UsefulChar::Digit(2),
    '3' => UsefulChar::Digit(3),
    '4' => UsefulChar::Digit(4),
    '5' => UsefulChar::Digit(5),
    '6' => UsefulChar::Digit(6),
    '7' => UsefulChar::Digit(7),
    '8' => UsefulChar::Digit(8),
    '9' => UsefulChar::Digit(9),
    'o' => UsefulChar::Letter(&[3]),
    't' => UsefulChar::Letter(&[3, 5]),
    'f' => UsefulChar::Letter(&[4]),
    's' => UsefulChar::Letter(&[3, 5]),
    'e' => UsefulChar::Letter(&[5]),
    'n' => UsefulChar::Letter(&[4]),
};

// Enum that describes the different valid states for a character worth checking
pub enum UsefulChar {
    Digit(u32),
    // Array of word lengths to check, I.E t could lead into two or three thus we check both 3 and 5 character lengths
    Letter(&'static [usize]),
}

fn main() {
    let input = include_str!("input.txt");
    answer(input);
}

fn answer(input: &str) {
    let start = std::time::Instant::now();
    let sum = input
        .lines()
        .map(|line| get_number(line).unwrap())
        .sum::<u32>();
    println!(
        "Result: {} in {} micro seconds",
        sum,
        start.elapsed().as_micros()
    );
}

fn get_number(line: &str) -> Option<u32> {
    let first_number = get_first_number(line);
    let last_number = get_last_number(line);
    if let (Some(first), Some(last)) = (first_number, last_number) {
        return Some(first * 10 + last);
    }
    None
}

fn get_first_number(line: &str) -> Option<u32> {
    for (index, char) in line.chars().enumerate() {
        if let Some(num) = check_char(index, char, line) {
            return Some(num);
        }
    }
    None
}

fn get_last_number(line: &str) -> Option<u32> {
    for (index, char) in line.chars().rev().enumerate() {
        let index = line.len() - index - 1;
        if let Some(num) = check_char(index, char, line) {
            return Some(num);
        }
    }
    None
}

fn check_char(index: usize, char: char, line: &str) -> Option<u32> {
    if let Some(useful) = USEFUL_CHARS.get(&char) {
        match useful {
            UsefulChar::Digit(digit) => {
                return Some(*digit);
            }
            UsefulChar::Letter(word_lengths) => {
                for word_length in word_lengths.iter() {
                    // Check if the word is too long
                    if index + word_length > line.len() {
                        continue;
                    }
                    // Get the word, if it has a matching number return it
                    let word = &line[index..index + word_length];
                    if let Some(number) = NUMBERS.get(word) {
                        return Some(*number);
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let line_one = "two1nine";
        let line_two = "eightwothree";
        let line_three = "abcone2threexyz";
        let line_four = "xtwone3four";
        let line_five = "4nineeightseven2";
        let line_six = "zoneight234";
        let line_seven = "7pqrstsixteen";

        assert_eq!(super::get_number(line_one), Some(29));
        assert_eq!(super::get_number(line_two), Some(83));
        assert_eq!(super::get_number(line_three), Some(13));
        assert_eq!(super::get_number(line_four), Some(24));
        assert_eq!(super::get_number(line_five), Some(42));
        assert_eq!(super::get_number(line_six), Some(14));
        assert_eq!(super::get_number(line_seven), Some(76));

        let sum = input
            .lines()
            .map(|line| {
                let number = super::get_number(line);
                number.unwrap()
            })
            .sum::<u32>();
        assert_eq!(sum, 281);
    }
}
