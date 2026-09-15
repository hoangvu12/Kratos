# Domain context

## Remote access

**Engine**:
The per-device process that owns sessions, transcripts, and agent harnesses, and serves them to clients over a WebSocket. One engine per machine.
_Avoid_: Environment, server, backend, daemon (when it means the engine generally)

**Pairing**:
The one-time act of registering a client with an engine by redeeming a short-lived pair code or pairing URL. After pairing, the client holds a session and never pairs again.
_Avoid_: Login, sign-in, connecting (when it means pairing)

**Session**:
The credential a paired client holds for an engine. Does not expire; lives until revoked.
_Avoid_: Token (when a human-facing word is wanted), login

## Theme vocabulary

- **Theme family** — A named collection of related theme variants that share an origin, such as Night Owl and Night Owl Light.
- **Theme variant** — One complete, resolved palette for a single appearance (`light` or `dark`). Runtime UI consumes variants, not source-format tokens.
- **Theme source** — The durable origin of a custom family: an imported snapshot, linked file, linked package, or editable native file.
- **Imported snapshot** — A self-contained copy of a compiled theme family. It no longer follows changes to its original source.
- **Linked theme** — A custom family that follows a source on disk and can be reloaded without re-importing it.
- **Linked file** — A link to one VS Code-compatible theme definition.
- **Linked package** — A link to a VS Code extension folder or `package.json` that declares one or more related theme variants.
- **Editable theme** — A duplicate stored as a native resolved-family JSON file. Users edit the file directly and explicitly reload it; invalid edits preserve the last known good family.
- **Last known good** — The most recent successfully compiled family retained by a linked theme when its current source is missing or invalid.
- **Import report** — A per-variant summary of mapped roles, fallbacks, unsupported values, and inferred decisions produced during compilation.
- **Mapping review** — The optional advanced view of an import report. Normal theme selection and import do not expose token names.
- **Theme hardening** — Deterministic post-mapping repairs that prefer stronger semantically related source colors, minimally adjust only shared Roboco roles when needed, and record every decision in the mapping review.
- **Theme default accent** — The interaction accent chosen or inferred for a theme variant by its authoring source.
- **Accent override** — A Roboco preset that replaces interaction roles only; syntax, terminal ANSI, diff, warning, error, and success colors remain owned by the theme.
- **Recommended surface treatment** — A theme variant's authored recommendation for whether its surfaces are frosted or opaque. It is used only when the user keeps the surface preference at theme default.
- **Surface preference** — A device-local choice of theme default, frosted, or opaque that is independent of appearance, theme, and accent selections.
- **Resolved surface treatment** — The effective frosted or opaque treatment produced by applying the surface preference to the active variant's recommendation.
