use std::collections::HashMap;

// Definiere die möglichen Aktionen eines Spielers
enum PlayerAction {
    Attack { target: String },
    Pass,
}

// Struktur für Spieler
struct Player {
    name: String,
    health: i32,
    score: u32,
}

// Fügt einen neuen Spieler hinzu
fn add_player(name: &str, players: &mut HashMap<String, Player>) {
    if players.contains_key(name) {
        println!("Spieler {} existiert bereits!", name);
    } else {
        players.insert(
            name.to_string(),
            Player {
                name: name.to_string(),
                health: 3,
                score: 0,
            },
        );
        println!("Spieler {} wurde hinzugefügt.", name);
    }
}

// Verarbeitet die Angriff-Logik
fn attack_player(actor_name: &str, target_name: &str, players: &mut HashMap<String, Player>) {
    if let Some(target) = players.get_mut(target_name) {
        target.health -= 1;
        println!(
            "{} wurde angegriffen! Verlor ein Herz. Verbleibende Herzen: {}",
            target.name, target.health
        );

        if target.health <= 0 {
            println!("{} wurde eliminiert!", target.name);
            players.remove(target_name);

            if let Some(actor) = players.get_mut(actor_name) {
                actor.score += 1;
                println!("{} erhält einen Punkt. Neuer Punktestand: {}", actor.name, actor.score);
            }
        }
    } else {
        println!("Zielspieler {} existiert nicht.", target_name);
    }
}

// Verarbeitet, wenn ein Spieler seine Runde aussetzt
fn pass_turn(player_name: &str) {
    println!("{} hat diese Runde ausgesetzt.", player_name);
}

// Verarbeitet die Spieleraktion
fn handle_player_action(action: PlayerAction, actor_name: &str, players: &mut HashMap<String, Player>) {
    match action {
        PlayerAction::Attack { target } => attack_player(actor_name, &target, players),
        PlayerAction::Pass => pass_turn(actor_name),
    }
}

// Zeigt den aktuellen Punktestand an
fn display_scores(players: &HashMap<String, Player>) {
    println!("Punktestand:");
    for player in players.values() {
        println!(
            "{}: Herzen {} | Punktestand {}",
            player.name, player.health, player.score
        );
    }
}
fn main() {
    let mut players: HashMap<String, Player> = HashMap::new();

    // Spieler hinzufügen
    add_player("A", &mut players);
    add_player("B", &mut players);
    add_player("C", &mut players);

    // Hauptschleife des Spiels
    while players.len() > 1 {
        handle_player_action(
            PlayerAction::Attack {
                target: "B".to_string(),
            },
            "A",
            &mut players,
        );

        handle_player_action(
            PlayerAction::Attack {
                target: "C".to_string(),
            },
            "A",
            &mut players,
        );

        handle_player_action(PlayerAction::Pass, "A", &mut players);

        println!("--- Aktueller Status ---");
        display_scores(&players);
    }

    // Gewinner ausgeben
    if let Some(winner) = players.values().next() {
        println!(
            "Das Spiel ist vorbei! Gewinner: {} mit {} Punkten.",
            winner.name, winner.score
        );
    } else {
        println!("Kein Gewinner. Alle Spieler wurden eliminiert.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_player() {
        let mut players: HashMap<String, Player> = HashMap::new();

        // Test: Spieler hinzufügen
        add_player("Alice", &mut players);
        assert!(players.contains_key("Alice"));
        assert_eq!(players.get("Alice").unwrap().health, 3);
        assert_eq!(players.get("Alice").unwrap().score, 0);

        // Test: Spieler existiert bereits
        add_player("Alice", &mut players);
        assert_eq!(players.len(), 1); // Keine doppelten Einträge
    }

    #[test]
    fn test_attack_player() {
        let mut players: HashMap<String, Player> = HashMap::new();

        // Spieler hinzufügen
        add_player("A", &mut players);
        add_player("B", &mut players);

        // Test: Angriff
        attack_player("A", "B", &mut players);
        assert_eq!(players.get("B").unwrap().health, 2);

        // Test: Eliminieren eines Spielers
        attack_player("A", "B", &mut players);
        attack_player("A", "B", &mut players); // B verliert alle Herzen
        assert!(!players.contains_key("B"));
        assert_eq!(players.get("A").unwrap().score, 1);
    }

    #[test]
    fn test_handle_player_action() {
        let mut players: HashMap<String, Player> = HashMap::new();

        // Spieler hinzufügen
        add_player("A", &mut players);
        add_player("B", &mut players);

        // Test: Angriff
        handle_player_action(
            PlayerAction::Attack {
                target: "B".to_string(),
            },
            "A",
            &mut players,
        );
        assert_eq!(players.get("B").unwrap().health, 2);

        // Test: Pass
        handle_player_action(PlayerAction::Pass, "A", &mut players);
        assert_eq!(players.get("A").unwrap().health, 3); // Keine Änderung bei Pass
    }

    #[test]
    fn test_game_end() {
        let mut players: HashMap<String, Player> = HashMap::new();

        // Spieler hinzufügen
        add_player("A", &mut players);
        add_player("B", &mut players);

        // A eliminiert B
        attack_player("A", "B", &mut players);
        attack_player("A", "B", &mut players);
        attack_player("A", "B", &mut players);

        // Nur A bleibt übrig
        assert_eq!(players.len(), 1);
        assert!(players.contains_key("A"));
    }
}

