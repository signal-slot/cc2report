# cc2report

*Lees in andere talen: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Een intelligente werkrapportgenerator voor Claude Code die conversatielogboeken analyseert en uitgebreide werkrapporten maakt met behulp van AI.

## Kenmerken

- 🔍 **Intelligente Analyse**: Gebruikt AI voor diepgaande conversatieanalyse
- 📊 **Uitgebreide Rapporten**: Genereert gestructureerde rapporten per project
- 🌍 **Meertalige Ondersteuning**: Creëert rapporten in 19 talen
- ⚡ **API Cache**: Vermindert API-kosten met intelligente caching
- 🔄 **Parallelle Verwerking**: Verwerkt meerdere projecten efficiënt
- 📈 **Token Tracking**: Monitort gebruik en schat kosten
- 🎨 **Aanpasbare Templates**: Volledige ondersteuning voor template-aanpassing via TOML

## Snel Starten

```bash
# Installeren via cargo
cargo install cc2report

# Genereer rapport voor de gesprekken van vandaag
cc2report

# Genereer rapport in het Nederlands
cc2report --language nl
```

## Installatie

### Vanaf Broncode

```bash
git clone https://github.com/signal-slot/cc2report
cd cc2report
cargo build --release
```

### Vereisten

- Rust 1.75 of hoger
- Anthropic API-sleutel (Claude)

## Configuratie

### Omgevingsvariabelen

```bash
# Verplicht
export ANTHROPIC_API_KEY="uw-api-sleutel"

# Optioneel
export CC2REPORT_CACHE_DIR="/pad/naar/cache"    # Standaard: ~/.cache/cc2report
export CC2REPORT_CACHE_TTL="3600"               # Seconden (standaard: 1 uur)
export CC2REPORT_MAX_PARALLEL="4"               # Standaard: 8
```

### Configuratiebestand

Maak `~/.config/cc2report/config.toml`:

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

## Gebruik

### Basisgebruik

Genereer een rapport voor de gesprekken van vandaag (standaard):

```bash
cc2report
```

Genereer een rapport voor alle gesprekken (zonder datumfilter):

```bash
cc2report --all
```

Andere opties:

```bash
# Specificeer uitvoerbestand
cc2report -o rapport.md

# Genereer rapport in andere taal
cc2report --language nl
```

### Geavanceerde Opties

```bash
# Gebruik aangepast template
cc2report --template mijn-template.toml

# Inclusief gedetailleerde statistieken
cc2report --include-stats

# Schakel cache uit
cc2report --no-cache

# Toon voortgang
cc2report --progress
```

### Meerdere Projecten

```bash
# Verwerk meerdere bestanden
cc2report project1.jsonl project2.jsonl project3.jsonl

# Verwerk een directory
cc2report --dir ./conversatie-logs/

# Met parallelle verwerking
cc2report --dir ./logs/ --max-parallel 4
```

## Ondersteunde Talen

cc2report ondersteunt rapportgeneratie in de volgende talen:
- Engels (en)
- Japans (ja)
- Vereenvoudigd Chinees (zh)
- Koreaans (ko)
- Spaans (es)
- Frans (fr)
- Duits (de)
- Portugees (pt)
- Russisch (ru)
- Italiaans (it)
- Nederlands (nl)
- Pools (pl)
- Turks (tr)
- Arabisch (ar)
- Hindi (hi)
- Thai (th)
- Vietnamees (vi)
- Indonesisch (id)
- Maleis (ms)

## Projectstructuur

```
cc2report/
├── src/
│   ├── main.rs           # Toegangspunt
│   ├── config.rs         # Configuratiebeheer
│   ├── parser.rs         # Conversatielog parser
│   ├── ai_analyzer.rs    # Claude API integratie
│   ├── cache.rs          # Cache systeem
│   ├── token_tracker.rs  # Token gebruik tracking
│   └── error.rs          # Foutafhandeling
├── templates/            # Rapport templates
└── tests/                # Test suite
```

## Ontwikkeling

### Tests Uitvoeren

```bash
# Voer alle tests uit
cargo test

# Voer uit met output
cargo test -- --nocapture

# Voer specifieke test uit
cargo test test_naam
```

### Bouwen

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build met alle features
cargo build --all-features
```

## Bijdragen

Bijdragen zijn welkom! Zie [CONTRIBUTING.md](CONTRIBUTING.md) voor richtlijnen.

## Licentie

Dit project is gelicentieerd onder de MIT-licentie - zie het [LICENSE](LICENSE) bestand voor details.

## Dankwoord

- Anthropic voor Claude API
- De Rust-gemeenschap voor uitstekende bibliotheken
- Projectbijdragers en gebruikers
