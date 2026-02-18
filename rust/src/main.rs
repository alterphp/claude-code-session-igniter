use chrono::Utc;
use std::io::{BufRead, BufReader};
use std::process::{Command, ExitCode, Stdio};

fn main() -> ExitCode {
    let ts = Utc::now().to_rfc3339();
    println!("[{ts}] Starting Claude Code session...");

    let mut child = match Command::new("claude")
        .args(["-p", "--max-turns", "1", "--no-session-persistence", "hello"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(e) => {
            eprintln!("[{ts}] FAILED: could not spawn claude: {e}");
            return ExitCode::FAILURE;
        }
    };

    // Drain stdout so the session registers server-side.
    if let Some(stdout) = child.stdout.take() {
        for _ in BufReader::new(stdout).lines() {}
    }

    match child.wait() {
        Ok(status) if status.success() => {
            let ts = Utc::now().to_rfc3339();
            println!("[{ts}] Session started successfully.");
            ExitCode::SUCCESS
        }
        Ok(status) => {
            let ts = Utc::now().to_rfc3339();
            eprintln!("[{ts}] FAILED: claude exited with {status}");
            ExitCode::FAILURE
        }
        Err(e) => {
            let ts = Utc::now().to_rfc3339();
            eprintln!("[{ts}] FAILED: {e}");
            ExitCode::FAILURE
        }
    }
}
