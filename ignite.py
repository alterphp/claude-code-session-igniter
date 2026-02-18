#!/usr/bin/env python3
"""Start a Claude Code session to trigger the 5h rate-limit window."""

import sys
from datetime import datetime, timezone

import anyio
from claude_agent_sdk import query, ClaudeAgentOptions


async def main() -> int:
    ts = datetime.now(timezone.utc).isoformat()
    print(f"[{ts}] Starting Claude Code session...")

    try:
        async for message in query(
            prompt="hello",
            options=ClaudeAgentOptions(
                max_turns=1,
                extra_args={"no-session-persistence": None},
            ),
        ):
            pass  # drain the stream to ensure the session registers server-side
    except Exception as exc:
        print(f"[{ts}] FAILED: {exc}", file=sys.stderr)
        return 1

    ts = datetime.now(timezone.utc).isoformat()
    print(f"[{ts}] Session started successfully.")
    return 0


if __name__ == "__main__":
    sys.exit(anyio.run(main))
