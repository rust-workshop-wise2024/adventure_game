use std::collections::HashMap;

enum PlayerAction {
    Attack,
    Pass,
}

struct Player {
    name: String,
    health: i32,
    score: i32,
}

fn add_player(name: &str, players: &mut HashMap<String, Player>) {
    if players.contains_key(name) {
        println!("Spieler ${:} existiert bereits!", name);
    } else {
        players.insert(
            name.to_string(),
            Player {
                name: name.to_string(),
                health: 3,
                score: 0,
            },
        );
    }
}

fn attack_player(actor_name: &str, target_name: &str, players: &mut HashMap<String, Player>) {
    players.get_mut(target_name).unwrap().health -= 1;

    println!(
        "Spieler ${:} hat Spieler ${:} angegriffen!",
        actor_name, target_name
    );

    if players.get(target_name).unwrap().health <= 0 {
        players.get_mut(actor_name).unwrap().score += 1;
        println!("Spieler ${:} wurde eliminiert!", target_name);
        players.remove(target_name);
    }
}

fn player_pass(actor_name: &str) {
    println!("Spieler ${:} hat gepasst!", actor_name);
}

fn display_scores(players: &HashMap<String, Player>) {
    println!("Punktestand:");
    for player in players.values() {
        println!(
            "{}: Herzen {} | Punktestand {}",
            player.name, player.health, player.score
        );
    }
}

fn handle_player_action(
    action: PlayerAction,
    actor_name: &str,
    target_name: &str,
    players: &mut HashMap<String, Player>,
) {
    match action {
        PlayerAction::Attack => {
            attack_player(actor_name, target_name, players);
        }
        PlayerAction::Pass => {
            player_pass(actor_name);
        }
    };
}

fn main() {
    let mut players: HashMap<String, Player> = HashMap::new();

    add_player("A", &mut players);
    add_player("B", &mut players);
    add_player("C", &mut players);

    while players.len() > 1 {
        handle_player_action(PlayerAction::Attack, "A", "B", &mut players);

        handle_player_action(PlayerAction::Attack, "A", "C", &mut players);

        handle_player_action(PlayerAction::Pass, "A", "A", &mut players);

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
    fn test_handle_player_action() {
        let mut players: HashMap<String, Player> = HashMap::new();

        // Spieler hinzufügen
        add_player("A", &mut players);
        add_player("B", &mut players);

        // Test: Angriff
        handle_player_action(PlayerAction::Attack, "A", "B", &mut players);
        assert_eq!(players.get("B").unwrap().health, 2);

        // Test: Pass
        handle_player_action(PlayerAction::Pass, "A", "", &mut players);
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
