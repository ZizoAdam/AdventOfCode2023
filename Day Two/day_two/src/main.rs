use day_two::GameHistory;

fn main() {
    let start = std::time::Instant::now();
    let game = include_str!("input_daytwo.txt");
    let game = GameHistory::new(game).unwrap();
    let sum = game.sum_game_ids_with_cubes(12, 13, 14);
    println!(
        "Part 1 - Sum of game ids: {} in {} micro seconds",
        sum,
        start.elapsed().as_micros()
    );

    let part_2_start = std::time::Instant::now();
    let min_power = game.sum_min_power();
    println!(
        "Part 2 - Sum of minimum power: {} in {} micro seconds",
        min_power,
        part_2_start.elapsed().as_micros()
    );

    println!("Total time: {} micro seconds", start.elapsed().as_micros());
}

#[cfg(test)]
mod test {

    #[test]
    fn test_draw() {
        let draw = " 3 blue, 4 red";
        let draw = day_two::Draw::new(draw).unwrap();
        assert_eq!(draw.red, 4);
        assert_eq!(draw.blue, 3);
    }

    #[test]
    fn test_game() {
        let game = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = day_two::Game::new(game).unwrap();
        assert_eq!(game.id, 1);

        let draws = game.draws;
        assert_eq!(draws.len(), 3);

        assert_eq!(draws[0].red, 4);
        assert_eq!(draws[0].blue, 3);

        assert_eq!(draws[1].red, 1);
        assert_eq!(draws[1].green, 2);
        assert_eq!(draws[1].blue, 6);

        assert_eq!(draws[2].green, 2);
    }

    #[test]
    fn test_game_history() {
        let log = include_str!("test_input.txt");
        let history = super::GameHistory::new(log).unwrap();
        assert_eq!(history.games.len(), 5);

        let game_1 = &history.games[0];
        assert_eq!(game_1.id, 1);
        assert_eq!(game_1.draws.len(), 3);
        assert_eq!(game_1.draws[0].red, 4);
        assert_eq!(game_1.draws[0].green, 0);
        assert_eq!(game_1.draws[0].blue, 3);
        assert_eq!(game_1.draws[1].red, 1);
        assert_eq!(game_1.draws[1].green, 2);
        assert_eq!(game_1.draws[1].blue, 6);
        assert_eq!(game_1.draws[2].red, 0);
        assert_eq!(game_1.draws[2].green, 2);
        assert_eq!(game_1.draws[2].blue, 0);

        assert!(game_1.has_enough_cubes(12, 13, 14));

        for game in history.games {
            let had_all = game.has_enough_cubes(12, 13, 14);
            if game.id == 1 || game.id == 2 || game.id == 5 {
                assert!(had_all)
            } else {
                assert!(!had_all)
            }
        }

        let history = super::GameHistory::new(log).unwrap();
        let sum = history.sum_game_ids_with_cubes(12, 13, 14);
        assert_eq!(sum, 8);
    }

    #[test]
    fn test_min_cubes() {
        let log = include_str!("test_input.txt");
        let history = super::GameHistory::new(log).unwrap();
        let game = &history.games[0];
        let min_cubes = game.minimum_cubes();
        assert_eq!(min_cubes.red, 4);
        assert_eq!(min_cubes.green, 2);
        assert_eq!(min_cubes.blue, 6);
        assert_eq!(game.min_power(), 48);

        let game = &history.games[1];
        let min_cubes = game.minimum_cubes();
        assert_eq!(min_cubes.red, 1);
        assert_eq!(min_cubes.green, 3);
        assert_eq!(min_cubes.blue, 4);
        assert_eq!(game.min_power(), 12);

        assert_eq!(history.sum_min_power(), 2286);
    }
}
