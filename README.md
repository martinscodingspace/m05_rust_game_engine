# m05_rust_game_engine

2.1 Game-Engine
In dieser Aufgabe entwickelst du ein zweidimensionales Spiel oder eine kleine Game-
Engine, auf deren Basis ein Spiel umgesetzt wird.

Ziel ist es, Spielzustände, Logik und Eingaben strukturiert zu verwalten und in
einer stabilen Architektur zusammenzuführen.

Die grafische Darstellung kann entweder im Terminal oder in einem Fenster erfolgen.

Du implementierst einen Game-Loop, der regelmäßig den Spielzustand aktualisiert,
Benutzereingaben verarbeitet und das aktuelle Spielfeld darstellt. Dabei modellierst
du unterschiedliche Spielzustände wie Start, Pause und Spielende und sorgst für saubere
Übergänge zwischen diesen Zuständen.

Zentrale Bestandteile sind außerdem die Kollisionserkennung sowie eine klare Trennung
wischen Spielmechanik, Darstellung und Eingabeverarbeitung.

Das Projekt soll so aufgebaut sein, dass Erweiterungen möglich sind und neue Spielelemente
ohne größere Umbauten integriert werden können.
• Level-System
• Highscore-Verwaltung
• Speicherung/Laden von Spielst¨anden
• Erweiterung des Schwierigkeitsgrades

Hinweis: Diese Aufgabe ist mittel bis schwer.
Folgende Crates konnten sich als hilfreich erweisen:
• pixels
• winit
