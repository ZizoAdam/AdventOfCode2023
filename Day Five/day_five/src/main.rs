fn main() {
    let part_1_start = std::time::Instant::now();
    let min = part1::main();
    let part_1_end = part_1_start.elapsed();
    println!("Part 1: {} ({:?})", min, part_1_end);
    let part_2_start = std::time::Instant::now();
    let min = part2::main();
    let part_2_end = part_2_start.elapsed();
    println!("Part 2: {} ({:?})", min, part_2_end);
}

pub mod part1;
pub mod part2;
