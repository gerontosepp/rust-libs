# Tasks

## 1. Workspace & Crate-Scaffold

- [ ] 1.1 Workspace-Root `Cargo.toml`, `rust-toolchain.toml`, `.gitignore`, README und MIT-Lizenz anlegen; `cargo metadata` listet `crates/ai-compat`.
- [ ] 1.2 `crates/ai-compat` mit Cargo-Metadaten, `src/lib.rs` und Feature-Flags `rustls-tls`/`rustls-no-provider` anlegen; `cargo check --workspace` läuft grün.
- [ ] 1.3 GitHub-CI für fmt, clippy und `cargo test --workspace` einrichten; Workflow ist im Repo vorhanden.

## 2. Chat-Client

- [ ] 2.1 `ChatMessage`, `ToolCall`, `AssistantMessage`, `ChatRequest`/`ChatResponse` implementieren; Unit-Test prüft Serialisierung von Text, Bild-Data-URL und Tool-Calls.
- [ ] 2.2 `Client::chat` mit Base-URL-Auflösung, `max_tokens` und erster Choice implementieren; Mock-Server-Test prüft Request-Body und Antwort ohne `usage`.
- [ ] 2.3 Leere Choices und ungültiges JSON als `AiError::InvalidResponse` abbilden; Tests decken beide Fälle ab.

## 3. Presets, Auth & Header

- [ ] 3.1 Preset-Registry mit OpenRouter, OpenAI, Anthropic, Gemini, Groq, Mistral, DeepSeek, Together, Fireworks, Ollama, LM Studio, Azure OpenAI und Custom; Unit-Test prüft Pflichtfelder und unbekannte Presets.
- [ ] 3.2 Auth-Arten `bearer`, `api-key-header`, `none` und konfigurierbare Zusatz-Header implementieren; Mock-Server-Tests prüfen alle drei Arten und die Header.
- [ ] 3.3 Azure-Deployment-Pfad inkl. Query-String (`api-version`) auflösen; Test prüft Pfad und `api-key`-Header.
- [ ] 3.4 App-spezifische Attribution (`ClientOptions`) statt hartkodierter Projekt-URL; Test prüft, dass ohne Option keine Attribution gesendet wird.

## 4. Modellliste, Test & Fehler

- [ ] 4.1 `Client::list_models` mit `data`- und `models`-Antworten implementieren; Tests für sortierte/deduplizierte IDs und 404/405 → `supported = false`.
- [ ] 4.2 `Client::test` als minimalen Chat-Request implementieren; Tests für Erfolg, 401/403 und Timeout.
- [ ] 4.3 `AiError` mit Auth, NotFound, RateLimited, Timeout, Transport, InvalidResponse und Provider implementieren; Tests prüfen das Mapping für 401/403/404/422/429/500.
- [ ] 4.4 Sicherstellen, dass Keys/Authorization-Header nie in `Display`/`Debug`/Logs erscheinen; Test formatiert Fehler mit gesetztem Key und prüft die Ausgabe.

## 5. Base-URL-Validierung

- [ ] 5.1 `validate_base_url` mit `UrlPolicy` implementieren (HTTP/HTTPS, Userinfo, Metadata-/Link-Local-Block, HTTPS öffentlich, HTTP lokal/privat); Tests für alle Fälle inkl. `http://localhost:11434/v1`.
- [ ] 5.2 Optionale Host-Allowlist ergänzen; Test prüft erlaubte Subdomain und abgelehnten Fremdhost.

## 6. Doku & Qualitätsgate

- [ ] 6.1 Crate-README mit Beispielaufruf, Preset-Liste, Fehler-Mapping, TLS-Feature und URL-Validierung schreiben; Beispiel kompiliert als Doc-Test oder Test.
- [ ] 6.2 Workspace-README und `CHANGELOG.md` aktualisieren; Konsum-Beispiel mit gepinntem Git-Tag dokumentieren.
- [ ] 6.3 Qualitätsgate: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace`; alles grün.
- [ ] 6.4 OpenSpec-Change archivieren und PR mit `Closes #1` anlegen.

## Workflow follow-up

- Tag `v0.1.0` setzen, sobald der PR gemergt ist.
- Im Vinyl Tracker ein Folge-Ticket anlegen: Crate per Git-Tag einbinden, `backend/src/clients/ai.rs` ersetzen und `AiError` auf `AppError` mappen.
- Danach die DJ-App als zweiten Konsumenten anbinden.
