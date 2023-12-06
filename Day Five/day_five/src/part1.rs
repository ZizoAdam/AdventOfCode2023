pub struct SeedMapSection {
    pub from: String,
    pub to: String,
    pub map_values: Vec<(u32, u32, u32)>,
}

impl SeedMapSection {
    pub fn get_seed(&self, seed: u32) -> u32 {
        for (src, src_max, len) in &self.map_values {
            if src <= &seed && src_max > &seed {
                return len + (seed - src);
            }
        }
        seed
    }

    pub fn add(&mut self, dest: u32, src: u32, len: u32) {
        self.map_values.push((src, src + len, dest));
    }
}

pub fn main() -> u32 {
    let mut lines = include_str!("../input.txt").lines();

    let seeds = lines
        .next()
        .unwrap()
        .trim_start_matches("seeds: ")
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();

    let mut seed_maps = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.ends_with("map:") {
            let line = line.split_whitespace().next().unwrap();
            let split = line.split("-to-").collect::<Vec<&str>>();
            seed_maps.push(SeedMapSection {
                from: split[0].to_string(),
                to: split[1].to_string(),
                map_values: Vec::new(),
            });
        } else {
            let numbers = line
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<_>>();

            seed_maps
                .last_mut()
                .unwrap()
                .add(numbers[0], numbers[1], numbers[2])
        }
    }
    seeds
        .into_iter()
        .map(|seed| seed_maps.iter().fold(seed, |seed, map| map.get_seed(seed)))
        .min()
        .unwrap()
}
