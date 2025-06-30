# Basic Usage Examples

This document provides practical examples of using cc2report.

## Prerequisites

Before running these examples, ensure you have:
1. Installed cc2report
2. Set your OpenAI API key: `export OPENAI_API_KEY="your-key-here"`
3. Have Claude Code project logs in `~/.claude/projects/`

## Example 1: Generate Today's Report

Generate a report for today's activities (default behavior):

```bash
cc2report
```

This will:
- Scan all projects in `~/.claude/projects/`
- Filter for today's activities only
- Generate a markdown report to stdout

To generate a report for all conversations without date filtering:

```bash
cc2report --all
```

## Example 2: Weekly Report to File

Generate a weekly report and save it:

```bash
cc2report --weekly --output weekly-report.md
```

## Example 3: Specific Date Range

Generate a report for a specific date range:

```bash
cc2report --from 2024-07-01 --to 2024-07-07 --output july-week1.md
```

## Example 4: JSON Output for Processing

Generate JSON output for further processing:

```bash
cc2report --format json --output report.json
```

Process with jq:
```bash
cc2report --format json | jq '.projects[].title'
```

## Example 5: Multi-language Reports

Generate reports in different languages:

```bash
# Japanese
cc2report --lang ja --output report-ja.md

# Chinese
cc2report --lang zh --output report-zh.md

# Spanish
cc2report --lang es --output report-es.md
```

## Example 6: Performance Optimization

Use caching and parallel processing:

```bash
# First run - builds cache
cc2report --parallel 4 --output report1.md

# Second run - uses cache (12x faster)
cc2report --parallel 4 --output report2.md

# Check cache size
cc2report --cache-info
```

## Example 7: Cost-Conscious Usage

Use cheaper models for regular reports:

```bash
# Daily standup (cheap, fast) - today's report by default
cc2report --model gpt-3.5-turbo --quiet

# Weekly detailed report (balanced)
cc2report --weekly --model gpt-4o-mini --show-token-usage

# Monthly executive report (highest quality)
cc2report --monthly --model gpt-4o --output monthly-exec.md
```

## Example 8: Custom Templates

Create and use custom templates:

```bash
# Generate template
cc2report --generate-template my-template.toml

# Edit my-template.toml to customize prompts

# Use custom template (auto-detected)
cc2report --output custom-report.md
```

## Example 9: Automation Script

Create a daily report script:

```bash
#!/bin/bash
# daily-report.sh

DATE=$(date +%Y-%m-%d)
OUTPUT_DIR="$HOME/work-reports"
mkdir -p "$OUTPUT_DIR"

# Generate today's report (default behavior)
cc2report \
  --model gpt-4o-mini \
  --quiet \
  --output "$OUTPUT_DIR/report-$DATE.md"

# Optional: Send notification
echo "Daily report generated: $OUTPUT_DIR/report-$DATE.md"
```

## Example 10: Team Report Generation

Generate reports for multiple team members:

```bash
#!/bin/bash
# team-reports.sh

TEAM_MEMBERS=("alice" "bob" "charlie")
DATE=$(date +%Y-%m-%d)

for member in "${TEAM_MEMBERS[@]}"; do
  echo "Generating report for $member..."
  
  cc2report \
    --log-dir "/shared/claude/projects/$member" \
    --output "reports/$member-$DATE.md" \
    --quiet
done
```

## Troubleshooting Common Issues

### No reports generated
```bash
# Check if log files exist
ls ~/.claude/projects/

# Check specific date
cc2report --date 2024-07-01 --output test.md
```

### API errors
```bash
# Test with minimal data
cc2report --date 2024-07-01 --model gpt-3.5-turbo

# Check API key
echo $OPENAI_API_KEY
```

### Performance issues
```bash
# Clear cache if corrupted
cc2report --clear-cache

# Use single thread
cc2report --parallel 1
```

## Advanced Tips

1. **Combine with other tools**:
   ```bash
   cc2report --format json | jq -r '.projects[] | "- \(.title): \(.objective)"'
   ```

2. **Create weekly digest**:
   ```bash
   cc2report --weekly --format json | \
     jq -r '.projects[] | .outcomes.delivered_value[]' | \
     sort | uniq
   ```

3. **Track token usage over time**:
   ```bash
   cc2report --show-token-usage --format json | \
     jq '.token_tracker.cost_estimate.total_cost'
   ```

4. **Generate reports for specific projects**:
   ```bash
   # Filter in post-processing
   cc2report --format json | \
     jq '.projects[] | select(.title | contains("backend"))'
   ```