# Remote access

Every engine keeps its local client connection. Remote access is off by default.
Enable **Settings → Remote access → Allow remote connections**, create a pairing
link, and paste it into **Settings → Devices** on another client. Links expire after five
minutes and work once. Paired sessions remain valid until revoked from Settings
or the CLI. Disabling remote access closes the remote listener and its connections.

## Headless engines

On an engine without saved remote settings:

```sh
roboco headless --network
```

The remote listener defaults to `0.0.0.0:27655`; local IPC remains on its own
loopback port. Startup prints a fresh pairing URL using the machine's LAN address.
You can choose a bind or advertise an operator-managed tunnel:

```sh
roboco headless --network --network-address 0.0.0.0:27655 --pairing-base-url https://my-engine.example
```

A base URL must be the server root: tunnel URLs carrying a path prefix (for
example `https://host.example/roboco`) are refused, because the engine serves
pairing, health, and WebSocket routes at the listener root.

`ROBOCO_NETWORK=true` is the headless environment equivalent. Accepted values are
`true`, `false`, `1`, and `0`. The desktop reads its saved settings. Startup flags
do not rewrite the Settings file.

**Conflicting choices keep the engine local.** If saved settings disable remote
access, `--network` cannot override that choice. Change the saved setting first.
Conflicts between the flag and environment also disable the remote bind. The
engine logs the configuration source and the reason for any refusal.

## Pairing administration

These commands work while the engine is running and use its normal data directory
(`ROBOCO_DATA_DIR` can select another engine installation):

```sh
roboco engine pairing create --base-url http://192.168.1.20:27655 --label Laptop
roboco engine pairing list
roboco engine pairing revoke SESSION_ID
```

Create and list accept `--json`. Create accepts `--ttl-seconds` from 1 to 3600.
Only create prints a secret; list shows session identifiers, labels, last-seen
timestamps, and revocation state. Revocation rejects the session's next connection.

## Saved settings

`remote-access.json` lives beside the engine's other settings:

```json
{
  "enabled": true,
  "bindAddress": "0.0.0.0:27655",
  "publicUrl": null
}
```

The Settings toggle takes effect immediately. Manual file edits are read at the
next startup or Settings change. Invalid files keep remote access off.

The engine serves plain HTTP/WebSocket on trusted networks. For internet access,
use your own TLS tunnel. Pairing links carry the code in a URL fragment; the
native client submits it in the Authorization header, never in a request URL.

## Using several engines

Paired engines reconnect automatically when the desktop starts. Their chats and
spaces appear in the existing sidebar with device labels. Pick a device in the
space palette, then choose a folder; the composer space chip identifies where the
new chat will run. You can also enter an absolute path for that engine. An existing
folder opens on Enter; a missing folder offers **Create and add**.

Settings → Devices shows connection state and lets you forget a paired engine.
Forgetting removes its saved connection and client cache. Revoke the session on
the engine when you also want to invalidate its credential.

When an engine is unreachable, cached chat lists and previously opened transcripts
remain readable. A compact strip marks the offline view, and sending is refused.
The client retries with capped backoff and restores live updates after reconnecting.
Each engine owns its files, terminals, queue and history; pairing does not copy
engine data between machines.
