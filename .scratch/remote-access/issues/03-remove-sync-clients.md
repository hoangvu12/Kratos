# 03 — Remove sync room clients; state feeds go engine-local

**What to build:** The sync-era machinery that pushed sessions/devices through the cloud is removed: the chat/registry room clients, the device-room relay client, and their tests. Client state (devices, spaces, `@ device` tags, session lists) now comes solely from the local engine's RPCs. The app behaves as a single-engine client, fully working locally.

**Blocked by:** None — can start immediately.

**Status:** ready-for-agent

**Parent:** `.scratch/remote-access/spec.md`

- [ ] Room/relay sync clients and their integration tests are deleted; local loro doc storage stays
- [ ] Sidebar, spaces, and device vocabulary render correctly from local-engine data alone
- [ ] No sign-in or cloud path is required for any local behavior
- [ ] Headless UI layout regression suite stays green
