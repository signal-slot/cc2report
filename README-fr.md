# cc2report

*Lire dans d'autres langues : [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Un générateur intelligent de rapports de travail pour Claude Code qui analyse les journaux de conversation et crée des rapports de travail complets en utilisant l'IA.

## Fonctionnalités

- **Analyse alimentée par l'IA** : Utilise les modèles GPT d'OpenAI pour analyser les conversations et générer des résumés intelligents
- **Support multilingue** : Génère des rapports en 19 langues (détection automatique à partir des paramètres régionaux du système)
- **Cache intelligent** : Met en cache les réponses API pour réduire les coûts et améliorer les performances (12x plus rapide sur les exécutions en cache)
- **Traitement parallèle** : Traite plusieurs projets simultanément pour une analyse plus rapide
- **Filtrage flexible des dates** : Génère des rapports pour des dates, semaines ou mois spécifiques
- **Indicateurs de progression** : Barres de progression visuelles pour les opérations de longue durée
- **Suivi de l'utilisation des tokens** : Surveille l'utilisation et les coûts de l'API
- **Personnalisation des modèles** : Personnalisez les invites IA et les formats de rapport

## Installation

### Depuis la source

```bash
# Cloner le dépôt
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# Compiler et installer
cargo build --release
cargo install --path .
```

### Prérequis

- Rust 1.70 ou supérieur
- Clé API OpenAI

## Configuration

### Configuration de la clé API

Définissez votre clé API OpenAI comme variable d'environnement :

```bash
export OPENAI_API_KEY="votre-clé-ici"
```

Ou passez-la directement via la ligne de commande :

```bash
cc2report --api-key "votre-clé-ici"
```

### Chemins par défaut

- **Répertoire des journaux** : `~/.claude/projects/` (répertoire de projet par défaut de Claude Code)
- **Répertoire du cache** : `~/.cache/cc2report/`
- **Fichier de modèle** : `./cc2report.toml` ou `~/.config/cc2report/templates.toml`

## Utilisation

### Utilisation de base

Générer un rapport pour les conversations d'aujourd'hui (par défaut) :

```bash
cc2report
```

Générer un rapport pour toutes les conversations (sans filtre de date) :

```bash
cc2report --all
```

### Filtrage par date

```bash
# Date spécifique
cc2report --date 2024-07-01

# Plage de dates
cc2report --from 2024-07-01 --to 2024-07-07

# Semaine en cours
cc2report --weekly

# Mois en cours
cc2report --monthly
```

### Options de sortie

```bash
# Enregistrer dans un fichier
cc2report --output rapport.md

# Format JSON
cc2report --format json --output rapport.json

# Spécifier la langue (détection automatique par défaut)
cc2report --lang ja  # Japonais
cc2report --lang zh  # Chinois
cc2report --lang es  # Espagnol

# Exemple : Rapport d'aujourd'hui en français
cc2report --lang fr

# Exemple : Rapport de cette semaine en espagnol
cc2report --weekly --lang es
```

### Options de performance

```bash
# Activer le traitement parallèle (maximum 10)
cc2report --parallel 4

# Désactiver les indicateurs de progression
cc2report --quiet

# Afficher l'utilisation des tokens et les coûts
cc2report --show-token-usage
```

### Gestion du cache

```bash
# Vider le cache
cc2report --clear-cache

# Afficher les informations du cache
cc2report --cache-info
```

### Personnalisation des modèles

Générer un fichier de modèle :

```bash
cc2report --generate-template mon-modele.toml
```

Modifiez le modèle pour personnaliser les invites et les formats de rapport.

## Langues prises en charge

L'outil détecte automatiquement la langue de votre système et génère des rapports en conséquence. Les langues prises en charge incluent :

- Anglais (en)
- Japonais (ja)
- Chinois (zh)
- Coréen (ko)
- Espagnol (es)
- Français (fr)
- Allemand (de)
- Portugais (pt)
- Russe (ru)
- Italien (it)
- Néerlandais (nl)
- Polonais (pl)
- Turc (tr)
- Arabe (ar)
- Hindi (hi)
- Thaï (th)
- Vietnamien (vi)
- Indonésien (id)
- Malais (ms)

