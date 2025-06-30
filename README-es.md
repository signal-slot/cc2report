# cc2report

*Lea esto en otros idiomas: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Un generador inteligente de informes de trabajo para Claude Code que analiza registros de conversación y crea informes de trabajo completos usando IA.

## Características

- **Análisis impulsado por IA**: Utiliza los modelos GPT de OpenAI para analizar conversaciones y generar resúmenes inteligentes
- **Soporte multiidioma**: Genera informes en 19 idiomas (detectados automáticamente desde la configuración regional del sistema)
- **Caché inteligente**: Almacena en caché las respuestas de la API para reducir costos y mejorar el rendimiento (12x más rápido en ejecuciones en caché)
- **Procesamiento paralelo**: Procesa múltiples proyectos simultáneamente para un análisis más rápido
- **Filtrado flexible de fechas**: Genera informes para fechas, semanas o meses específicos
- **Indicadores de progreso**: Barras de progreso visuales para operaciones de larga duración
- **Seguimiento de uso de tokens**: Monitorea el uso y los costos de la API
- **Personalización de plantillas**: Personaliza los prompts de IA y los formatos de informes

## Instalación

### Desde el código fuente

```bash
# Clonar el repositorio
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# Compilar e instalar
cargo build --release
cargo install --path .
```

### Prerrequisitos

- Rust 1.70 o superior
- Clave API de OpenAI

## Configuración

### Configuración de la clave API

Configure su clave API de OpenAI como variable de entorno:

```bash
export OPENAI_API_KEY="su-clave-aquí"
```

O pásela directamente por línea de comandos:

```bash
cc2report --api-key "su-clave-aquí"
```

### Rutas predeterminadas

- **Directorio de registros**: `~/.claude/projects/` (directorio de proyectos predeterminado de Claude Code)
- **Directorio de caché**: `~/.cache/cc2report/`
- **Archivo de plantilla**: `./cc2report.toml` o `~/.config/cc2report/templates.toml`

## Uso

### Uso básico

Generar un informe para las conversaciones de hoy (por defecto):

```bash
cc2report
```

Generar un informe para todas las conversaciones (sin filtro de fecha):

```bash
cc2report --all
```

### Filtrado por fecha

```bash
# Fecha específica
cc2report --date 2024-07-01

# Rango de fechas
cc2report --from 2024-07-01 --to 2024-07-07

# Semana actual
cc2report --weekly

# Mes actual
cc2report --monthly
```

### Opciones de salida

```bash
# Guardar en archivo
cc2report --output informe.md

# Formato JSON
cc2report --format json --output informe.json

# Especificar idioma (detectado automáticamente por defecto)
cc2report --lang ja  # Japonés
cc2report --lang zh  # Chino
cc2report --lang es  # Español

# Ejemplo: Informe de hoy en español
cc2report --lang es

# Ejemplo: Informe de esta semana en japonés
cc2report --weekly --lang ja
```

### Opciones de rendimiento

```bash
# Habilitar procesamiento paralelo (máximo 10)
cc2report --parallel 4

# Desactivar indicadores de progreso
cc2report --quiet

# Mostrar uso de tokens y costos
cc2report --show-token-usage
```

### Gestión de caché

```bash
# Limpiar caché
cc2report --clear-cache

# Mostrar información de caché
cc2report --cache-info
```

### Personalización de plantillas

Generar un archivo de plantilla:

```bash
cc2report --generate-template mi-plantilla.toml
```

Edite la plantilla para personalizar prompts y formatos de informes.

## Idiomas soportados

La herramienta detecta automáticamente el idioma de su sistema y genera informes en consecuencia. Los idiomas soportados incluyen:

- Inglés (en)
- Japonés (ja)
- Chino (zh)
- Coreano (ko)
- Español (es)
- Francés (fr)
- Alemán (de)
- Portugués (pt)
- Ruso (ru)
- Italiano (it)
- Neerlandés (nl)
- Polaco (pl)
- Turco (tr)
- Árabe (ar)
- Hindi (hi)
- Tailandés (th)
- Vietnamita (vi)
- Indonesio (id)
- Malayo (ms)

