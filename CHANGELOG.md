# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial project structure
- Core HTTP client with JWT token authentication
- API models for:
  - Projects and Subprojects
  - Contacts and Contact Persons
  - Equipment and Serial Numbers
  - Crew Members and Availability
  - Appointments
- Endpoint implementations for:
  - Projects (list, get, create, update, delete)
  - Contacts (list, get)
  - Equipment (list, get)
- Query parameter support for filtering and pagination
- Comprehensive error handling with structured error types
- Examples:
  - List projects
  - Create project
  - List contacts
  - List equipment
- Integration tests
- Full documentation in README.md

### Security
- API token authentication via JWT Bearer tokens
- Gitignore configured to prevent token leakage

## [0.1.0] - 2026-01-22

### Added
- Initial release of rentman.rs
- Basic CRUD operations for core resources
- Async/await support with tokio
- Type-safe API with full serde serialization

[Unreleased]: https://github.com/joschawagner/Rentman.rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/joschawagner/Rentman.rs/releases/tag/v0.1.0
