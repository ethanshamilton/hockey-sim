use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
    C,
    RW,
    LW,
    LD,
    RD,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Handedness {
    L,
    R,
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
pub struct Player {
    pub id: usize,
    pub first_name: String,
    pub last_name: String,
    pub handedness: Handedness,
    pub primary_position: Position,
    pub secondary_position: Option<Position>,
    pub skills: Skills,
    pub current_stamina: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub name: String,
    pub players: Vec<Player>,
}
