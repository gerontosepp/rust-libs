# Proposal: `ai-compat` als erste gemeinsame Crate

## Why

Mehrere Projekte brauchen eine Anbindung an OpenAI-kompatible KI-Anbieter. Der Vinyl Tracker hat dafür bereits einen Client, der aber an seine Anwendungsschicht gebunden ist. Eine neutrale Crate vermeidet Duplikation und macht den Client zuerst im Tracker, später in der DJ-App nutzbar.

## What Changes

- Neuer Workspace `rust-libs` mit der Crate `crates/ai-compat`.
- OpenAI-kompatibler Chat-Client für Text, Bild-Input und Tool-Calls.
- Provider-Presets und Auth-Arten (`bearer`, `api-key-header`, `none`).
- Modellliste und minimaler Verbindungstest.
- Eigener `AiError` mit anbieterneutralem Mapping.
- Konfigurierbare Zusatz-Header/Attribution statt fest verdrahteter Projekt-URL.
- Optionale Base-URL-Validierung (HTTP/HTTPS, Metadata-/Link-Local-Block).
- Mock-Server-Tests, Crate-README, gemeinsame CI.
- Der Vinyl Tracker wird erster Konsument (separates Ticket im Tracker-Repo); die DJ-App folgt später.

**Nicht im Umfang:** Playlist-Logik, Bibliothekssuche, BPM-/Tonart-Validierung, Persistenz von Verbindungen, Verschlüsselung, UI, Streaming, Embeddings sowie jede Tracker-/DJ-Domänenlogik.

## Capabilities

### New Capabilities

- `ai-compat`: Domänenfreier OpenAI-kompatibler Client mit Provider-Presets, Tool-Calls, Modellliste, Verbindungstest und sicherer Fehlerbehandlung.

### Modified Capabilities

Keine.

## Impact

- **Neues Repo:** `gerontosepp/rust-libs`, Workspace mit `crates/*`.
- **Crate:** `crates/ai-compat` mit eigener README und Tests.
- **CI:** fmt, clippy und Tests über den gesamten Workspace.
- **Vinyl Tracker:** Folge-Ticket bindet die Crate per Git-Tag ein und ersetzt `backend/src/clients/ai.rs`.
- **DJ-App:** späterer Konsument; keine Änderung in diesem Change.
- **Keine** Datenbank, Migration, API oder UI in diesem Repo.
