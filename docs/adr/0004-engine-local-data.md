# Data is engine-local; there is no cross-engine sync

Every durable thing — sessions, transcripts, queue, search, settings, pairing credentials — belongs to exactly one engine (per-machine process) and lives in that engine's storage. Clients are views/controls of an engine, not replicas of it. There is no account tier and no cross-engine sync, ever; a client holding several engines just switches which one it drives. Consequence: features that were account-scoped in zeron (synced queue, account-wide search) become per-engine or die.
