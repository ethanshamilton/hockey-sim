use crate::models::Team;

pub struct GameState {
    pub time: u32,
    pub home: Team,
    pub away: Team,
}

impl GameState {
    pub fn new(home: Team, away: Team) -> Self {
        Self {
            time: 0,
            home,
            away,
        }
    }
}

pub fn simulate_gameplay(game: &mut GameState, ticks: u32) {
    for i in 0..ticks {
        game.time += 1;

        // placeholder game logic
        match game.time {
            1 => println!("Tick {}: Faceoff won by {} center", game.time, game.home.name),
            3 => println!("Tick {}: {} LW carries puck into zone", game.time, game.home.name),
            8 => println!("Tick {}: {} LW shoots the puck", game.time, game.home.name),
            9 => println!("Tick {}: He scores!", game.time),
            _ => println!("Tick {}: ...", game.time),
        }
    }
}