## Structure du rapport

Les rapports générés incluent :

- **Titre et objectif du projet** : Résumé clair de ce sur quoi on travaillait
- **Activités** : Liste catégorisée du travail effectué
- **Valeur livrée** : Réalisations concrètes et fonctionnalités implémentées
- **Améliorations techniques** : Améliorations de la qualité du code et des performances
- **Problèmes non résolus** : Tout blocage ou tâche en attente

## Optimisation des coûts

L'outil comprend plusieurs fonctionnalités pour minimiser les coûts de l'API :

1. **Cache** : Les réponses sont mises en cache pendant 24 heures
2. **Sélection du modèle** : Choisissez entre différents modèles selon le compromis coût/qualité
3. **Suivi des tokens** : Surveillez l'utilisation pour rester dans le budget

### Tarification des modèles (à partir de 2024)

| Modèle | Coût d'entrée | Coût de sortie | Recommandation |
|--------|---------------|----------------|----------------|
| gpt-4o (par défaut) | 2,50$/1M | 10,00$/1M | Meilleure qualité |
| gpt-4o-mini | 0,15$/1M | 0,60$/1M | Meilleur rapport qualité-prix |
| gpt-3.5-turbo | 0,50$/1M | 1,50$/1M | Option économique |

## Exemple de rapport

```markdown
# Rapport de travail - 2024-07-01

## cc2report - Générateur de rapports de travail

**Objectif** : Générer des rapports de travail lisibles par l'homme à partir des journaux Claude Code

**Activités** :
- Développement - Implémentation de l'intégration API OpenAI avec GPT-4
- Ajout de fonctionnalités - Développement d'une analyse intelligente avec catégorisation intelligente
- Amélioration de l'interface - Amélioration de l'expérience de l'interface en ligne de commande

**Valeur livrée** :
- Réduction de la catégorie "Autres tâches" de 90%
- Amélioration significative de la lisibilité des rapports
- Support multilingue atteint (19 langues)

**Améliorations techniques** :
- Performance optimisée
- Gestion des erreurs améliorée
```

## Dépannage

### Problèmes courants

1. **"OpenAI API key is required"**
   - Assurez-vous que `OPENAI_API_KEY` est défini dans votre environnement
   - Ou utilisez l'option `--api-key`

2. **"Log directory does not exist"**
   - Spécifiez le bon chemin avec `--log-dir`
   - La valeur par défaut est `~/.claude/projects/`

3. **Erreurs de limite de débit**
   - Réduisez les requêtes parallèles : `--parallel 1`
   - Utilisez un modèle de niveau inférieur : `--model gpt-3.5-turbo`

## Développement

### Compilation depuis la source

```bash
# Compilation de développement
cargo build

# Exécuter les tests
cargo test

# Exécuter avec sortie de débogage
RUST_LOG=debug cargo run
```

### Architecture

Le projet est organisé en modules suivants :

- `parser` : Analyse des fichiers journaux JSONL
- `conversation_analyzer` : Extraction des sujets et du contexte des conversations
- `ai_analyzer` : Intégration de l'API OpenAI
- `smart_analyzer` : Génération de rapports
- `cache` : Mise en cache des réponses API
- `templates` : Invites et formats personnalisables
- `config` : Gestion de la configuration
- `error` : Gestion des erreurs
- `cli` : Interface de ligne de commande

## Contribution

Les contributions sont les bienvenues ! Veuillez consulter [CONTRIBUTING.md](CONTRIBUTING.md) pour les directives.

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Remerciements

- Construit pour [Claude Code](https://github.com/cline/cline) (anciennement Claude Engineer)
- Utilise les modèles GPT d'OpenAI pour une analyse intelligente
- Inspiré par le besoin de rapports de travail automatisés dans le développement assisté par IA