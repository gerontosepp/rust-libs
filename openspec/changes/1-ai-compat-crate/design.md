# Design: `ai-compat` im Workspace `rust-libs`

## Context

`rust-libs` ist ein neuer, neutraler Cargo-Workspace. Der Vinyl Tracker ist der erste Konsument, die DJ-/Party-App folgt später. Die Crate darf keine Domänen- oder Anwendungstypen enthalten; die fachliche Vorlage ist der Tracker-Client aus PR #394, nicht dessen Einbettung.

## Goals / Non-Goals

**Goals:**
- Stabile öffentliche API für Chat, Tool-Calls, Modellliste und Verbindungstest.
- Keine Kopplung an Datenbank, Axum, AppState, Crypto oder eine Anwendungsdomäne.
- Klare Fehler-Taxonomie und sichere Behandlung von Keys.
- Tests ohne echten Anbieter.
- Erster echter Konsument: Vinyl Tracker.

**Non-Goals:**
- Playlist-Logik, Retrieval, BPM-/Tonart-Validierung.
- Persistenz von Verbindungen oder Nutzerverwaltung.
- Streaming, Embeddings, Retries, Fallback-Ketten.
- UI.

## Decisions

### 1. Workspace mit austauschbaren Crates

`Cargo.toml` ist Workspace-Root (`members = ["crates/*"]`). Weitere Crates können später daneben liegen, ohne ein neues Repo zu brauchen.

*Alternative:* Einzelnes Crate-Repo – für weitere gemeinsame Crates zu eng.

### 2. Öffentliche API

```rust
pub struct Client { /* reqwest::Client */ }
pub struct ProviderConfig { base_url, auth_style, api_key, extra_headers }
pub struct ClientOptions { app_name, referer, timeout }
pub enum AuthStyle { Bearer, ApiKeyHeader, None }
pub struct AiPreset { kind, label, base_url, auth_style, requires_api_key }
pub struct ChatMessage { role, content, tool_calls, tool_call_id }
pub struct ChatRequest<'a> { model, messages, tools, max_tokens }
pub struct ChatResponse { message, usage }
pub enum AiError { Auth, NotFound, RateLimited, Timeout, Transport, InvalidResponse, Provider }
```

Kernmethoden: `Client::chat`, `Client::list_models`, `Client::test`. Presets über `presets()` und `preset(kind)`.

### 3. Fehler statt Anwendungsfehler

`AiError` nutzt `thiserror`. Konsumenten mappen ihn auf ihren eigenen Fehlertyp (Tracker: `AppError`). Anbieter-Bodies werden nur gekürzt geloggt; Keys und Authorization-Header nie.

### 4. TLS als Feature

Die Crate bietet `rustls-tls` (Default) und `rustls-no-provider` an. Der Tracker nutzt `rustls-no-provider` und installiert seinen `ring`-Provider selbst (ADR-0024); die DJ-App kann den Default verwenden.

### 5. App-spezifische Attribution

`ClientOptions` enthält App-Name und optionale Referer-URL. OpenRouter-Attribution wird nur gesetzt, wenn konfiguriert. Kein hartkodierter Tracker- oder DJ-Link.

### 6. URL-Validierung als optionale Funktion

`validate_base_url(url, policy)` ist eine reine Funktion mit `UrlPolicy { allowed_hosts }`. Die Anwendung entscheidet, ob sie sie nutzt.

### 7. Tests mit Mock-Server

Dev-Dependency `wiremock` prüft Request-Form, Auth-Header, Zusatz-Header, 401/403/404/405/429/Timeout und Modelllisten. Keine echten API-Keys.

### 8. Versionierung

Workspace-Version `0.1.0`; Tags `v0.1.0`. Konsumenten pinnen den Tag. Breaking Changes erhöhen die Version und stehen im `CHANGELOG.md`.

## Risks / Trade-offs

- **Zusätzliche Indirektion** → Der Tracker stellt in einem Folge-Ticket um; die Crate wird dort sofort real erprobt.
- **TLS-Feature-Matrix** → Tests laufen mit dem Default; die Tracker-Integration prüft `rustls-no-provider`.
- **Anbieter-Drift** → Presets sind Defaults, URLs überschreibbar; der Test-Endpunkt macht Fehler sichtbar.
- **Workspace wächst** → Neue Crates brauchen einen klaren, unabhängigen Zweck, sonst wird das Repo zum Sammelbecken.

## Migration Plan

1. Workspace und Crate implementieren, Tests und README fertigstellen.
2. Tag `v0.1.0` setzen.
3. Tracker-Folgeticket: Crate per Git-Tag einbinden, `clients/ai.rs` ersetzen, `AiError` auf `AppError` mappen.
4. DJ-App später als zweiter Konsument.
