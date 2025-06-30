# cc2report

*Czytaj w innych językach: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Inteligentny generator raportów pracy dla Claude Code, który analizuje dzienniki konwersacji i tworzy kompleksowe raporty pracy przy użyciu AI.

## Funkcje

- 🔍 **Inteligentna Analiza**: Wykorzystuje AI do głębokiej analizy konwersacji
- 📊 **Kompleksowe Raporty**: Generuje ustrukturyzowane raporty dla każdego projektu
- 🌍 **Wsparcie Wielojęzyczne**: Tworzy raporty w 19 językach
- ⚡ **Pamięć Podręczna API**: Zmniejsza koszty API dzięki inteligentnemu buforowaniu
- 🔄 **Przetwarzanie Równoległe**: Efektywnie obsługuje wiele projektów
- 📈 **Śledzenie Tokenów**: Monitoruje zużycie i szacuje koszty
- 🎨 **Dostosowywalne Szablony**: Pełne wsparcie dla dostosowywania szablonów przez TOML

## Szybki Start

```bash
# Instalacja przez cargo
cargo install cc2report

# Generuj raport dla dzisiejszych rozmów
cc2report

# Generuj raport po polsku
cc2report --language pl
```

## Instalacja

### Ze Źródła

```bash
git clone https://github.com/signal-slot/cc2report
cd cc2report
cargo build --release
```

### Wymagania Wstępne

- Rust 1.75 lub nowszy
- Klucz API Anthropic (Claude)

## Konfiguracja

### Zmienne Środowiskowe

```bash
# Wymagane
export ANTHROPIC_API_KEY="twój-klucz-api"

# Opcjonalne
export CC2REPORT_CACHE_DIR="/ścieżka/do/cache"  # Domyślnie: ~/.cache/cc2report
export CC2REPORT_CACHE_TTL="3600"               # Sekundy (domyślnie: 1 godzina)
export CC2REPORT_MAX_PARALLEL="4"               # Domyślnie: 8
```

### Plik Konfiguracyjny

Utwórz `~/.config/cc2report/config.toml`:

```toml
[api]
base_url = "https://api.anthropic.com/v1"
max_retries = 3
retry_delay = 1000

[cache]
ttl_seconds = 3600
max_size_mb = 500

[processing]
max_parallel = 8
chunk_size = 100

[output]
format = "markdown"
include_stats = false
include_summary = true
```

## Użycie

### Podstawowe Użycie

Generuj raport dla dzisiejszych rozmów (domyślnie):

```bash
cc2report
```

Generuj raport dla wszystkich rozmów (bez filtra daty):

```bash
cc2report --all
```

Inne opcje:

```bash
# Określ plik wyjściowy
cc2report -o raport.md

# Generuj raport w innym języku
cc2report --language pl
```

### Zaawansowane Opcje

```bash
# Użyj własnego szablonu
cc2report --template mój-szablon.toml

# Dołącz szczegółowe statystyki
cc2report --include-stats

# Wyłącz buforowanie
cc2report --no-cache

# Pokaż postęp
cc2report --progress
```

### Wiele Projektów

```bash
# Przetwarzaj wiele plików
cc2report projekt1.jsonl projekt2.jsonl projekt3.jsonl

# Przetwarzaj katalog
cc2report --dir ./dzienniki-konwersacji/

# Z przetwarzaniem równoległym
cc2report --dir ./logs/ --max-parallel 4
```

## Obsługiwane Języki

cc2report obsługuje generowanie raportów w następujących językach:
- Angielski (en)
- Japoński (ja)
- Chiński Uproszczony (zh)
- Koreański (ko)
- Hiszpański (es)
- Francuski (fr)
- Niemiecki (de)
- Portugalski (pt)
- Rosyjski (ru)
- Włoski (it)
- Holenderski (nl)
- Polski (pl)
- Turecki (tr)
- Arabski (ar)
- Hindi (hi)
- Tajski (th)
- Wietnamski (vi)
- Indonezyjski (id)
- Malajski (ms)

## Struktura Projektu

```
cc2report/
├── src/
│   ├── main.rs           # Punkt wejścia
│   ├── config.rs         # Zarządzanie konfiguracją
│   ├── parser.rs         # Parser dziennika konwersacji
│   ├── ai_analyzer.rs    # Integracja z Claude API
│   ├── cache.rs          # System buforowania
│   ├── token_tracker.rs  # Śledzenie zużycia tokenów
│   └── error.rs          # Obsługa błędów
├── templates/            # Szablony raportów
└── tests/                # Zestaw testów
```

## Rozwój

### Uruchamianie Testów

```bash
# Uruchom wszystkie testy
cargo test

# Uruchom z wyjściem
cargo test -- --nocapture

# Uruchom konkretny test
cargo test nazwa_testu
```

### Budowanie

```bash
# Budowanie debug
cargo build

# Budowanie release
cargo build --release

# Budowanie ze wszystkimi funkcjami
cargo build --all-features
```

## Wkład w Projekt

Zachęcamy do współpracy! Zobacz [CONTRIBUTING.md](CONTRIBUTING.md) po wytyczne.

## Licencja

Ten projekt jest licencjonowany na licencji MIT - zobacz plik [LICENSE](LICENSE) po szczegóły.

## Podziękowania

- Anthropic za Claude API
- Społeczność Rust za świetne biblioteki
- Współtwórcy i użytkownicy projektu
