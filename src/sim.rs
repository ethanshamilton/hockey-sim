use::std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::models::{Team, Puck, Rink, TeamSide, Player};

///
/// ENUMS
///

#[derive(Debug)]
pub enum GameEvent {
    FaceoffResult {
        winner_team: TeamSide,
        winner_player: String,
    },
    Pass {
        from_player: String,
        to_player: String,
    },
    Shot {
        shooter: String,
        result: ShotResult,
    },
    Goal {
        scorer: String,
        assist: Option<String>
    },
    LoosePuck {
        x: f32,
        y: f32,
    },
    PossessionChange {
        team: String,
        player: String,
    },
    TickLog(String),
}

#[derive(Debug)]
pub enum GamePhase {
    Faceoff,
    InPlay,
    GoalScored,
    Stoppage,
}

#[derive(Debug)]
pub enum ShotResult {
    Saved,
    Missed,
    Blocked,
    Goal,
}

///
/// STRUCTS
///

#[derive(Debug)]
pub struct GameState {
    pub time: u32,
    pub home: Team,
    pub away: Team,
    pub puck: Puck,
    pub rink: Rink,
    pub player_lookup: HashMap<String, Player>,
    pub players_on_ice: Vec<SimPlayer>,
    pub events: Vec<GameEvent>,
    pub phase: GamePhase,
}

impl GameState {
    pub fn new(home: Team, away: Team) -> Self {
        let mut player_lookup = HashMap::new();

        for player in &home.players {
            player_lookup.insert(player.player_id.clone(), player.clone());
        }

        for player in &away.players {
            player_lookup.insert(player.player_id.clone(), player.clone());
        }
        
        let mut game = Self {
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
            player_lookup,
            players_on_ice: vec![],
            events: vec![],
            phase: GamePhase::Faceoff,
        };
        
        populate_starting_lineups(&mut game);

        game
    }

