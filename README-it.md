# cc2report

*Leggi in altre lingue: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Un generatore intelligente di report di lavoro per Claude Code che analizza i log delle conversazioni e crea report di lavoro completi utilizzando l'IA.

## Caratteristiche

- 🔍 **Analisi Intelligente**: Utilizza l'IA per analisi approfondite delle conversazioni
- 📊 **Report Completi**: Genera report strutturati per progetto
- 🌍 **Supporto Multilingue**: Crea report in 19 lingue
- ⚡ **Cache API**: Riduce i costi API con cache intelligente
- 🔄 **Elaborazione Parallela**: Gestisce efficacemente progetti multipli
- 📈 **Tracciamento Token**: Monitora l'utilizzo e stima i costi
- 🎨 **Template Personalizzabili**: Supporto completo per personalizzazione template tramite TOML

## Avvio Rapido

```bash
# Installa tramite cargo
cargo install cc2report

# Genera report per le conversazioni di oggi
cc2report

# Genera report in italiano
cc2report --language it
```

## Installazione

### Da Sorgente

```bash
git clone https://github.com/signal-slot/cc2report
cd cc2report
cargo build --release
```

### Prerequisiti

- Rust 1.75 o superiore
- Chiave API Anthropic (Claude)

## Configurazione

### Variabili d'Ambiente

```bash
# Obbligatorio
export ANTHROPIC_API_KEY="tua-chiave-api"

# Opzionale
export CC2REPORT_CACHE_DIR="/percorso/cache"     # Default: ~/.cache/cc2report
export CC2REPORT_CACHE_TTL="3600"               # Secondi (default: 1 ora)
export CC2REPORT_MAX_PARALLEL="4"               # Default: 8
```

### File di Configurazione

Crea `~/.config/cc2report/config.toml`:

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

## Utilizzo

### Utilizzo Base

Genera un report per le conversazioni di oggi (predefinito):

```bash
cc2report
```

Genera un report per tutte le conversazioni (senza filtro data):

```bash
cc2report --all
```

Altre opzioni:

```bash
# Specifica file di output
cc2report -o report.md

# Genera report in altra lingua
cc2report --language it
```

### Opzioni Avanzate

```bash
# Usa template personalizzato
cc2report --template mio-template.toml

# Includi statistiche dettagliate
cc2report --include-stats

# Disabilita cache
cc2report --no-cache

# Mostra progresso
cc2report --progress
```

### Progetti Multipli

```bash
# Elabora più file
cc2report progetto1.jsonl progetto2.jsonl progetto3.jsonl

# Elabora una directory
cc2report --dir ./log-conversazioni/

# Con elaborazione parallela
cc2report --dir ./logs/ --max-parallel 4
```

## Lingue Supportate

cc2report supporta la generazione di report nelle seguenti lingue:
- Inglese (en)
- Giapponese (ja)
- Cinese Semplificato (zh)
- Coreano (ko)
- Spagnolo (es)
- Francese (fr)
- Tedesco (de)
- Portoghese (pt)
- Russo (ru)
- Italiano (it)
- Olandese (nl)
- Polacco (pl)
- Turco (tr)
- Arabo (ar)
- Hindi (hi)
- Thai (th)
- Vietnamita (vi)
- Indonesiano (id)
- Malese (ms)

## Struttura del Progetto

```
cc2report/
├── src/
│   ├── main.rs           # Punto di ingresso
│   ├── config.rs         # Gestione configurazione
│   ├── parser.rs         # Parser log conversazione
│   ├── ai_analyzer.rs    # Integrazione Claude API
│   ├── cache.rs          # Sistema di cache
│   ├── token_tracker.rs  # Tracciamento utilizzo token
│   └── error.rs          # Gestione errori
├── templates/            # Template report
└── tests/                # Suite di test
```

## Sviluppo

### Esecuzione Test

```bash
# Esegui tutti i test
cargo test

# Esegui con output
cargo test -- --nocapture

# Esegui test specifico
cargo test nome_test
```

### Build

```bash
# Build debug
cargo build

# Build release
cargo build --release

# Build con tutte le features
cargo build --all-features
```

## Contribuire

I contributi sono benvenuti! Consulta [CONTRIBUTING.md](CONTRIBUTING.md) per le linee guida.

## Licenza

Questo progetto è concesso in licenza MIT - vedi il file [LICENSE](LICENSE) per i dettagli.

## Ringraziamenti

- Anthropic per Claude API
- La comunità Rust per le eccellenti librerie
- Contributori e utenti del progetto
