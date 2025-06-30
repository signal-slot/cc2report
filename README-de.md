# cc2report

*In anderen Sprachen lesen: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Ein intelligenter Arbeitsberichtsgenerator für Claude Code, der Konversationsprotokolle analysiert und umfassende Arbeitsberichte mit KI erstellt.

## Funktionen

- **KI-gestützte Analyse**: Verwendet OpenAIs GPT-Modelle zur Analyse von Konversationen und Generierung intelligenter Zusammenfassungen
- **Mehrsprachige Unterstützung**: Generiert Berichte in 19 Sprachen (automatisch aus den Systemeinstellungen erkannt)
- **Intelligentes Caching**: Speichert API-Antworten zwischen, um Kosten zu reduzieren und die Leistung zu verbessern (12-fache Beschleunigung bei gecachten Ausführungen)
- **Parallele Verarbeitung**: Verarbeitet mehrere Projekte gleichzeitig für schnellere Analysen
- **Flexible Datumsfilterung**: Generiert Berichte für bestimmte Daten, Wochen oder Monate
- **Fortschrittsanzeigen**: Visuelle Fortschrittsbalken für langlaufende Operationen
- **Token-Nutzungsverfolgung**: Überwacht API-Nutzung und Kosten
- **Template-Anpassung**: Anpassung von KI-Prompts und Berichtsformaten

## Installation

### Aus dem Quellcode

```bash
# Repository klonen
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# Erstellen und installieren
cargo build --release
cargo install --path .
```

### Voraussetzungen

- Rust 1.70 oder höher
- OpenAI API-Schlüssel

## Konfiguration

### API-Schlüssel einrichten

Setzen Sie Ihren OpenAI API-Schlüssel als Umgebungsvariable:

```bash
export OPENAI_API_KEY="Ihr-Schlüssel-hier"
```

Oder übergeben Sie ihn direkt über die Kommandozeile:

```bash
cc2report --api-key "Ihr-Schlüssel-hier"
```

### Standardpfade

- **Log-Verzeichnis**: `~/.claude/projects/` (Claude Codes Standard-Projektverzeichnis)
- **Cache-Verzeichnis**: `~/.cache/cc2report/`
- **Template-Datei**: `./cc2report.toml` oder `~/.config/cc2report/templates.toml`

## Verwendung

### Grundlegende Verwendung

Bericht für die heutigen Gespräche generieren (Standard):

```bash
cc2report
```

Bericht für alle Gespräche generieren (ohne Datumsfilter):

```bash
cc2report --all
```

### Datumsfilterung

```bash
# Bestimmtes Datum
cc2report --date 2024-07-01

# Datumsbereich
cc2report --from 2024-07-01 --to 2024-07-07

# Aktuelle Woche
cc2report --weekly

# Aktueller Monat
cc2report --monthly
```

### Ausgabeoptionen

```bash
# In Datei speichern
cc2report --output bericht.md

# JSON-Format
cc2report --format json --output bericht.json

# Sprache angeben (standardmäßig automatische Erkennung)
cc2report --lang ja  # Japanisch
cc2report --lang zh  # Chinesisch
cc2report --lang es  # Spanisch

# Beispiel: Heutiger Bericht auf Deutsch
cc2report --lang de

# Beispiel: Wochenbericht auf Spanisch
cc2report --weekly --lang es
```

### Leistungsoptionen

```bash
# Parallele Verarbeitung aktivieren (maximal 10)
cc2report --parallel 4

# Fortschrittsanzeigen deaktivieren
cc2report --quiet

# Token-Nutzung und Kosten anzeigen
cc2report --show-token-usage
```

### Cache-Verwaltung

```bash
# Cache leeren
cc2report --clear-cache

# Cache-Informationen anzeigen
cc2report --cache-info
```

### Template-Anpassung

Template-Datei generieren:

```bash
cc2report --generate-template mein-template.toml
```

Bearbeiten Sie das Template, um Prompts und Berichtsformate anzupassen.

## Unterstützte Sprachen

Das Tool erkennt automatisch Ihre Systemsprache und generiert entsprechende Berichte. Unterstützte Sprachen umfassen:

- Englisch (en)
- Japanisch (ja)
- Chinesisch (zh)
- Koreanisch (ko)
- Spanisch (es)
- Französisch (fr)
- Deutsch (de)
- Portugiesisch (pt)
- Russisch (ru)
- Italienisch (it)
- Niederländisch (nl)
- Polnisch (pl)
- Türkisch (tr)
- Arabisch (ar)
- Hindi (hi)
- Thailändisch (th)
- Vietnamesisch (vi)
- Indonesisch (id)
- Malaiisch (ms)

