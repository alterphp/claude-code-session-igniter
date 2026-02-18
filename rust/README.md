# Claude Code Session Igniter (Rust)

Start a Claude Code session programmatically to trigger the 5-hour rate-limit window at a fixed time every working day.

## Build

```bash
cargo build --release
```

The binary is at `target/release/claude-code-session-igniter`.

## Install on a remote server

### 1. Build locally and copy the binary

Cross-compile or build on the target architecture, then copy:

```bash
scp target/release/claude-code-session-igniter user@server:/usr/local/bin/
```

### 2. Install the Claude Code CLI on the server

```bash
npm install -g @anthropic-ai/claude-code
```

### 3. Authenticate

Run `claude` interactively once to complete the OAuth login flow. This stores credentials in `~/.claude/`.

```bash
claude
```

### 4. Test

```bash
claude-code-session-igniter
```

### 5. Schedule with cron

Run every weekday at 9:00 AM:

```cron
0 9 * * 1-5 /usr/local/bin/claude-code-session-igniter >> /var/log/claude-ignite.log 2>&1
```

### 6. Schedule with systemd timer (alternative)

Create `/etc/systemd/system/claude-ignite.service`:

```ini
[Unit]
Description=Claude Code Session Igniter

[Service]
Type=oneshot
User=your-user
ExecStart=/usr/local/bin/claude-code-session-igniter
```

Create `/etc/systemd/system/claude-ignite.timer`:

```ini
[Unit]
Description=Run Claude Code Session Igniter on weekdays at 9:00

[Timer]
OnCalendar=Mon..Fri 09:00
Persistent=true

[Install]
WantedBy=timers.target
```

Enable:

```bash
sudo systemctl daemon-reload
sudo systemctl enable --now claude-ignite.timer
```
