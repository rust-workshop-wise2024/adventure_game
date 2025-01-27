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

        // handle_player_action(action, &player_name, &target_name, &mut players);

        // display_scores(&players);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_player() {}

    #[test]
    fn test_handle_player_action() {}

    #[test]
    fn test_game_end() {}
}