## Berichtsstruktur

Die generierten Berichte enthalten:

- **Projekttitel und -ziel**: Klare Zusammenfassung der bearbeiteten Aufgaben
- **Aktivitäten**: Kategorisierte Liste der durchgeführten Arbeiten
- **Gelieferter Wert**: Konkrete Errungenschaften und implementierte Funktionen
- **Technische Verbesserungen**: Code-Qualität und Leistungsverbesserungen
- **Ungelöste Probleme**: Blockaden oder ausstehende Aufgaben

## Kostenoptimierung

Das Tool enthält mehrere Funktionen zur Minimierung der API-Kosten:

1. **Caching**: Antworten werden 24 Stunden zwischengespeichert
2. **Modellauswahl**: Wählen Sie zwischen verschiedenen Modellen basierend auf dem Kosten-/Qualitätsverhältnis
3. **Token-Verfolgung**: Überwachen Sie die Nutzung, um im Budget zu bleiben

### Modellpreise (Stand 2024)

| Modell | Eingabekosten | Ausgabekosten | Empfehlung |
|--------|---------------|---------------|------------|
| gpt-4o (Standard) | $2.50/1M | $10.00/1M | Beste Qualität |
| gpt-4o-mini | $0.15/1M | $0.60/1M | Bestes Preis-Leistungs-Verhältnis |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | Budget-Option |

## Beispielbericht

```markdown
# Arbeitsbericht - 2024-07-01

## cc2report - Arbeitsberichtsgenerator

**Ziel**: Menschenlesbare Arbeitsberichte aus Claude Code-Protokollen generieren

**Aktivitäten**:
- Entwicklung - OpenAI API-Integration mit GPT-4 implementiert
- Feature-Ergänzung - Intelligente Analyse mit intelligenter Kategorisierung entwickelt
- UI-Verbesserung - Kommandozeilen-Schnittstelle verbessert

**Gelieferter Wert**:
- Kategorie "Sonstige Aufgaben" um 90% reduziert
- Berichtslesbarkeit erheblich verbessert
- Mehrsprachige Unterstützung erreicht (19 Sprachen)

**Technische Verbesserungen**:
- Leistung optimiert
- Fehlerbehandlung verbessert
```

## Fehlerbehebung

### Häufige Probleme

1. **"OpenAI API key is required"**
   - Stellen Sie sicher, dass `OPENAI_API_KEY` in Ihrer Umgebung gesetzt ist
   - Oder verwenden Sie die Option `--api-key`

2. **"Log directory does not exist"**
   - Geben Sie den korrekten Pfad mit `--log-dir` an
   - Standard ist `~/.claude/projects/`

3. **Ratenlimit-Fehler**
   - Reduzieren Sie parallele Anfragen: `--parallel 1`
   - Verwenden Sie ein niedrigeres Modell: `--model gpt-3.5-turbo`

## Entwicklung

### Aus dem Quellcode erstellen

```bash
# Entwicklungs-Build
cargo build

# Tests ausführen
cargo test

# Mit Debug-Ausgabe ausführen
RUST_LOG=debug cargo run
```

### Architektur

Das Projekt ist in folgende Module organisiert:

- `parser`: JSONL-Protokolldatei-Analyse
- `conversation_analyzer`: Extrahiert Themen und Kontext aus Konversationen
- `ai_analyzer`: OpenAI API-Integration
- `smart_analyzer`: Berichtsgenerierung
- `cache`: API-Antwort-Caching
- `templates`: Anpassbare Prompts und Formate
- `config`: Konfigurationsverwaltung
- `error`: Fehlerbehandlung
- `cli`: Kommandozeilen-Schnittstelle

## Beiträge

Beiträge sind willkommen! Bitte sehen Sie sich [CONTRIBUTING.md](CONTRIBUTING.md) für Richtlinien an.

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert - siehe die [LICENSE](LICENSE)-Datei für Details.

## Danksagungen

- Erstellt für [Claude Code](https://github.com/cline/cline) (ehemals Claude Engineer)
- Verwendet OpenAIs GPT-Modelle für intelligente Analysen
- Inspiriert durch den Bedarf an automatisierten Arbeitsberichten in der KI-unterstützten Entwicklung