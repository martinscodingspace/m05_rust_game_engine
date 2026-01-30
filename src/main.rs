/*****************************************************************************
***  IHK Rust Developer 2025/2026                                          ***
***  m05_rust_game_engine                                                  ***
***  Topic : Game-Engine für Spiel mit graphischer Darstellung             ***
***  Fälligkeit 01.02.2026 23:59                                           ***
***  written by the author Martin Hildebrand                               ***
***  Projekt-Postfach  :   m05_abschlussprojekt@turbofisch.de              ***
***  2026 ©  ALL RIGHTS RESERVED                                           ***
******************************************************************************
***  Das Repository befindet sich hier:                                    ***
***  https://github.com/martinscodingspace/m05_rust_game_engine            ***
******************************************************************************
***  Diese Software steht unter folgender LIZENZ                           ***
***  GNU General Public License 3                                          ***
***  http://www.gnu.org/licenses/gpl-3.0.de.html                           ***
*****************************************************************************/
//! 2.1 Game-Engine
//! In dieser Aufgabe entwickelst du ein zweidimensionales Spiel oder eine kleine Game-
//! Engine, auf deren Basis ein Spiel umgesetzt wird.
//!
//! Ziel ist es, Spielzustände, Logik und Eingaben strukturiert zu verwalten und in
//! einer stabilen Architektur zusammenzuführen.
//!
//! Die grafische Darstellung kann entweder im Terminal oder in einem Fenster erfolgen.
//!
//! Du implementierst einen Game-Loop, der regelmäßig den Spielzustand aktualisiert,
//! Benutzereingaben verarbeitet und das aktuelle Spielfeld darstellt. Dabei modellierst
//! du unterschiedliche Spielzustände wie Start, Pause und Spielende und sorgst für saubere
//! Übergänge zwischen diesen Zuständen.
//!
//! Zentrale Bestandteile sind außerdem die Kollisionserkennung sowie eine klare Trennung
//! zwischen Spielmechanik, Darstellung und Eingabeverarbeitung.
//!
//! Das Projekt soll so aufgebaut sein, dass Erweiterungen möglich sind und neue Spielelemente
//! ohne größere Umbauten integriert werden können.
//! • Level-System
//! • Highscore-Verwaltung
//! • Speicherung/Laden von Spielst¨anden
//! • Erweiterung des Schwierigkeitsgrades
//!
//! Hinweis: Diese Aufgabe ist mittel bis schwer.
//! Folgende Crates konnten sich als hilfreich erweisen:
//! • pixels
//! • winit
//! 2.1 Game-Engine
//! In dieser Aufgabe entwickelst du ein zweidimensionales Spiel oder eine kleine Game-
//! Engine, auf deren Basis ein Spiel umgesetzt wird.
//!
//! Ziel ist es, Spielzustände, Logik und Eingaben strukturiert zu verwalten und in
//! einer stabilen Architektur zusammenzuführen.
//!
//! Die grafische Darstellung kann entweder im Terminal oder in einem Fenster erfolgen.
//!
//! Du implementierst einen Game-Loop, der regelmäßig den Spielzustand aktualisiert,
//! Benutzereingaben verarbeitet und das aktuelle Spielfeld darstellt. Dabei modellierst
//! du unterschiedliche Spielzustände wie Start, Pause und Spielende und sorgst für saubere
//! Übergänge zwischen diesen Zuständen.
//!
//! Zentrale Bestandteile sind außerdem die Kollisionserkennung sowie eine klare Trennung
//! zwischen Spielmechanik, Darstellung und Eingabeverarbeitung.
//!
//! Das Projekt soll so aufgebaut sein, dass Erweiterungen möglich sind und neue Spielelemente
//! ohne größere Umbauten integriert werden können.
//! • Level-System
//! • Highscore-Verwaltung
//! • Speicherung/Laden von Spielst¨anden
//! • Erweiterung des Schwierigkeitsgrades
//!
//! Hinweis: Diese Aufgabe ist mittel bis schwer.

// Import der am häufigsten genutzen Funktionen vom Crate Macroquad
use macroquad::prelude::*;
// Import der doppelt verkettete Liste für effizientes Einfügen und Entfernen am Anfang und am Ende
use std::collections::LinkedList;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Größe des Spielfeldes festlegen
const QUADRAT: i16 = 12;

// Koordinate
type Feld = (i16, i16);

// Das Herzstück : Die Schlange mit Kopf, Körper (LinkedList) und Bewegungsrichtung
struct Schlange {
    kopf: Feld,
    rumpf: LinkedList<Feld>,
    richtung: Feld,
}

// Für den Head-Liner
#[macroquad::main("M05 GAME-ENGINE")]

