# cc2report

*Читать на других языках: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Интеллектуальный генератор рабочих отчетов для Claude Code, который анализирует журналы разговоров и создает комплексные рабочие отчеты с использованием ИИ.

## Возможности

- 🔍 **Интеллектуальный анализ**: Использует ИИ для глубокого анализа разговоров
- 📊 **Комплексные отчеты**: Генерирует структурированные отчеты по проектам
- 🌍 **Мультиязычная поддержка**: Создает отчеты на 19 языках
- ⚡ **Кеширование API**: Снижает затраты на API с интеллектуальным кешированием
- 🔄 **Параллельная обработка**: Эффективно обрабатывает несколько проектов
- 📈 **Отслеживание токенов**: Мониторит использование токенов и оценивает стоимость
- 🎨 **Настройка шаблонов**: Полная поддержка настройки шаблонов через TOML

## Быстрый старт

```bash
# Установка через cargo
cargo install cc2report

# Генерация отчета для сегодняшних разговоров
cc2report

# Генерация отчета на русском языке
cc2report --language ru
```

## Установка

### Из исходного кода

```bash
git clone https://github.com/signal-slot/cc2report
cd cc2report
cargo build --release
```

### Предварительные требования

- Rust 1.75 или новее
- API ключ от Anthropic (Claude)

## Конфигурация

### Переменные окружения

```bash
# Обязательно
export ANTHROPIC_API_KEY="ваш-api-ключ"

# Опционально
export CC2REPORT_CACHE_DIR="/путь/к/кешу"     # По умолчанию: ~/.cache/cc2report
export CC2REPORT_CACHE_TTL="3600"             # Секунды (по умолчанию: 1 час)
export CC2REPORT_MAX_PARALLEL="4"             # По умолчанию: 8
```

### Файл конфигурации

Создайте `~/.config/cc2report/config.toml`:

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

## Использование

### Основное использование

Генерация отчета для сегодняшних разговоров (по умолчанию):

```bash
cc2report
```

Генерация отчета для всех разговоров (без фильтра по дате):

```bash
cc2report --all
```

Другие опции:

```bash
# Указать выходной файл
cc2report -o report.md

# Генерация отчета на другом языке
cc2report --language ru
```

### Расширенные опции

```bash
# Использовать пользовательский шаблон
cc2report --template my-template.toml

# Включить подробную статистику
cc2report --include-stats

# Отключить кеширование
cc2report --no-cache

# Показать прогресс
cc2report --progress
```

### Обработка нескольких проектов

```bash
# Обработка нескольких файлов
cc2report project1.jsonl project2.jsonl project3.jsonl

# Обработка каталога
cc2report --dir ./conversation-logs/

# С параллельной обработкой
cc2report --dir ./logs/ --max-parallel 4
```

## Поддерживаемые языки

cc2report поддерживает генерацию отчетов на следующих языках:
- Английский (en)
- Японский (ja)
- Китайский упрощенный (zh)
- Корейский (ko)
- Испанский (es)
- Французский (fr)
- Немецкий (de)
- Португальский (pt)
- Русский (ru)
- Итальянский (it)
- Голландский (nl)
- Польский (pl)
- Турецкий (tr)
- Арабский (ar)
- Хинди (hi)
- Тайский (th)
- Вьетнамский (vi)
- Индонезийский (id)
- Малайский (ms)

## Структура проекта

```
cc2report/
├── src/
│   ├── main.rs           # Точка входа
│   ├── config.rs         # Управление конфигурацией
│   ├── parser.rs         # Парсер журнала разговора
│   ├── ai_analyzer.rs    # Интеграция с Claude API
│   ├── cache.rs          # Система кеширования
│   ├── token_tracker.rs  # Отслеживание использования токенов
│   └── error.rs          # Обработка ошибок
├── templates/            # Шаблоны отчетов
└── tests/                # Тестовый набор
```

## Разработка

### Запуск тестов

```bash
# Запуск всех тестов
cargo test

# Запуск с выводом
cargo test -- --nocapture

# Запуск конкретного теста
cargo test test_name
```

### Сборка

```bash
# Отладочная сборка
cargo build

# Релизная сборка
cargo build --release

# Сборка с всеми функциями
cargo build --all-features
```

## Вклад в проект

Мы приветствуем вклад в проект! Пожалуйста, ознакомьтесь с [CONTRIBUTING.md](CONTRIBUTING.md) для получения рекомендаций.

## Лицензия

Этот проект лицензирован под лицензией MIT - см. файл [LICENSE](LICENSE) для деталей.

## Благодарности

- Anthropic за Claude API
- Сообщество Rust за отличные библиотеки
- Участники и пользователи проекта
