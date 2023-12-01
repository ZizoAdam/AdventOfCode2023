fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("input.txt");

    println!("Calibration sum: {}", get_calibration_sum(input));
    println!("Time: {}micro seconds", start.elapsed().as_micros());
}

pub fn get_calibration(input: &str) -> i32 {
    let first_num = input.chars().find(|c| c.is_ascii_digit()).unwrap();

    let last_num = input.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

    format!("{}{}", first_num, last_num).parse::<i32>().unwrap()
}

pub fn get_calibration_sum(input: &str) -> i32 {
    input.lines().map(get_calibration).sum()
}

mod tests {
    #[test]
    fn test() {
        let line_one = "1abc2";

        assert_eq!(super::get_calibration(line_one), 12);
    }

    #[test]
    fn test_all() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";

        assert_eq!(super::get_calibration_sum(input), 142);
    }
}
