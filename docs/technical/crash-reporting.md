# Crash Reporting

Automatic crash report generation and submission.

## Overview

When kjxlkj crashes, it generates a report to help
diagnose and fix the issue.

## Crash Report Contents

### Included

- Backtrace
- kjxlkj version
- OS and terminal info
- Recent actions (anonymized)
- Configuration (sanitized)

### Excluded

- File contents
- Personal information
- Credentials
- Private paths (obfuscated)

## Report Location


## Report Format


## Automatic Submission

### Opt-In Only


### Submission


## Privacy

### Sanitization


### No File Contents

File contents are never included.

## Manual Submission

### GitHub Issues

1. Open crash report file
2. Create GitHub issue
3. Paste report (review first)
4. Add reproduction steps

### Template

Create a GitHub issue with the title "Crash Report" containing:

1. **Reproduction Steps** - Numbered list of actions that caused the crash
2. **Report** - Paste the crash report in a collapsible details block
3. **Session Recovery** - Note if session was restored

Previous session crashed.
Restore session? [y/n]