## Estructura del informe

Los informes generados incluyen:

- **Título y objetivo del proyecto**: Resumen claro de lo que se estaba trabajando
- **Actividades**: Lista categorizada del trabajo realizado
- **Valor entregado**: Logros concretos y características implementadas
- **Mejoras técnicas**: Mejoras en la calidad del código y el rendimiento
- **Problemas no resueltos**: Cualquier bloqueador o tarea pendiente

## Optimización de costos

La herramienta incluye varias características para minimizar los costos de la API:

1. **Caché**: Las respuestas se almacenan en caché durante 24 horas
2. **Selección de modelo**: Elija entre diferentes modelos según el equilibrio costo/calidad
3. **Seguimiento de tokens**: Monitoree el uso para mantenerse dentro del presupuesto

### Precios de modelos (a partir de 2024)

| Modelo | Costo de entrada | Costo de salida | Recomendación |
|--------|------------------|-----------------|---------------|
| gpt-4o (predeterminado) | $2.50/1M | $10.00/1M | Mejor calidad |
| gpt-4o-mini | $0.15/1M | $0.60/1M | Mejor valor |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | Opción económica |

## Informe de ejemplo

```markdown
# Informe de trabajo - 2024-07-01

## cc2report - Generador de informes de trabajo

**Objetivo**: Generar informes de trabajo legibles para humanos a partir de registros de Claude Code

**Actividades**:
- Desarrollo - Implementada integración de API de OpenAI con GPT-4
- Adición de características - Desarrollado análisis inteligente con categorización inteligente
- Mejora de UI - Mejorada la experiencia de interfaz de línea de comandos

**Valor entregado**:
- Reducida la categoría "Otras tareas" en un 90%
- Mejorada significativamente la legibilidad del informe
- Logrado soporte multiidioma (19 idiomas)

**Mejoras técnicas**:
- Rendimiento optimizado
- Manejo de errores mejorado
```

## Solución de problemas

### Problemas comunes

1. **"OpenAI API key is required"**
   - Asegúrese de que `OPENAI_API_KEY` esté configurado en su entorno
   - O use la opción `--api-key`

2. **"Log directory does not exist"**
   - Especifique la ruta correcta con `--log-dir`
   - El valor predeterminado es `~/.claude/projects/`

3. **Errores de límite de velocidad**
   - Reduzca las solicitudes paralelas: `--parallel 1`
   - Use un modelo de nivel inferior: `--model gpt-3.5-turbo`

## Desarrollo

### Compilación desde el código fuente

```bash
# Compilación de desarrollo
cargo build

# Ejecutar pruebas
cargo test

# Ejecutar con salida de depuración
RUST_LOG=debug cargo run
```

### Arquitectura

El proyecto está organizado en los siguientes módulos:

- `parser`: Análisis de archivos de registro JSONL
- `conversation_analyzer`: Extrae temas y contexto de las conversaciones
- `ai_analyzer`: Integración con la API de OpenAI
- `smart_analyzer`: Generación de informes
- `cache`: Caché de respuestas de API
- `templates`: Prompts y formatos personalizables
- `config`: Gestión de configuración
- `error`: Manejo de errores
- `cli`: Interfaz de línea de comandos

## Contribuciones

¡Las contribuciones son bienvenidas! Consulte [CONTRIBUTING.md](CONTRIBUTING.md) para conocer las pautas.

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - vea el archivo [LICENSE](LICENSE) para más detalles.

## Agradecimientos

- Construido para [Claude Code](https://github.com/cline/cline) (anteriormente Claude Engineer)
- Utiliza los modelos GPT de OpenAI para análisis inteligente
- Inspirado en la necesidad de informes de trabajo automatizados en el desarrollo asistido por IA