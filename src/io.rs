use crate::models::{Skills, Handedness, Position, Player, Team};
use serde::Deserialize;
use std::fs::File;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub struct PlayerCsvRow {
    pub number: u8,
    pub first_name: String,
    pub last_name: String,
    pub skating: f32,
    pub passing: f32,
    pub shooting: f32,
    pub defense: f32,
    pub stamina: f32,
    pub handedness: String,
    pub primary_position: String,
    pub secondary_position: Option<String>,
}

fn parse_position(pos: &str) -> Result<Position, String> {
    match pos {
        "C" => Ok(Position::C),
        "RW" => Ok(Position::RW),
        "LW" => Ok(Position::LW),
        "RD" => Ok(Position::RD),
        "LD" => Ok(Position::LD),
        other => Err(format!("Unknown position: {}", other)),
    }
}

fn parse_handedness(h: &str) -> Result<Handedness, String> {
    match h {
        "L" => Ok(Handedness::L),
        "R" => Ok(Handedness::R),
        other => Err(format!("Unknown handedness: {}", other)),
    }
}

// player ID is firstlast00, like alexovechkin08 -- this may fail to dedupe players
fn generate_player_id(first_name: &str, last_name: &str, number: u8) -> String {
    format!(
        "{}{}{:02}",
        first_name,
        last_name,
        number
    )
}

pub fn load_team_from_csv(path: &str, team_name: &str) -> Result<Team, Box<dyn Error>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut players = Vec::new();

    for (i, result) in rdr.deserialize().enumerate() {
        let row: PlayerCsvRow = result?;
        let player_id = generate_player_id(&row.first_name, &row.last_name, row.number);
        let handedness = parse_handedness(&row.handedness)?;
        let primary_pos = parse_position(&row.primary_position)?;
        let secondary_pos = match row.secondary_position {
            Some(ref pos) if !pos.trim().is_empty() => Some(parse_position(pos)?),
            _ => None,
        };

        players.push(Player {
            player_id: player_id,
            number: row.number,
            first_name: row.first_name,
            last_name: row.last_name,
            skills: Skills {
                skating: row.skating,
                passing: row.passing,
                shooting: row.shooting,
                defense: row.defense,
                stamina: row.stamina,
            },
            handedness,
            primary_position: primary_pos,
            secondary_position: secondary_pos,
            current_stamina: row.stamina,
        });
    }

    Ok(Team {
        name: team_name.to_string(),
        players,
    })
}
