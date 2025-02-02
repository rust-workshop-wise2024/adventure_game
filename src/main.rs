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
        let rand_actor_index = rand::random::<usize>() % players.len();
        let rand_target_index = (rand_actor_index + 1) % players.len();
        let rand_action_index = rand::random::<usize>() % 2;

        let player_name = players.keys().nth(rand_actor_index).unwrap().clone();
        let target_name = players.keys().nth(rand_target_index).unwrap().clone();

        let action: PlayerAction = if rand_action_index == 0 {
            PlayerAction::Attack
        } else {
            PlayerAction::Pass
        };

        handle_player_action(action, &player_name, &target_name, &mut players);

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
