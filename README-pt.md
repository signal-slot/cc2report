# cc2report

*Leia em outros idiomas: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Um gerador inteligente de relatórios de trabalho para Claude Code que analisa logs de conversação e cria relatórios de trabalho abrangentes usando IA.

## Recursos

- **Análise alimentada por IA**: Usa os modelos GPT da OpenAI para analisar conversas e gerar resumos inteligentes
- **Suporte multilíngue**: Gera relatórios em 19 idiomas (detectados automaticamente a partir da configuração regional do sistema)
- **Cache inteligente**: Armazena em cache as respostas da API para reduzir custos e melhorar o desempenho (12x mais rápido em execuções em cache)
- **Processamento paralelo**: Processa vários projetos simultaneamente para análise mais rápida
- **Filtragem flexível de datas**: Gera relatórios para datas, semanas ou meses específicos
- **Indicadores de progresso**: Barras de progresso visuais para operações de longa duração
- **Rastreamento de uso de tokens**: Monitora o uso e os custos da API
- **Personalização de modelos**: Personalize prompts de IA e formatos de relatório

## Instalação

### A partir do código-fonte

```bash
# Clonar o repositório
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# Compilar e instalar
cargo build --release
cargo install --path .
```

### Pré-requisitos

- Rust 1.70 ou superior
- Chave API da OpenAI

## Configuração

### Configuração da chave API

Defina sua chave API da OpenAI como variável de ambiente:

```bash
export OPENAI_API_KEY="sua-chave-aqui"
```

Ou passe diretamente via linha de comando:

```bash
cc2report --api-key "sua-chave-aqui"
```

### Caminhos padrão

- **Diretório de logs**: `~/.claude/projects/` (diretório de projetos padrão do Claude Code)
- **Diretório de cache**: `~/.cache/cc2report/`
- **Arquivo de modelo**: `./cc2report.toml` ou `~/.config/cc2report/templates.toml`

## Uso

### Uso básico

Gerar um relatório para as conversas de hoje (padrão):

```bash
cc2report
```

Gerar um relatório para todas as conversas (sem filtro de data):

```bash
cc2report --all
```

### Filtragem por data

```bash
# Data específica
cc2report --date 2024-07-01

# Intervalo de datas
cc2report --from 2024-07-01 --to 2024-07-07

# Semana atual
cc2report --weekly

# Mês atual
cc2report --monthly
```

### Opções de saída

```bash
# Salvar em arquivo
cc2report --output relatorio.md

# Formato JSON
cc2report --format json --output relatorio.json

# Especificar idioma (detectado automaticamente por padrão)
cc2report --lang ja  # Japonês
cc2report --lang zh  # Chinês
cc2report --lang es  # Espanhol

# Exemplo: Relatório de hoje em português
cc2report --lang pt

# Exemplo: Relatório desta semana em espanhol
cc2report --weekly --lang es
```

### Opções de desempenho

```bash
# Habilitar processamento paralelo (máximo 10)
cc2report --parallel 4

# Desabilitar indicadores de progresso
cc2report --quiet

# Mostrar uso de tokens e custos
cc2report --show-token-usage
```

### Gerenciamento de cache

```bash
# Limpar cache
cc2report --clear-cache

# Mostrar informações do cache
cc2report --cache-info
```

### Personalização de modelos

Gerar um arquivo de modelo:

```bash
cc2report --generate-template meu-modelo.toml
```

Edite o modelo para personalizar prompts e formatos de relatório.

## Idiomas suportados

A ferramenta detecta automaticamente o idioma do seu sistema e gera relatórios de acordo. Os idiomas suportados incluem:

- Inglês (en)
- Japonês (ja)
- Chinês (zh)
- Coreano (ko)
- Espanhol (es)
- Francês (fr)
- Alemão (de)
- Português (pt)
- Russo (ru)
- Italiano (it)
- Holandês (nl)
- Polonês (pl)
- Turco (tr)
- Árabe (ar)
- Hindi (hi)
- Tailandês (th)
- Vietnamita (vi)
- Indonésio (id)
- Malaio (ms)

