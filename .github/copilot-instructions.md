# Rentman.rs - AI Coding Agent Instructions

## Projektübersicht

Rentman.rs ist eine Rust-Bibliothek (crate) für die Interaktion mit der Rentman API. Rentman ist eine Rental-Management-Software für die Veranstaltungsbranche.

## Technologie-Stack

- **Sprache**: Rust 1.85+ (Edition 2024)
- **MSRV**: 1.85.0 (Minimum Supported Rust Version)
- **Ziel**: API-Client-Bibliothek (library crate)
- **API**: Rentman REST API (siehe [Rentman API Docs](https://api.rentman.net))

## Projektstruktur (zu etablieren)

```
rentman/
├── src/
│   ├── lib.rs          # Haupteinstiegspunkt, re-exports
│   ├── client.rs       # HTTP-Client mit Authentication
│   ├── models/         # API-Datenmodelle (serde Structs)
│   ├── endpoints/      # API-Endpunkt-Implementierungen
│   ├── error.rs        # Fehlertypen
│   └── prelude.rs      # Convenience re-exports
├── Cargo.toml          # Dependencies und Metadaten
├── examples/           # Verwendungsbeispiele
└── tests/              # Integration-Tests
```

## Entwicklungskonventionen

### Dependencies

Verwende diese Standard-Dependencies:

- `reqwest` - HTTP-Client mit async/await Support
- `serde` & `serde_json` - JSON-Serialisierung
- `tokio` - Async Runtime
- `thiserror` - Ergonomische Error-Definitionen
- `chrono` - Datum/Zeit-Handling

### API-Client Pattern

```rust
// Client mit Builder-Pattern und Token-Auth
let client = RentmanClient::builder()
    .token("API_TOKEN")
    .base_url("https://api.rentman.net") // optional
    .build()?;

// Resourcen als async Methoden
let projects = client.projects().list().await?;
```

### Error Handling

- Nutze `thiserror` für strukturierte Fehler
- Unterscheide: API-Errors, Network-Errors, Serialization-Errors
- Alle public APIs sollten `Result<T, RentmanError>` zurückgeben

### Testing

- Unit-Tests für Modell-Serialisierung
- Integration-Tests mit `wiremock` für API-Mocking
- Beispiele in `examples/` als Dokumentation und Smoke-Tests

## Wichtige Hinweise

- Rentman API verwendet Token-basierte Authentifizierung
- API-Responses sind paginated - implementiere `Iterator`-Pattern für große Resultsets
- Rate Limiting beachten - überlege Circuit Breaker oder Retry-Logic
- Alle API-Calls sind async

## Nächste Schritte

1. `Cargo.toml` mit Metadaten und Dependencies initialisieren
2. `src/lib.rs` und `src/client.rs` erstellen
3. Basis-Fehlertypen in `src/error.rs` definieren
4. Ein erstes Modell und Endpoint implementieren (z.B. Projects)
5. Integration-Tests mit wiremock aufsetzen
