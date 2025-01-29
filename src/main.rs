use std::collections::HashMap;

enum PlayerAction {}

struct Player {}

fn add_player(name: &str, players: &mut HashMap<String, Player>) {}

fn attack_player(actor_name: &str, target_name: &str, players: &mut HashMap<String, Player>) {}

fn player_pass(actor_name: &str) {}

fn display_scores(players: &HashMap<String, Player>) {}

fn handle_player_action(
    action: PlayerAction,
    actor_name: &str,
    target_name: &str,
    players: &mut HashMap<String, Player>,
) {
}

fn main() {
    let mut players: HashMap<String, Player> = HashMap::new();

    add_player("A", &mut players);
    add_player("B", &mut players);
    add_player("C", &mut players);

    while players.len() > 1 {
        
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
        handle_player_action(
            PlayerAction::Attack,
            "A",
            "B",
            &mut players,
        );
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