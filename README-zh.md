# cc2report

*其他语言版本: [English](README.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

一个智能的工作报告生成器，用于分析 Claude Code 的对话日志并使用 AI 创建全面的工作报告。

## 功能特点

- **AI 驱动分析**: 使用 OpenAI 的 GPT 模型分析对话并生成智能摘要
- **多语言支持**: 生成 19 种语言的报告（从系统区域设置自动检测）
- **智能缓存**: 缓存 API 响应以降低成本并提高性能（缓存运行时速度提升 12 倍）
- **并行处理**: 同时处理多个项目以加快分析速度
- **灵活的日期过滤**: 生成特定日期、周或月的报告
- **进度指示器**: 长时间运行操作的可视化进度条
- **令牌使用跟踪**: 监控 API 使用情况和成本
- **模板自定义**: 自定义 AI 提示和报告格式

## 安装

### 从源代码

```bash
# 克隆仓库
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# 构建并安装
cargo build --release
cargo install --path .
```

### 先决条件

- Rust 1.70 或更高版本
- OpenAI API 密钥

## 配置

### API 密钥设置

将您的 OpenAI API 密钥设置为环境变量：

```bash
export OPENAI_API_KEY="your-api-key-here"
```

或通过命令行直接传递：

```bash
cc2report --api-key "your-api-key-here"
```

### 默认路径

- **日志目录**: `~/.claude/projects/`（Claude Code 的默认项目目录）
- **缓存目录**: `~/.cache/cc2report/`
- **模板文件**: `./cc2report.toml` 或 `~/.config/cc2report/templates.toml`

## 使用方法

### 基本用法

生成今天的对话报告（默认）：

```bash
cc2report
```

生成所有对话的报告（无日期过滤）：

```bash
cc2report --all
```

### 日期过滤

```bash
# 特定日期
cc2report --date 2024-07-01

# 日期范围
cc2report --from 2024-07-01 --to 2024-07-07

# 本周
cc2report --weekly

# 本月
cc2report --monthly
```

### 输出选项

```bash
# 保存到文件
cc2report --output report.md

# JSON 格式
cc2report --format json --output report.json

# 指定语言（默认自动检测）
cc2report --lang ja  # 日语
cc2report --lang zh  # 中文
cc2report --lang es  # 西班牙语

# 示例：今天的报告（中文）
cc2report --lang zh

# 示例：本周的报告（西班牙语）
cc2report --weekly --lang es
```

### 性能选项

```bash
# 启用并行处理（最多 10 个）
cc2report --parallel 4

# 禁用进度指示器
cc2report --quiet

# 显示令牌使用情况和成本
cc2report --show-token-usage
```

### 缓存管理

```bash
# 清除缓存
cc2report --clear-cache

# 显示缓存信息
cc2report --cache-info
```

### 模板自定义

生成模板文件：

```bash
cc2report --generate-template my-template.toml
```

编辑模板以自定义提示和报告格式。

## 支持的语言

该工具会自动检测您的系统语言并相应地生成报告。支持的语言包括：

- 英语 (en)
- 日语 (ja)
- 中文 (zh)
- 韩语 (ko)
- 西班牙语 (es)
- 法语 (fr)
- 德语 (de)
- 葡萄牙语 (pt)
- 俄语 (ru)
- 意大利语 (it)
- 荷兰语 (nl)
- 波兰语 (pl)
- 土耳其语 (tr)
- 阿拉伯语 (ar)
- 印地语 (hi)
- 泰语 (th)
- 越南语 (vi)
- 印尼语 (id)
- 马来语 (ms)

## 报告结构

生成的报告包括：

- **项目标题和目标**: 清晰总结正在进行的工作
- **活动**: 按类别划分的工作列表
- **交付价值**: 具体成就和实现的功能
- **技术改进**: 代码质量和性能增强
- **未解决的问题**: 任何阻塞因素或待处理任务

## 成本优化

该工具包含几个功能以最小化 API 成本：

1. **缓存**: 响应缓存 24 小时
2. **模型选择**: 根据成本/质量权衡选择不同的模型
3. **令牌跟踪**: 监控使用情况以保持在预算内

### 模型定价（截至 2024 年）

| 模型 | 输入成本 | 输出成本 | 推荐 |
|------|----------|----------|------|
| gpt-4o（默认） | $2.50/1M | $10.00/1M | 最佳质量 |
| gpt-4o-mini | $0.15/1M | $0.60/1M | 最佳价值 |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | 预算选项 |

## 示例报告

```markdown
# 工作报告 - 2024-07-01

## cc2report - 工作报告生成器

**目标**: 从 Claude Code 日志生成人类可读的工作报告

**活动**:
- 开发 - 实现了与 GPT-4 的 OpenAI API 集成
- 功能添加 - 开发了具有智能分类的智能分析
- UI 增强 - 改进了命令行界面体验

**交付价值**:
- 将"其他任务"类别减少了 90%
- 显著提高了报告可读性
- 实现了多语言支持（19 种语言）

**技术改进**:
- 优化了性能
- 增强了错误处理
```

## 故障排除

### 常见问题

1. **"OpenAI API key is required"**
   - 确保在环境中设置了 `OPENAI_API_KEY`
   - 或使用 `--api-key` 选项

2. **"Log directory does not exist"**
   - 使用 `--log-dir` 指定正确的路径
   - 默认为 `~/.claude/projects/`

3. **速率限制错误**
   - 减少并行请求：`--parallel 1`
   - 使用较低级别的模型：`--model gpt-3.5-turbo`

## 开发

### 从源代码构建

```bash
# 开发构建
cargo build

# 运行测试
cargo test

# 使用调试输出运行
RUST_LOG=debug cargo run
```

### 架构

项目组织为以下模块：

- `parser`: JSONL 日志文件解析
- `conversation_analyzer`: 从对话中提取主题和上下文
- `ai_analyzer`: OpenAI API 集成
- `smart_analyzer`: 报告生成
- `cache`: API 响应缓存
- `templates`: 可自定义的提示和格式
- `config`: 配置管理
- `error`: 错误处理
- `cli`: 命令行界面

## 贡献

欢迎贡献！请参阅 [CONTRIBUTING.md](CONTRIBUTING.md) 了解准则。

## 许可证

该项目在 MIT 许可证下许可 - 有关详细信息，请参阅 [LICENSE](LICENSE) 文件。

## 致谢

- 为 [Claude Code](https://github.com/cline/cline)（前身为 Claude Engineer）构建
- 使用 OpenAI 的 GPT 模型进行智能分析
- 受到 AI 辅助开发中自动化工作报告需求的启发