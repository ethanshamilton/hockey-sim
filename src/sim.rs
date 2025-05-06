use crate::models::{Team, Puck, Rink, SimPlayer};

pub struct GameState {
    pub time: u32,
    pub home: Team,
    pub away: Team,
    pub puck: Puck,
    pub rink: Rink,
    pub players_on_ice: Vec<SimPlayer>,
    // pub events: vec![],
}

impl GameState {
    pub fn new(home: Team, away: Team) -> Self {
        Self {
            time: 0,
            home,
            away,
            puck: Puck {
                x: 100.0,
                y: 42.5,
                velocity_x: 0.0,
                velocity_y: 0.0,
                possessed_by: None,
            },
            rink: Rink::standard(),
            players_on_ice: vec![],
            // events: vec![],
        }
    }
}

pub fn simulate_gameplay(game: &mut GameState, ticks: u32) {
    for _i in 0..ticks {
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
