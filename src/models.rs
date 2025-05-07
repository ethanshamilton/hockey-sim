use std::fmt;
use serde::{Deserialize, Serialize};

///
/// ENUMS
/// 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    C,
    RW,
    LW,
    LD,
    RD,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Position::C => "C",
            Position::LW => "LW",
            Position::RW => "RW",
            Position::LD => "LD",
            Position::RD => "RD",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Handedness {
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamSide {
    Home,
    Away,
}

///
/// STRUCTS
/// 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub player_id: String,
    pub number: u8,
    pub first_name: String,
    pub last_name: String,
    pub handedness: Handedness,
    pub primary_position: Position,
    pub secondary_position: Option<Position>,
    pub skills: Skills,
    pub current_stamina: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Puck {
    pub x: f32,
    pub y: f32,
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub possessed_by: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rink {
    pub width: f32,
    pub length: f32,
    pub goal_line_left: f32,
    pub goal_line_right: f32,
    pub blue_line_left: f32,
    pub blue_line_right: f32,
    pub center_ice: (f32, f32),
    pub faceoff_spots: Vec<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub skating: f32,
    pub passing: f32,
    pub shooting: f32,
    pub defense: f32,
    pub stamina: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
}

///
/// IMPLS
/// 

impl Rink {
    pub fn standard() -> Self {
        let width = 85.0;
        let length = 200.0;

        let faceoff_spots = vec![
            // center ice
            (100.0, 42.5),

            // away zone
            (69.0, 22.0), // away right
            (69.0, 63.0), // away left

            // home zone
            (131.0, 22.0), // home left
            (131.0, 63.0), // home right
        ];

        Self {
            width,
            length,
            goal_line_left: 11.0,
            goal_line_right: 189.0,
            blue_line_left: 75.0,
            blue_line_right: 125.0,
            center_ice: (100.0, 42.5),
            faceoff_spots,
        }
    }

    pub fn is_in_bounds(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && x <= self.length && y >= 0.0 && y <= self.width
    }

}
