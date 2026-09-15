# Roboco

Control your coding agents (Claude Code, Codex, Cursor, Devin, Grok, Hermes, Pi) locally by default, with optional multi-device sync.

*English | [简体中文](README.zh-CN.md)*

![Roboco driving a Claude Code session with a live branch diff sidebar](apps/landing/public/assets/app-screenshot.jpg)

Every device runs a small engine that stores sessions on that device. A new installation starts in local-only mode without an account or a network connection.

> **Roboco** is a hard fork of [zeronsh/zeron](https://github.com/zeronsh/zeron) (Zeron) with native Windows support. It tracks upstream and contributes back via PRs. Upstream's `zeron.sh` installer installs *Zeron* — for Roboco, build from source.

## Build from source

```bash
git clone https://github.com/hoangvu12/roboco
cd roboco
cargo run -p roboco
```

Windows development notes: [docs/reference/windows-development.md](docs/reference/windows-development.md). The daemon keeps running across reboots once installed. No sign-in or sync configuration is required.

The desktop sidebar browser also needs the [Linux browser runtime](docs/reference/linux-browser.md).

Day-to-day:

```bash
roboco status      # local/synced mode and engine status
roboco update      # update to the latest release
roboco daemon start|stop|restart|status
```

## Optional multi-device sync

Sign in only when you want to open your account's synced workspace. Authentication changes the profile selected by the next engine start, so stop the daemon before changing it:

```bash
roboco daemon stop
roboco login
roboco daemon start
```

You can then start an agent on one synced device and follow or drive it from another. An always-on machine such as a VPS can keep those agents working after you close your laptop.

Devices signed in to the same synced account are trusted with remote workspace access. A device controlling a workspace on another device can list, read, and write its files; enabling `Show ignored files` also makes gitignored files such as `.env` available remotely. `.git` is always excluded. Only sign in devices you trust with the full contents of your workspaces.

Signing in does not upload, move, or import existing local sessions. Local sessions and their attachments remain under the local profile and reappear when you return to local-only mode:

```bash
roboco daemon stop
roboco logout
roboco daemon start
```

`roboco login` and `roboco logout` refuse to modify credentials while an engine owns the data directory. The desktop app follows the same next-restart profile boundary.

On macOS: use the desktop release, or build `roboco` from source and run `roboco daemon install` to install the launchd service.

On Windows: extract the portable release ZIP and run `roboco.exe`. Keep `roboco-update.json` beside it for in-app updates. See the [development notes](docs/reference/windows-development.md) for source builds.

---

Developing or curious how it works? [![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/zeronsh/roboco) or check out [ARCHITECTURE.md](ARCHITECTURE.md).

Licensed under the [MIT License](LICENSE).
