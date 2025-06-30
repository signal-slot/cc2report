# cc2report

*Diğer dillerde okuyun: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Claude Code için konuşma günlüklerini analiz eden ve AI kullanarak kapsamlı çalışma raporları oluşturan akıllı bir çalışma raporu oluşturucu.

## Özellikler

- 🔍 **Akıllı Analiz**: Derin konuşma analizi için AI kullanır
- 📊 **Kapsamlı Raporlar**: Proje bazında yapılandırılmış raporlar üretir
- 🌍 **Çok Dilli Destek**: 19 dilde rapor oluşturur
- ⚡ **API Önbelleği**: Akıllı önbellekleme ile API maliyetlerini azaltır
- 🔄 **Paralel İşleme**: Birden fazla projeyi verimli bir şekilde işler
- 📈 **Token Takibi**: Kullanımı izler ve maliyetleri tahmin eder
- 🎨 **Özelleştirilebilir Şablonlar**: TOML ile tam şablon özelleştirme desteği

## Hızlı Başlangıç

```bash
# Cargo ile kurulum
cargo install cc2report

# Bugünün konuşmaları için rapor oluştur
cc2report

# Türkçe rapor oluştur
cc2report --language tr
```

## Kurulum

### Kaynak Koddan

```bash
git clone https://github.com/signal-slot/cc2report
cd cc2report
cargo build --release
```

### Ön Koşullar

- Rust 1.75 veya daha yeni
- Anthropic API anahtarı (Claude)

## Yapılandırma

### Ortam Değişkenleri

```bash
# Zorunlu
export ANTHROPIC_API_KEY="api-anahtarınız"

# İsteğe bağlı
export CC2REPORT_CACHE_DIR="/önbellek/yolu"     # Varsayılan: ~/.cache/cc2report
export CC2REPORT_CACHE_TTL="3600"               # Saniye (varsayılan: 1 saat)
export CC2REPORT_MAX_PARALLEL="4"               # Varsayılan: 8
```

### Yapılandırma Dosyası

`~/.config/cc2report/config.toml` oluşturun:

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

## Kullanım

### Temel Kullanım

Bugünün konuşmaları için rapor oluştur (varsayılan):

```bash
cc2report
```

Tüm konuşmalar için rapor oluştur (tarih filtresi olmadan):

```bash
cc2report --all
```

Diğer seçenekler:

```bash
# Çıktı dosyasını belirt
cc2report -o rapor.md

# Başka bir dilde rapor oluştur
cc2report --language tr
```

### Gelişmiş Seçenekler

```bash
# Özel şablon kullan
cc2report --template benim-sablonum.toml

# Detaylı istatistikleri dahil et
cc2report --include-stats

# Önbelleği devre dışı bırak
cc2report --no-cache

# İlerlemeyi göster
cc2report --progress
```

### Birden Fazla Proje

```bash
# Birden fazla dosyayı işle
cc2report proje1.jsonl proje2.jsonl proje3.jsonl

# Bir dizini işle
cc2report --dir ./konusma-gunlukleri/

# Paralel işleme ile
cc2report --dir ./logs/ --max-parallel 4
```

## Desteklenen Diller

cc2report aşağıdaki dillerde rapor oluşturmayı destekler:
- İngilizce (en)
- Japonca (ja)
- Basitleştirilmiş Çince (zh)
- Korece (ko)
- İspanyolca (es)
- Fransızca (fr)
- Almanca (de)
- Portekizce (pt)
- Rusça (ru)
- İtalyanca (it)
- Hollandaca (nl)
- Lehçe (pl)
- Türkçe (tr)
- Arapça (ar)
- Hintce (hi)
- Tayca (th)
- Vietnamca (vi)
- Endonezce (id)
- Malayca (ms)

## Proje Yapısı

```
cc2report/
├── src/
│   ├── main.rs           # Giriş noktası
│   ├── config.rs         # Yapılandırma yönetimi
│   ├── parser.rs         # Konuşma günlüğü ayrıştırıcı
│   ├── ai_analyzer.rs    # Claude API entegrasyonu
│   ├── cache.rs          # Önbellek sistemi
│   ├── token_tracker.rs  # Token kullanım takibi
│   └── error.rs          # Hata yönetimi
├── templates/            # Rapor şablonları
└── tests/                # Test paketi
```

## Geliştirme

### Testleri Çalıştırma

```bash
# Tüm testleri çalıştır
cargo test

# Çıktı ile çalıştır
cargo test -- --nocapture

# Belirli bir testi çalıştır
cargo test test_adı
```

### Derleme

```bash
# Hata ayıklama derlemesi
cargo build

# Yayın derlemesi
cargo build --release

# Tüm özelliklerle derleme
cargo build --all-features
```

## Katkıda Bulunma

Katkılarınızı bekliyoruz! Yönergeler için [CONTRIBUTING.md](CONTRIBUTING.md) dosyasına bakın.

## Lisans

Bu proje MIT lisansı altında lisanslanmıştır - ayrıntılar için [LICENSE](LICENSE) dosyasına bakın.

## Teşekkürler

- Claude API için Anthropic'e
- Harika kütüphaneler için Rust topluluğuna
- Proje katkıcıları ve kullanıcılarına