## Estrutura do relatório

Os relatórios gerados incluem:

- **Título e objetivo do projeto**: Resumo claro do que estava sendo trabalhado
- **Atividades**: Lista categorizada do trabalho realizado
- **Valor entregue**: Realizações concretas e recursos implementados
- **Melhorias técnicas**: Melhorias na qualidade do código e desempenho
- **Problemas não resolvidos**: Quaisquer bloqueios ou tarefas pendentes

## Otimização de custos

A ferramenta inclui vários recursos para minimizar os custos da API:

1. **Cache**: As respostas são armazenadas em cache por 24 horas
2. **Seleção de modelo**: Escolha entre diferentes modelos com base no equilíbrio custo/qualidade
3. **Rastreamento de tokens**: Monitore o uso para permanecer dentro do orçamento

### Preços dos modelos (a partir de 2024)

| Modelo | Custo de entrada | Custo de saída | Recomendação |
|--------|------------------|----------------|--------------|
| gpt-4o (padrão) | $2.50/1M | $10.00/1M | Melhor qualidade |
| gpt-4o-mini | $0.15/1M | $0.60/1M | Melhor custo-benefício |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | Opção econômica |

## Exemplo de relatório

```markdown
# Relatório de trabalho - 2024-07-01

## cc2report - Gerador de relatórios de trabalho

**Objetivo**: Gerar relatórios de trabalho legíveis por humanos a partir de logs do Claude Code

**Atividades**:
- Desenvolvimento - Implementada integração da API OpenAI com GPT-4
- Adição de recursos - Desenvolvida análise inteligente com categorização inteligente
- Melhoria da interface - Melhorada a experiência da interface de linha de comando

**Valor entregue**:
- Reduzida a categoria "Outras tarefas" em 90%
- Melhorada significativamente a legibilidade do relatório
- Alcançado suporte multilíngue (19 idiomas)

**Melhorias técnicas**:
- Desempenho otimizado
- Tratamento de erros aprimorado
```

## Solução de problemas

### Problemas comuns

1. **"OpenAI API key is required"**
   - Certifique-se de que `OPENAI_API_KEY` esteja definido em seu ambiente
   - Ou use a opção `--api-key`

2. **"Log directory does not exist"**
   - Especifique o caminho correto com `--log-dir`
   - O padrão é `~/.claude/projects/`

3. **Erros de limite de taxa**
   - Reduza as solicitações paralelas: `--parallel 1`
   - Use um modelo de nível inferior: `--model gpt-3.5-turbo`

## Desenvolvimento

### Compilando a partir do código-fonte

```bash
# Build de desenvolvimento
cargo build

# Executar testes
cargo test

# Executar com saída de depuração
RUST_LOG=debug cargo run
```

### Arquitetura

O projeto está organizado nos seguintes módulos:

- `parser`: Análise de arquivos de log JSONL
- `conversation_analyzer`: Extrai tópicos e contexto das conversas
- `ai_analyzer`: Integração com a API OpenAI
- `smart_analyzer`: Geração de relatórios
- `cache`: Cache de respostas da API
- `templates`: Prompts e formatos personalizáveis
- `config`: Gerenciamento de configuração
- `error`: Tratamento de erros
- `cli`: Interface de linha de comando

## Contribuindo

Contribuições são bem-vindas! Por favor, consulte [CONTRIBUTING.md](CONTRIBUTING.md) para diretrizes.

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Agradecimentos

- Construído para [Claude Code](https://github.com/cline/cline) (anteriormente Claude Engineer)
- Usa os modelos GPT da OpenAI para análise inteligente
- Inspirado pela necessidade de relatórios de trabalho automatizados no desenvolvimento assistido por IA