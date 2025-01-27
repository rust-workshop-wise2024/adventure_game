Rust Spieler- und Angriffssystem

Beschreibung des Spiels
In diesem Spiel entwickelst du ein einfaches, konsolenbasiertes Spieler- und Angriffssystem in Rust. Spieler können hinzugefügt werden, sich gegenseitig angreifen oder ihre Runde aussetzen. Ziel ist es, bis zum Schluss zu überleben, während die anderen Spieler eliminiert werden.
Eigenschaften der Spieler:
Jeder Spieler hat einen Namen (String).
Spieler starten mit 3 Herzen (Gesundheit).
Jeder Spieler hat eine Punktzahl (Score), die zu Beginn bei 0 liegt.
Spielregeln:
Ein Spieler kann einen anderen angreifen. Dabei verliert der angegriffene Spieler 1 Herz.
Wenn die Herzen eines Spielers auf 0 fallen, wird dieser Spieler eliminiert, und der Angreifer erhält 1 Punkt.
Spieler können ihre Runde aussetzen.
Das Spiel endet, wenn nur noch ein Spieler übrig bleibt, der dann als Gewinner erklärt wird.

Aufgabe 1: Spieler-Datenstruktur
Erstelle eine Datenstruktur, die einen Spieler repräsentiert. Ein Spieler hat:
name: Der Name des Spielers (String).
health: Die Anzahl der Herzen des Spielers (i32), initialisiert mit 3.
score: Die Punkte des Spielers (u32), initialisiert mit 0.

Aufgabe 2: Spieler hinzufügen
Schreibe eine Funktion, die einen neuen Spieler zur Liste der Spieler (HashMap) hinzufügt.
Methode:

fn add_player(name: &str, players: &mut HashMap<String, Player>)

Anforderungen:
Wenn ein Spieler mit dem angegebenen Namen bereits existiert, soll eine Nachricht ausgegeben werden: „Spieler [Name] existiert bereits!“
Andernfalls soll ein neuer Spieler mit 3 Herzen und 0 Punkten hinzugefügt werden.

Aufgabe 3: Spieleraktionen definieren
Definiere die möglichen Aktionen, die ein Spieler ausführen kann:
Angriff: Der Spieler greift einen anderen Spieler an.
Pass: Der Spieler setzt seine Runde aus.
Anforderungen:
Beschreibe die Aktionen so, dass sie alle notwendigen Informationen enthalten.
Zum Beispiel sollte ein Angriff die Zielinformation enthalten (z. B. den Namen des angegriffenen Spielers).


Aufgabe 4: Angriffsfunktion implementieren
Schreibe eine Funktion, die die Logik eines Angriffs verarbeitet.
Methode:

fn attack_player(actor_name: &str, target_name: &str, players: &mut HashMap<String, Player>)

Anforderungen:
Der Zielspieler verliert 1 Herz.
Wenn die Herzen des Zielspielers auf 0 fallen:
Entferne ihn aus der HashMap.
Gib eine Nachricht aus: „[Name] wurde eliminiert!“.
Der Angreifer erhält 1 Punkt.
Wenn der Zielspieler nicht existiert, gib eine Nachricht aus: „Zielspieler [Name] existiert nicht.“

Aufgabe 5: Spieleraktionen verarbeiten
Schreibe eine Funktion, die eine Spieleraktion verarbeitet.
Methode:

fn handle_player_action(action: PlayerAction, actor_name: &str, players: &mut HashMap<String, Player>)

Anforderungen:
Angriff: Rufe die Funktion attack_player auf, um die Angriffslogik auszuführen.
Pass: Gib eine Nachricht aus: „[Name] hat diese Runde ausgesetzt.“

Aufgabe 6: Hauptschleife
Implementiere die Hauptschleife, die das Spiel steuert.
Methode:

fn main()

Anforderungen:
Erstelle eine HashMap zum Speichern der Spieler.
Füge mindestens 3 Spieler hinzu (z. B. "A", "B", "C").
Solange mehr als ein Spieler in der HashMap ist:
Verarbeite Aktionen wie Angriffe oder Passaktionen.
Zeige den aktuellen Status aller Spieler an.
Sobald nur noch ein Spieler übrig bleibt, gib diesen als Gewinner aus.

Optionale Aufgabe 7: Punktestand anzeigen
Schreibe eine Funktion, die den aktuellen Punktestand aller Spieler ausgibt.
Methode:

fn display_scores(players: &HashMap<String, Player>)

Anforderungen:
Gib den Namen, die verbleibenden Herzen und die Punktzahl jedes Spielers aus.