    pub fn get_team_name(&self, side: TeamSide) -> &str {
        match side {
            TeamSide::Home => &self.home.name,
            TeamSide::Away => &self.away.name,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimPlayer {
    pub player_id: String,
    pub team_side: TeamSide,
    pub position: String,
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub target_x: f32,
    pub target_y: f32,
}

///
/// FUNCTIONS
///

pub fn simulate_tick(game: &mut GameState) {
    if game.time == 0 {
        handle_faceoff(game);
    } else {
        update_ai(game);
        update_positions(game);
        update_puck(game);
        check_shots(game);
    }

    game.time += 1;
}

pub fn dummy_simulate_gameplay(game: &mut GameState, ticks: u32) {
    for _ in 0..ticks {
        game.time += 1;

        match game.time {
            1 => handle_faceoff(game),
            _ => {}
        }

        for event in &game.events {
            if let Some(line) = render_event(event, game) {
                println!("Tick {}: {}", game.time, line)
            }
        }
        game.events.clear();
    }
}

fn populate_starting_lineups(game: &mut GameState) {
    for player in &game.home.players[..5] {
        game.players_on_ice.push(SimPlayer {
            player_id: player.player_id.clone(),
            team_side: TeamSide::Home,
            position: player.primary_position.to_string(),
            x: 100.0,
            y: 30.0,
            speed: 0.0,
            target_x: 100.0,
            target_y: 100.0,
        });
    }

    for player in &game.away.players[..5] {
        game.players_on_ice.push(SimPlayer {
            player_id: player.player_id.clone(),
            team_side: TeamSide::Away,
            position: player.primary_position.to_string(),
            x: 100.0,
            y: 30.0,
            speed: 0.0,
            target_x: 100.0,
            target_y: 100.0,
        });
    }
}

fn handle_faceoff(game: &mut GameState) {
    // find centers on ice
    let home_center_id = game.players_on_ice
        .iter()
        .find(|p| p.position == "C" && p.team_side == TeamSide::Home)
        .map(|p| p.player_id.clone())
        .unwrap();

    let away_center_id = game.players_on_ice
        .iter()
        .find(|p| p.position == "C" && p.team_side == TeamSide::Away)
        .map(|p| p.player_id.clone())
        .unwrap();

    // get player data for each center
    let home_player = &game.player_lookup[&home_center_id];
    let away_player = &game.player_lookup[&away_center_id];

    // assign faceoff scores
    let mut rng = rand::thread_rng();
    let home_score = home_player.skills.defense + rand::random::<f32>() * 0.2;
    let away_score = away_player.skills.defense + rand::random::<f32>() * 0.2;

    // determine winner
    let winner_team = if (home_score - away_score).abs() < 0.1 {
        None
    } else if home_score > away_score {
        Some((TeamSide::Home, home_center_id.clone()))
    } else {
        Some((TeamSide::Away, away_center_id.clone()))
    };

    // initiate play
    if let Some((team, center_id)) = winner_team {
        let dman = game.players_on_ice.iter()
            .find(|p| p.team_side == team && p.position.contains("D"))
            .unwrap();
        
        game.puck.possessed_by = Some(dman.player_id.clone());

        game.events.push(GameEvent::FaceoffResult { 
            winner_team: team, 
            winner_player: center_id,
        });
    } else {
        game.puck.possessed_by = None;
        game.puck.x = game.rink.center_ice.0;
        game.puck.y = game.rink.center_ice.1;

        game.events.push(GameEvent::LoosePuck { x: (game.puck.x), y: (game.puck.y) });
    }

    game.phase = GamePhase::InPlay;


}

fn update_ai(game: &mut GameState) {
    // for player in &mut game.players_on_ice {
    //     if Some(&player.id) == game.puck.possessed_by.as_ref() {
    //         // Skate toward offensive zone
    //         player.target_x += match player.team {
    //             TeamSide::Home => 2.0,
    //             TeamSide::Away => -2.0,
    //         };
    //     } else {
    //         // Stay in place for now
    //         player.target_x = player.x;
    //         player.target_y = player.y;
    //     }
    // }
}

fn update_positions(game: &mut GameState) {
    // for player in &mut game.players_on_ice {
    //     let dx = player.target_x - player.x;
    //     let dy = player.target_y - player.y;
    //     let dist = (dx * dx + dy * dy).sqrt();
    //     let max_step = 1.5;

    //     if dist > max_step {
    //         let norm_x = dx / dist;
    //         let norm_y = dy / dist;
    //         player.x += norm_x * max_step;
    //         player.y += norm_y * max_step;
    //     } else {
    //         player.x = player.target_x;
    //         player.y = player.target_y;
    //     }
    // }
}

fn update_puck(game: &mut GameState) {
    // if let Some(owner_id) = &game.puck.possessed_by {
    //     if let Some(player) = game.players_on_ice.iter().find(|p| &p.id == owner_id) {
    //         game.puck.x = player.x;
    //         game.puck.y = player.y;
    //     }
    // } else {
    //     // TODO: loose puck logic
    // }
}


fn check_shots(game: &mut GameState) {
    // if let Some(owner_id) = &game.puck.possessed_by {
    //     let player = game.players_on_ice.iter().find(|p| &p.id == owner_id).unwrap();

    //     let goal_line = match player.team {
    //         TeamSide::Home => game.rink.goal_line_away,
    //         TeamSide::Away => game.rink.goal_line_home,
    //     };

    //     let distance = (player.x - goal_line).abs();

    //     if distance < 10.0 {
    //         game.events.push(GameEvent::Shot {
    //             shooter_id: player.id.clone(),
    //             result: ShotResult::Goal,
    //         });
    //         game.events.push(GameEvent::Goal {
    //             scorer_id: player.id.clone(),
    //             assist_id: None,
    //         });
    //         game.phase = GamePhase::GoalScored;
    //     }
    // }
}

fn render_event(event: &GameEvent, game: &GameState) -> Option<String> {
    match event {
        GameEvent::FaceoffResult { winner_team, winner_player } => {
            let team_name = game.get_team_name(*winner_team);
            let player = &game.player_lookup[winner_player];
            Some(format!("Faceoff won by {} center, {}", team_name, player.player_id))
        }
        _ => None,
    }
}
