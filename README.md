# Claude Code Session Igniter

Start a Claude Code session programmatically to trigger the 5-hour rate-limit window at a fixed time every working day.

## Prerequisites

- Node.js (for the Claude Code CLI)
- Python 3.10+

## Setup

### 1. Install the Claude Code CLI

```bash
npm install -g @anthropic-ai/claude-code
```

### 2. Authenticate

Run `claude` interactively once and complete the OAuth login flow. This stores your credentials in `~/.claude/`.

```bash
claude
```

### 3. Install Python dependencies

```bash
pip install -r requirements.txt
```

## Usage

```bash
python ignite.py
```

The script sends "hello" to Claude Code, waits for the response, and exits. No session files are persisted on disk.

## Scheduling (cron)

To run every weekday at 9:00 AM:

```cron
0 9 * * 1-5 cd /path/to/claude-code-session-igniter && python ignite.py >> /var/log/claude-ignite.log 2>&1
```
