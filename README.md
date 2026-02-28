# Flow

A CLI tool for developer context preservation. Never lose your train of thought again.

## What it does

Flow helps developers preserve their mental context when switching tasks or getting interrupted.

**Problem**: You're deep in code, everything's in your head. Then: Slack notification. Meeting. You come back 30 minutes later and think: "What was I doing?"

**Solution**: Capture your context before the interruption, resume instantly after.

## Installation

```bash
cargo install flow
```

## Usage

### Save what you're working on

```bash
flow note "Debugging auth issue in login.js - checking JWT refresh token"
```

### Check current context

```bash
flow status
```

Output:
```
📍 Current Context
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Project: my-app (branch: fix/jwt-refresh)

💭 "Debugging auth issue in login.js..."
   Last updated: 32 minutes ago

💡 Resume with: flow resume
```

### Get help resuming

```bash
flow resume
```

### Mark complete and start fresh

```bash
flow done
```

## Features

- ✅ **Git-aware**: Automatically detects your repo and branch
- ✅ **Project-specific**: Different contexts per project (coming soon)
- ✅ **Fast**: < 50ms for all operations
- ✅ **Offline**: Works without network
- ✅ **Simple**: JSON storage, human-readable

## Roadmap

- [ ] Project-specific contexts
- [ ] Context history tracking
- [ ] Team context sharing
- [ ] Integration with git hooks
- [ ] Time tracking

## Development

Built with ❤️ by onoht

### Tech Stack

- Rust 🦀
- clap (CLI parsing)
- serde (JSON)
- git2 (Git integration)
- chrono (timestamps)

## License

MIT
