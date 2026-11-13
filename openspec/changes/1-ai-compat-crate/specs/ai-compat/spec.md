# Spec Delta

## Purpose

Stellt eine domänenfreie, wiederverwendbare Anbindung an OpenAI-kompatible KI-Anbieter für Chat, Tool-Calls, Modelllisten und Verbindungstests bereit.

## ADDED Requirements

### Requirement: Chat-Completions

Die Crate MUST Chat-Completions gegen einen konfigurierbaren OpenAI-kompatiblen Endpunkt senden und die erste Assistant-Nachricht zurückgeben. Sie MUST Text-, Bild- und Tool-Call-Nachrichten unterstützen, ein Modell sowie `max_tokens` mitsenden und Anbieter-Responses ohne `usage` akzeptieren.

#### Scenario: Text- und Tool-Call
- **WHEN** ein Chat mit einer System- und User-Nachricht sowie Tool-Definitionen gesendet wird
- **THEN** enthält die Antwort die Assistant-Nachricht mit Inhalt oder Tool-Calls

#### Scenario: Bild-Input
- **WHEN** eine User-Nachricht ein JPEG als Data-URL enthält
- **THEN** wird das Bild im OpenAI-kompatiblen `image_url`-Format übertragen

#### Scenario: Keine Choices
- **WHEN** der Anbieter mit Erfolg antwortet, aber keine Choices liefert
- **THEN** liefert die Crate einen Fehler

### Requirement: Provider-Presets und Authentifizierung

Die Crate MUST Presets für OpenRouter, OpenAI, Anthropic, Google Gemini, Groq, Mistral, DeepSeek, Together, Fireworks, Ollama, LM Studio, Azure OpenAI und Custom bereitstellen. Ein Preset MUST Standard-Base-URL, Auth-Art und Key-Pflicht liefern. Die Auth-Arten `bearer`, `api-key-header` und `none` MÜSSEN unterstützt werden; Zusatz-Header MÜSSEN konfigurierbar sein.

#### Scenario: Bearer
- **WHEN** eine Verbindung die Auth-Art `bearer` verwendet
- **THEN** sendet die Crate den Key als `Authorization: Bearer …`

#### Scenario: Azure
- **WHEN** eine Azure-OpenAI-Verbindung genutzt wird
- **THEN** sendet die Crate den Key im `api-key`-Header und spricht den konfigurierten Deployment-Pfad an

#### Scenario: Lokal ohne Key
- **WHEN** eine Ollama- oder LM-Studio-Verbindung ohne Key genutzt wird
- **THEN** wird keine Authentifizierung gesendet

### Requirement: Modellliste

Die Crate MUST die Modell-IDs eines OpenAI-kompatiblen `/models`-Endpunkts laden, sofern der Anbieter ihn anbietet. Bei 404 oder 405 MUST sie „nicht unterstützt“ melden, ohne einen Fehler zu erzeugen.

#### Scenario: Modellliste verfügbar
- **WHEN** der Anbieter `/models` mit `data: [{ id }]` beantwortet
- **THEN** liefert die Crate die sortierten, eindeutigen Modell-IDs

#### Scenario: Keine Modellliste
- **WHEN** der Anbieter `/models` mit 404 oder 405 beantwortet
- **THEN** meldet die Crate `supported = false`

### Requirement: Verbindungstest

Die Crate MUST einen minimalen Chat-Request als Verbindungstest anbieten. Der Test MUST Endpoint, Authentifizierung und Modell prüfen und bei Erfolg ohne Fehler zurückkehren.

#### Scenario: Erfolgreicher Test
- **WHEN** Endpoint, Key und Modell gültig sind
- **THEN** kehrt der Test erfolgreich zurück

#### Scenario: Ungültiger Key
- **WHEN** der Anbieter mit 401 oder 403 antwortet
- **THEN** liefert die Crate einen Auth-Fehler

### Requirement: Fehler und Secrets

Die Crate MUST einen eigenen Fehlertyp mit unterscheidbaren Fällen für Authentifizierung, fehlendes Modell/Endpoint, Rate-Limit, Timeout, Transport und unerwartete Antworten liefern. Keys und Authorization-Header DÜRFEN NICHT in Fehlermeldungen oder Logs erscheinen. Anbieter-Fehlertexte DÜRFEN nur gekürzt geloggt werden.

#### Scenario: Rate-Limit
- **WHEN** der Anbieter mit 429 antwortet
- **THEN** liefert die Crate einen Rate-Limit-Fehler

#### Scenario: Timeout
- **WHEN** die Anfrage in ein Timeout läuft
- **THEN** liefert die Crate einen Timeout-Fehler

#### Scenario: Key im Fehler
- **WHEN** ein Fehler formatiert oder geloggt wird
- **THEN** enthält die Ausgabe weder den Key noch den Authorization-Header

### Requirement: Base-URL-Sicherheit

Die Crate MUST eine optionale Validierung von Base-URLs anbieten: nur HTTP/HTTPS, keine Userinfo, Metadata-/Link-Local-Ziele blockiert, HTTPS für öffentliche Hosts und HTTP nur für Loopback/private Netze.

#### Scenario: Metadata-Ziel
- **WHEN** eine Base-URL auf ein Link-Local- oder Metadata-Ziel zeigt
- **THEN** wird sie abgelehnt

#### Scenario: Lokaler HTTP-Endpunkt
- **WHEN** eine Base-URL `http://localhost:11434/v1` lautet
- **THEN** wird sie akzeptiert

### Requirement: Domänenfreiheit

Die Crate MUST keine Typen oder Abhängigkeiten aus der Vinyl-Tracker- oder DJ-Domäne enthalten. Insbesondere DÜRFEN keine `vinyl-*`-Crates, Datenbank-, Axum-, AppState-, Crypto- oder Playlist-Typen importiert werden.

#### Scenario: Abhängigkeitsprüfung
- **WHEN** die Crate gebaut wird
- **THEN** enthält ihr Dependency-Baum weder `vinyl-api-types` noch SQLx, Axum oder Tracker-/DJ-spezifische Crates
