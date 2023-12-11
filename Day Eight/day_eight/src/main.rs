use indexmap::IndexSet;
use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn from_char(c: char) -> Instruction {
        match c {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Invalid instruction"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn from_str(s: &str) -> Instructions {
        Instructions {
            instructions: s.chars().map(Instruction::from_char).collect(),
        }
    }
}

struct InstructionIterator<'a> {
    instructions: &'a Instructions,
    index: usize,
}

impl Iterator for InstructionIterator<'_> {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.instructions.instructions.len() {
            self.index += 1;
            Some(self.instructions.instructions[self.index])
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Instructions {
    type Item = Instruction;
    type IntoIter = InstructionIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        InstructionIterator {
            instructions: self,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CamelMap {
    locations: IndexSet<Box<str>>,
    map: Vec<Location>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Location {
    left: usize,
    right: usize,
}

impl GuidedMap {
    fn from_str(s: &str) -> GuidedMap {
        let mut lines = s.lines();
        let instructions = Instructions::from_str(lines.next().unwrap());

        let (locations, map) = lines.skip(1).fold(
            (
                IndexSet::<Box<str>>::new(),
                Vec::<(Box<str>, Box<str>)>::new(),
            ),
            |(mut locations, mut map), line| {
                let split = line.split(" = ").collect_vec();
                let (location, (left, right)) = (split[0].into(), split[1].split_at(4));
                map.push(((&left[1..4]).into(), (&right[2..5]).into()));
                locations.insert(location);
                (locations, map)
            },
        );

        let map = map
            .into_iter()
            .map(|(left, right)| Location {
                left: locations.get_index_of(&left).unwrap(),
                right: locations.get_index_of(&right).unwrap(),
            })
            .collect_vec();

        GuidedMap {
            instructions,
            camel_map: CamelMap { locations, map },
        }
    }
}
struct GuidedMap {
    instructions: Instructions,
    camel_map: CamelMap,
}

impl GuidedMap {
    fn route(&self, start: &str, end: &str) -> usize {
        let start_idx = self.camel_map.locations.get_index_of(start).unwrap();
        let end_idx = self.camel_map.locations.get_index_of(end).unwrap();
        self.steps_from(start_idx, end_idx, 0, 0)
    }

    fn steps_from(
        &self,
        start_idx: usize,
        end_idx: usize,
        steps: usize,
        cur_instruction: usize,
    ) -> usize {
        if start_idx == end_idx {
            return steps;
        }
        let instruction = self.instructions.instructions[cur_instruction];
        let next_idx = match instruction {
            Instruction::Left => self.camel_map.map[start_idx].left,
            Instruction::Right => self.camel_map.map[start_idx].right,
        };

        let next_instruction = if cur_instruction == self.instructions.instructions.len() - 1 {
            0
        } else {
            cur_instruction + 1
        };

        self.steps_from(next_idx, end_idx, steps + 1, next_instruction)
    }

    fn a_to_z(&self) -> usize {
        let start_indexes = self
            .camel_map
            .locations
            .iter()
            .enumerate()
            .filter(|(_, location)| location.ends_with('A'))
            .map(|(idx, _)| idx)
            .collect::<Vec<_>>();

        let steps_to_z = start_indexes
            .iter()
            .map(|start_idx| {
                let (steps, _, _) = self.steps_to_z_loc(*start_idx, 0, 0);
                steps
            })
            .collect::<Vec<_>>();

        // get lcm of all nums in steps_to_z
        let lcm = steps_to_z.iter().fold(1, |acc, &num| {
            let gcd = num / num::integer::gcd(acc, num);
            acc * gcd
        });

        lcm
    }

    fn steps_to_z_loc(
        &self,
        start_idx: usize,
        steps: usize,
        cur_instruction: usize,
    ) -> (usize, usize, usize) {
        if self.camel_map.locations[start_idx].ends_with('Z') {
            return (steps, cur_instruction, start_idx);
        }

        let instruction = self.instructions.instructions[cur_instruction];
        let next_idx = match instruction {
            Instruction::Left => self.camel_map.map[start_idx].left,
            Instruction::Right => self.camel_map.map[start_idx].right,
        };

        let next_instruction = if cur_instruction == self.instructions.instructions.len() - 1 {
            0
        } else {
            cur_instruction + 1
        };

        self.steps_to_z_loc(next_idx, steps + 1, next_instruction)
    }
}

fn main() {
    let start = std::time::Instant::now();
    let guided_map = GuidedMap::from_str(include_str!("../input.txt"));
    let steps = guided_map.route("AAA", "ZZZ");
    let elapsed = start.elapsed();
    println!("Part 1: {steps} ({elapsed:?})");
    let start = std::time::Instant::now();
    let steps = guided_map.a_to_z();
    let elapsed = start.elapsed();
    println!("Part 2: {steps} ({elapsed:?})");
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        let guided_map = super::GuidedMap::from_str(include_str!("../test_input.txt"));
        assert_eq!(guided_map.route("AAA", "ZZZ"), 2);
    }
}
