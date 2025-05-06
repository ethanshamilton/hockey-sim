mod io;
mod sim;
mod models;

use io::load_team_from_csv;
use sim::{GameState, simulate_gameplay};

fn main() {
    let home = load_team_from_csv("data/caps.csv", "Capitals").unwrap();
    let away = load_team_from_csv("data/canes.csv", "Hurricanes").unwrap();

    println!("🏒 Game Start: {} at {}!", away.name, home.name);
    println!("");
    println!("--- {} Starting Lineups ---", home.name);
    println!("");

    for i in 0..5 {
        println!(
            "{} {} - {:?}",
            home.players[i].first_name,
            home.players[i].last_name,
            home.players[i].primary_position,
        );
    }

    println!("");
    println!("--- {} Starting Lineups ---", away.name);
    println!("");

    for i in 0..5 {
        println!(
            "{} {} - {:?}",
            away.players[i].first_name,
            away.players[i].last_name,
            away.players[i].primary_position,
        );
    }

    let mut game = GameState::new(home, away);
    println!("\nHere we go!");
    simulate_gameplay(&mut game, 10);
}
