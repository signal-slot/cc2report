# cc2report

*다른 언어로 읽기: [English](README.md) | [日本語](README-ja.md) | [简体中文](README-zh.md) | [Español](README-es.md) | [Français](README-fr.md) | [Deutsch](README-de.md) | [Português](README-pt.md) | [Русский](README-ru.md) | [Italiano](README-it.md) | [Nederlands](README-nl.md) | [Polski](README-pl.md) | [Türkçe](README-tr.md) | [العربية](README-ar.md) | [हिन्दी](README-hi.md) | [ไทย](README-th.md) | [Tiếng Việt](README-vi.md) | [Bahasa Indonesia](README-id.md) | [Bahasa Melayu](README-ms.md)*

Claude Code의 대화 로그를 분석하고 AI를 사용하여 포괄적인 작업 보고서를 생성하는 지능형 작업 보고서 생성기입니다.

## 기능

- **AI 기반 분석**: OpenAI의 GPT 모델을 사용하여 대화를 분석하고 지능적인 요약 생성
- **다국어 지원**: 19개 언어로 보고서 생성 (시스템 로케일에서 자동 감지)
- **스마트 캐싱**: API 응답을 캐시하여 비용 절감 및 성능 향상 (캐시된 실행 시 12배 속도 향상)
- **병렬 처리**: 여러 프로젝트를 동시에 처리하여 더 빠른 분석
- **유연한 날짜 필터링**: 특정 날짜, 주 또는 월에 대한 보고서 생성
- **진행률 표시기**: 장시간 실행 작업에 대한 시각적 진행률 표시줄
- **토큰 사용량 추적**: API 사용량 및 비용 모니터링
- **템플릿 사용자 지정**: AI 프롬프트 및 보고서 형식 사용자 지정

## 설치

### 소스에서

```bash
# 저장소 복제
git clone https://github.com/signal-slot/cc2report.git
cd cc2report

# 빌드 및 설치
cargo build --release
cargo install --path .
```

### 필수 조건

- Rust 1.70 이상
- OpenAI API 키

## 구성

### API 키 설정

OpenAI API 키를 환경 변수로 설정:

```bash
export OPENAI_API_KEY="your-api-key-here"
```

또는 명령줄을 통해 직접 전달:

```bash
cc2report --api-key "your-api-key-here"
```

### 기본 경로

- **로그 디렉토리**: `~/.claude/projects/` (Claude Code의 기본 프로젝트 디렉토리)
- **캐시 디렉토리**: `~/.cache/cc2report/`
- **템플릿 파일**: `./cc2report.toml` 또는 `~/.config/cc2report/templates.toml`

## 사용법

### 기본 사용법

오늘의 대화 보고서 생성 (기본값):

```bash
cc2report
```

모든 대화의 보고서 생성 (날짜 필터 없음):

```bash
cc2report --all
```

### 날짜 필터링

```bash
# 특정 날짜
cc2report --date 2024-07-01

# 날짜 범위
cc2report --from 2024-07-01 --to 2024-07-07

# 이번 주
cc2report --weekly

# 이번 달
cc2report --monthly
```

### 출력 옵션

```bash
# 파일로 저장
cc2report --output report.md

# JSON 형식
cc2report --format json --output report.json

# 언어 지정 (기본값은 자동 감지)
cc2report --lang ja  # 일본어
cc2report --lang zh  # 중국어
cc2report --lang es  # 스페인어

# 예시: 오늘의 보고서 (한국어)
cc2report --lang ko

# 예시: 이번 주 보고서 (스페인어)
cc2report --weekly --lang es
```

### 성능 옵션

```bash
# 병렬 처리 활성화 (최대 10)
cc2report --parallel 4

# 진행률 표시기 비활성화
cc2report --quiet

# 토큰 사용량 및 비용 표시
cc2report --show-token-usage
```

### 캐시 관리

```bash
# 캐시 지우기
cc2report --clear-cache

# 캐시 정보 표시
cc2report --cache-info
```

### 템플릿 사용자 지정

템플릿 파일 생성:

```bash
cc2report --generate-template my-template.toml
```

프롬프트 및 보고서 형식을 사용자 지정하려면 템플릿을 편집하세요.

## 지원 언어

도구는 시스템 언어를 자동으로 감지하고 그에 따라 보고서를 생성합니다. 지원 언어:

