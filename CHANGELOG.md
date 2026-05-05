# Changelog

All notable changes to this companion repo are recorded here. The format
follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and the repo
follows [Semantic Versioning](https://semver.org/) for the Rust crate
(`postgres-reports`); the SQL examples and Makefile entry points are
versioned together as a single repo.

## [Unreleased]

### Added
- `make verify` headline-count smoke test (film/actor/rental/language)
- `make demo` SQL walkthrough across `sql/01-fundamentals/` and `sql/02-joins/`
- `make capstone` entry point that runs `postgres-reports` against Pagila
- `make coverage` cargo-llvm-cov gate with a 100% line floor
- `crates/postgres-reports/` with `sqlx::PgPool`, clap CLI, and 13 named
  provable contracts asserted at runtime
- `contracts/postgres-reports.yaml` machine-readable contract index
- `docker-compose.yml` for `postgres:16` with a persistent `pgdata` volume
- `scripts/load-pagila.sh` idempotent loader

## [0.1.0] - 2026-05-05

Initial public release. Companion repo for the Coursera course
**Postgres From Zero**, sibling to `paiml/mysql-from-zero` and
`paiml/duckdb-from-zero`.