// Asynchrone Funktionalität für gleichzeites Starten der Anwendung und Warten auf Ergebnisse,
// um Programm-Ausführung effizienter zu gestalten
// Anstatt direkten Rückkabewert liefert ASYNC eine "Repräsentation" des zukünftiten Wertes,
// einen sogenannten "Future" (eine Art Versprechen auf einen Wert)
async fn main() {
    let mut name = String::new();
    let mut eingabe = false;
    let datei = "highscorer_db.json";

    // Datenbank laden oder neu anlegen
    let mut db: HashMap<String, u32> = if Path::new(datei).exists() {
        let inhalt = fs::read_to_string(datei).expect("Datei konnte nicht gelesen werden");
        serde_json::from_str(&inhalt).unwrap_or_default()
    } else {
        HashMap::new()
    };

    // Das Start-Menü mit Benutzereingabe vor Spielbeginn
    loop {
        clear_background(LIGHTGRAY);

        if !eingabe {
            draw_text(format!("Willkommen beim Spiel Snaca").as_str(), 40., 50., 30., BLACK);
            draw_text("Bitte gib deinen Namen ein:", 40.0, 180.0, 30.0, BLACK);
            draw_rectangle_lines(40.0, 200.0, 400.0, 40.0, 2.0, BLACK);
            draw_text(&name, 50.0, 230.0, 30.0, DARKGRAY);

            draw_text(format!("Alternativ   [ESCAPE] für Anonym").as_str(), 40., 340., 30., BLACK);
            draw_text(format!("Nach Eingabe [ENTER]  für Start").as_str(), 40., 400., 30., BLACK);


            // Buchstabe für Buchstabe in die Zeichekette (Name) mit aufnehmen
            // Tastatureingaben abfangen und auf Zeichenlöschen reagieren
            while let Some(c) = get_char_pressed() {
                if c != '\n' && c != '\r' && c!= ' ' {
                    name.push(c);
                }
                if is_key_pressed(KeyCode::Backspace) {
                    name.pop();
                }
                break;
            }
            // Vereinfachung zur Fehlervermeidung
            name = name.to_uppercase();

            // Backspace
            if is_key_pressed(KeyCode::Backspace) {
                name.pop();
            }

            // Enter bestätigt Eingabe und Schleife kann verlassen werden
            if is_key_pressed(KeyCode::Enter) && !name.is_empty() {
                eingabe = true;
            }
            // Für den Datenschutz-sensiblen Mitspieler ;-)
            if is_key_pressed(KeyCode::Escape) {
                name = "ANONYMOUS".to_string();
                eingabe = true;
            }
            // Beispiel: Einzelner reservierter Buchstabe kürzt die Eingabe schon ab
            // Kaa ist die Schlange aus dem Dschungelbuch ;-)
            if is_key_pressed(KeyCode::X) {
                name = "Kaa".to_string();
                eingabe = true;
            }
        } else {
            draw_text(
                &format!("Hallo, {}!", name),
                40.0,
                120.0,
                40.0,
                DARKGREEN,
            );            draw_text("Viel Erfolg !!! Los geht's mit [ENTER]", 40.0, 170.0, 20.0, GRAY);

            if is_key_pressed(KeyCode::Enter) {
                break;
            }
        }
        next_frame().await;
    }


    // Der Ausgangspunkt : die Schlange startet links oben in Richtung rechts nur mit dem Kopf (leerer Körper)
    let mut schlange = Schlange {
        kopf: (0, 0),
        richtung: (1, 0),
        rumpf: LinkedList::new(),
    };
    // Das Futter der Schlange : Zufälliges Feld innerhalb der Spielfläche
    let mut futter: Feld = (rand::gen_range(0, QUADRAT), rand::gen_range(0, QUADRAT));
    // Beginnend mit 0 Punkten
    let mut score = 0;
    // Beginnend mit einer kleinen Geschwindigkeit
    let mut geschw = 0.3;
    // Aktuellen Zeitwert nehmen
    let mut zeitnahme = get_time();
    // Noch keine Taste getätigt
    let mut taste_aktiv = false;
    // Setzen der Abbruch-Variable
    let mut spiel_ende = false;
    // Schierogkeits-Stärke
    let mut level = 1;
    // Name des Spielers
    // let name =  "Mustermann";

    // Die 4 Richtungen und ihre Auswirkungen auf das Koordinatenfeld
    let oben = (0, -1);
    let unten = (0, 1);
    let rechts = (1, 0);
    let links = (-1, 0);

    // Die GAME-Loop, gültig bis zum ESCAPE oder Anklicken "X" rechts oben zum Schließen des Fensters
    loop {
        // Bedingung: solange das Spiel nicht vorbei ist => zunächst umgekehrtes FALSE
        if !spiel_ende {


            // Die 4 möglichen Richtungen als aktiven Navigations-Befehl
            if is_key_down(KeyCode::Right) && schlange.richtung != links && !taste_aktiv {
                schlange.richtung = rechts;
                taste_aktiv = true;
            } else if is_key_down(KeyCode::Left) && schlange.richtung != rechts && !taste_aktiv {
                schlange.richtung = links;
                taste_aktiv = true;
            } else if is_key_down(KeyCode::Up) && schlange.richtung != unten && !taste_aktiv {
                schlange.richtung = oben;
                taste_aktiv = true;
            } else if is_key_down(KeyCode::Down) && schlange.richtung != oben && !taste_aktiv {
                schlange.richtung = unten;
                taste_aktiv = true;
            }

            // Wenn neue Zwischenzeit-Messung größer als die Mindest-Geschwindigkeit
            if get_time() - zeitnahme > geschw {


                zeitnahme = get_time();
                // Link-List-Element vorne hinzufügen, weil der Kopf um 1 Stelle verschoben wird
                schlange.rumpf.push_front(schlange.kopf);
                // Kopf der Schlange mit neuen Koordinaten um 1 Stelle verschoben
                schlange.kopf = (schlange.kopf.0 + schlange.richtung.0, schlange.kopf.1 + schlange.richtung.1);
                // Falls das Futter-Feld erreicht wird, gibt es Punkte und erhöhte Geschwindigkeit
                if schlange.kopf == futter {
                    futter = (rand::gen_range(0, QUADRAT), rand::gen_range(0, QUADRAT));
                    if score % 1000 >= 900 {
                        geschw *= 0.9;
                        level = level + 1;
                    }
                    score += 100;

                } else {
                    // Durch das Weiter-Schlängeln muss das hinteste Element wieder gekürzt werden
                    schlange.rumpf.pop_back();
                }
                // Die Abbruchbedingung erfolgt, wenn der Rand berührt wird
                if schlange.kopf.0 < 0
                    || schlange.kopf.1 < 0
                    || schlange.kopf.0 >= QUADRAT
                    || schlange.kopf.1 >= QUADRAT
                {
                    spiel_ende = true;
                }
                // Ebenfalls Abbruchbedingung, wenn der Kopf der Schlange auf den eigenen Körper trifft
                for (x, y) in &schlange.rumpf {
                    if *x == schlange.kopf.0 && *y == schlange.kopf.1 {
                        spiel_ende = true;
                    }
                }
                // Vor dem nächsten Zeitabgleich muss der Navigations-Befehl wieder zurückgesetzt werden
                taste_aktiv = false;
            }
        }
        if !spiel_ende {
            // Noch im Spiel => Setzen der angepassten Koordinaten für den nächsten LOOP
            clear_background(SKYBLUE);

            // screen_width().min() liefert einen Wert (z.B. u32), und .min() wird auf diesem
            // Wert oder einer Sammlung von Werten verwendet, um den kleinsten Wert zu finden

            // screen_width() liefert die absolute Breite des Bildschirms.

            // game_size präsentiert die Breite (und gewöhnlich Höhe) des zentrierten Objekts
            // let game_size = screen_width().min(screen_height());
            let game_size = screen_height() / 1.1;

            // screen_width() - game_size berechnet die absoluten Leerstellen links
            // /  2: Leerstellen durch 2 teilen für perfekte Zentrierung
            // + 10: Versatz um 10 Pixel und verschiebt das zentrierte Objekt nach rechts
            let x_setzen = (screen_width() - game_size)  / 2.0  + 10.;
            let y_setzen = (screen_height() - game_size) / 2.0  + 10.;

            // speichert die berechnete Seitenlänge für jedes Quadrat
            let quadrat_gr = (screen_height() - y_setzen * 2.) / QUADRAT as f32;

            // Zeichnen des Vierecks mit den vier Koordinaten und der Farbe
            draw_rectangle(x_setzen, y_setzen, game_size - 20., game_size - 20., LIGHTGRAY);

            // Waagerechte Linien für ein Gittermuster mit Koordinaten, Strichstärke und Farbe
            for i in 1..QUADRAT {
                draw_line(
                    x_setzen,
                    y_setzen + quadrat_gr * i as f32,
                    screen_width() - x_setzen,
                    y_setzen + quadrat_gr * i as f32,
                    2.,
                    SKYBLUE,
                );
            }

            // Senkrechte Linien für ein Gittermuster mit Koordinaten, Strichstärke und Farbe
            for i in 1..QUADRAT {
                draw_line(
                    x_setzen + quadrat_gr * i as f32,
                    y_setzen,
                    x_setzen + quadrat_gr * i as f32,
                    screen_height() - y_setzen,
                    2.,
                    SKYBLUE,
                );
            }

            // Der aktuelle Kopf der Schlange
            draw_rectangle(
                x_setzen + schlange.kopf.0 as f32 * quadrat_gr,
                y_setzen + schlange.kopf.1 as f32 * quadrat_gr,
                quadrat_gr,
                quadrat_gr,
                DARKPURPLE,
            );

            // der länger werdende Rumpf der Schlange
            for (x, y) in &schlange.rumpf {
                draw_rectangle(
                    x_setzen + *x as f32 * quadrat_gr,
                    y_setzen + *y as f32 * quadrat_gr,
                    quadrat_gr,
                    quadrat_gr,
                    PURPLE,
                );
            }

            // Das Futter-Feld
            draw_rectangle(
                x_setzen + futter.0 as f32 * quadrat_gr,
                y_setzen + futter.1 as f32 * quadrat_gr,
                quadrat_gr,
                quadrat_gr,
                MAROON,
            );

            draw_text(format!("NAME : {name }").as_str(), 10., 20., 20., BLACK);
            draw_text(format!("LEVEL: {level}").as_str(), 10., 50., 20., BLACK);
            draw_text(format!("SCORE: {score}").as_str(), 10., 80., 20., BLACK);
            draw_text(format!("HIGHSCORE:").as_str(), 10., 240., 20., BLACK);
            // Highscore-Anzeige während des Spiels
            // Es gibt immer nur den einen Eintrag
            if let Some((key, value)) = db.iter().next() {
                draw_text(format!("{}", key).as_str(), 10., 280., 20., BLACK);
                draw_text(format!("Punkte : {}", value).as_str(), 10., 320., 20., BLACK);
                // draw_text(format!("Level    :   {}", level).as_str(), 10., 360., 15., BLACK);
            }

            // Pausen-Routine
            if is_key_pressed(KeyCode::Space) {
                let mut is_paused = true;
                while is_paused == true {
                    clear_background(DARKGREEN);
                    let text = "Spiel angehalten! Weiter mit <ENTER> oder Ende mit <ESC>";
                    let font_size = 30.;
                    let text_size = measure_text(text, None, font_size as _, 1.0);

                    draw_text(
                        text,
                        screen_width() / 2. - text_size.width / 2.,
                        screen_height() / 2. + text_size.height / 2.,
                        font_size,
                        GREEN,
                    );
                    next_frame().await;
                    if is_key_pressed(KeyCode::Enter) {
                        is_paused = false;
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        is_paused = false;
                        spiel_ende = true;
                    }
                }
            }


        } else {
            // Abbruchvariable TRUE mit Auswahlfrage an User
            clear_background(LIGHTGRAY);


            // Darstellung im Ergebnis-Fenster mit Koordinaten, Schriftgröße und Farbe
            draw_text(format!("GAME OVER! Presse [ENTER] to play again or [ESCAPE]").as_str(), 10.0, 40., 30.0, BLACK);

            draw_text(format!("SCORER: {name}                SCORE: {score}").as_str(), 10., 120., 30., BLACK);
            if let Some((_alter_key, alter_value)) = db.iter().next() {
                if score > *alter_value {
                    // Alten Eintrag entfernen
                    db.clear();

                    // Neuen Eintrag setzen
                    db.insert(name.clone(), score);
                    // Neuen Wert direkt abspeichern
                    let json = serde_json::to_string_pretty(&db).expect("Serialisierung fehlgeschlagen");
                    fs::write(datei, json).expect("Datei konnte nicht geschrieben werden");
                }
            }

            draw_text(format!("HIGHSCORE").as_str(), 10., 240., 50., DARKPURPLE);

            // Ausgabe des Highscore-Wertes
            if let Some((key, value)) = db.iter().next() {
                draw_text(format!("Bester  :   {}", key).as_str(), 10., 290., 30., BLACK);
                draw_text(format!("Punkte  :   {}", value).as_str(), 10., 330., 30., BLACK);
            }

            // Reguläres ENDE ohne das Fenster selber schließen zu müssen
            if is_key_down(KeyCode::Escape) {
                break;
            }

            // Erneuter Spielbeginn wird vorbereitet
            if is_key_down(KeyCode::Enter) {
                schlange = Schlange {
                    kopf: (0, 0),
                    richtung: (1, 0),
                    rumpf: LinkedList::new(),
                };
                futter = (rand::gen_range(0, QUADRAT), rand::gen_range(0, QUADRAT));
                // score = 0;
                geschw = 0.3;
                zeitnahme = get_time();
                spiel_ende = false;
                score = 0;
                level = 1;
            }
        }
        // Warten im Rahmen der asynchronen Funktionalität
        next_frame().await;
    }
    println!("\nProgramm-ENDE  Bis zum nächsten Mal\n");
}