- 영어 (en)
- 일본어 (ja)
- 중국어 (zh)
- 한국어 (ko)
- 스페인어 (es)
- 프랑스어 (fr)
- 독일어 (de)
- 포르투갈어 (pt)
- 러시아어 (ru)
- 이탈리아어 (it)
- 네덜란드어 (nl)
- 폴란드어 (pl)
- 터키어 (tr)
- 아랍어 (ar)
- 힌디어 (hi)
- 태국어 (th)
- 베트남어 (vi)
- 인도네시아어 (id)
- 말레이어 (ms)

## 보고서 구조

생성된 보고서에는 다음이 포함됩니다:

- **프로젝트 제목 및 목표**: 작업 중인 내용의 명확한 요약
- **활동**: 수행된 작업의 분류된 목록
- **제공된 가치**: 구체적인 성과 및 구현된 기능
- **기술적 개선**: 코드 품질 및 성능 향상
- **미해결 문제**: 차단 요소 또는 보류 중인 작업

## 비용 최적화

이 도구에는 API 비용을 최소화하기 위한 여러 기능이 포함되어 있습니다:

1. **캐싱**: 응답은 24시간 동안 캐시됩니다
2. **모델 선택**: 비용/품질 트레이드오프에 따라 다른 모델 선택
3. **토큰 추적**: 예산 내에서 유지하기 위해 사용량 모니터링

### 모델 가격 (2024년 기준)

| 모델 | 입력 비용 | 출력 비용 | 권장 사항 |
|------|-----------|-----------|----------|
| gpt-4o (기본값) | $2.50/1M | $10.00/1M | 최고 품질 |
| gpt-4o-mini | $0.15/1M | $0.60/1M | 최고의 가치 |
| gpt-3.5-turbo | $0.50/1M | $1.50/1M | 예산 옵션 |

## 예제 보고서

```markdown
# 작업 보고서 - 2024-07-01

## cc2report - 작업 보고서 생성기

**목표**: Claude Code 로그에서 사람이 읽을 수 있는 작업 보고서 생성

**활동**:
- 개발 - GPT-4와 OpenAI API 통합 구현
- 기능 추가 - 지능적인 분류를 통한 스마트 분석 개발
- UI 향상 - 명령줄 인터페이스 경험 개선

**제공된 가치**:
- "기타 작업" 카테고리를 90% 감소
- 보고서 가독성 크게 향상
- 다국어 지원 (19개 언어) 달성

**기술적 개선**:
- 성능 최적화
- 오류 처리 강화
```

## 문제 해결

### 일반적인 문제

1. **"OpenAI API key is required"**
   - 환경에 `OPENAI_API_KEY`가 설정되어 있는지 확인
   - 또는 `--api-key` 옵션 사용

2. **"Log directory does not exist"**
   - `--log-dir`로 올바른 경로 지정
   - 기본값은 `~/.claude/projects/`

3. **속도 제한 오류**
   - 병렬 요청 감소: `--parallel 1`
   - 하위 계층 모델 사용: `--model gpt-3.5-turbo`

## 개발

### 소스에서 빌드

```bash
# 개발 빌드
cargo build

# 테스트 실행
cargo test

# 디버그 출력으로 실행
RUST_LOG=debug cargo run
```

### 아키텍처

프로젝트는 다음 모듈로 구성됩니다:

- `parser`: JSONL 로그 파일 구문 분석
- `conversation_analyzer`: 대화에서 주제 및 컨텍스트 추출
- `ai_analyzer`: OpenAI API 통합
- `smart_analyzer`: 보고서 생성
- `cache`: API 응답 캐싱
- `templates`: 사용자 지정 가능한 프롬프트 및 형식
- `config`: 구성 관리
- `error`: 오류 처리
- `cli`: 명령줄 인터페이스

## 기여

기여를 환영합니다! 가이드라인은 [CONTRIBUTING.md](CONTRIBUTING.md)를 참조하세요.

## 라이선스

이 프로젝트는 MIT 라이선스에 따라 라이선스가 부여됩니다 - 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

## 감사의 말

- [Claude Code](https://github.com/cline/cline) (이전 Claude Engineer)를 위해 제작됨
- 지능적인 분석을 위해 OpenAI의 GPT 모델 사용
- AI 지원 개발에서 자동화된 작업 보고의 필요성에서 영감을 받음