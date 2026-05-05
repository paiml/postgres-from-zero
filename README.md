# Postgres From Zero — Companion Repo

The runnable companion to the Coursera course **Postgres From Zero**, part of
the *Rust for Data Engineering* specialization.

This repo bundles every example shown in the videos plus the optional Rust
capstone artifact (`postgres-reports`). The Coursera lab boots a sandbox with
Pagila pre-loaded — but you can also run everything locally.

## Installation

Prerequisites:

- Docker 20.10+ with Compose v2 (the `docker compose` plugin, not the old
  `docker-compose` binary)
- Rust 1.85+ via rustup (`rust-toolchain.toml` pins to 1.95 for parity with
  the sibling `mysql-from-zero` and `duckdb-from-zero` repos)
- ~500 MB free disk for the Postgres image and Pagila volume

Clone and bootstrap:

    git clone https://github.com/paiml/postgres-from-zero
    cd postgres-from-zero
    cp .env.example .env       # only needed if a tool reads .env directly
    make up                    # boots postgres in docker
    make pagila                # idempotently loads Pagila (~10s)

The first `make up` pulls `postgres:16` if not cached (~150 MB). Subsequent
runs use the cached image and the persisted volume.

## Usage

The Makefile is the canonical entry point — `make help` lists every target.
Headline commands:

    make demo        # run the SQL example walkthrough against pagila
    make capstone    # build and run the Rust postgres-reports binary
    make verify      # assert all headline counts (CI smoke test)
    make psql        # drop into a psql shell against pagila
    make test        # cargo test for the Rust crate
    make coverage    # cargo llvm-cov for the Rust crate (100% gate)
    make fmt lint    # cargo fmt && cargo clippy

The Rust binary takes three flags:

    cargo run --release -- --report customers --limit 5
    cargo run --release -- --report films     --limit 5 --out films.json
    cargo run --release -- --report actors    --limit 5

## What's here

- `sql/01-fundamentals/` — Module 1 examples: connect, list tables, modify, export
- `sql/02-joins/` — Module 2 examples: traversal, INNER vs LEFT, EXPLAIN ANALYZE
- `sql/pagila-analytics/` — the queries the Rust capstone runs
- `crates/postgres-reports/` — sqlx::PgPool + clap binary that runs three reports
  with runtime contracts; the foundation for the course capstone
- `scripts/` — bash glue for load, demo, and verify

## Why Postgres

If you have done the MySQL course in this specialization, the same
`customer → rental → inventory → film` traversal works here unchanged — Pagila
is the Postgres community port of MySQL's Sakila. The deltas are:

- `\d film` instead of `DESCRIBE film`
- `EXPLAIN (ANALYZE, BUFFERS)` instead of MySQL's `EXPLAIN ANALYZE` — richer
  output, more readable plan tree
- `COPY ... TO STDOUT` instead of `INTO OUTFILE` — same shape, fewer surprises
- `pg_stat_statements`, `CREATE INDEX CONCURRENTLY`, range types, `JSONB`,
  `LATERAL` — features that have no MySQL equivalent (covered in the videos)

## Provable contracts

`postgres-reports` proves a named runtime contract for every Top-N query and
every CLI invocation. The library aborts if the database disagrees with its
stated invariants instead of silently shipping malformed JSON downstream.

| Contract                      | Where it's proved                                      |
|-------------------------------|--------------------------------------------------------|
| `customers-row-count`         | `len(rows) == --limit`                                 |
| `customers-top-nonzero`       | leading `rental_count >= 1`                            |
| `customers-monotonic`         | `ORDER BY rental_count DESC` preserved row-to-row      |
| `customers-row-shape`         | `customer_id > 0`, `name` is `'First Last'`            |
| `films-row-count`             | `len(rows) == --limit`                                 |
| `films-top-nonzero`           | leading `rental_count >= 1`                            |
| `films-monotonic`             | `ORDER BY rental_count DESC` preserved                 |
| `films-row-shape`             | `film_id > 0`, `title` non-empty                       |
| `actors-row-count`            | `len(rows) == --limit`                                 |
| `actors-top-nonzero`          | leading `film_count >= 1`                              |
| `actors-monotonic`            | `ORDER BY film_count DESC` preserved                   |
| `actors-row-shape`            | `actor_id > 0`, `first_name`/`last_name` non-empty     |
| `run-output`                  | rendered JSON parses and is an array                   |

Each successful proof emits `contract: <name> OK` on stderr so a screencast
or CI run can show the contract held. `make capstone` exits non-zero if any
contract fires.

## Capstone

The capstone for this course extends `crates/postgres-reports/` from three
hardcoded reports to a configurable analytics tool with compile-time SQL
verification via `sqlx::query_as!`. See the course's capstone reading for
the full brief.

## Contributing

This repo is a companion artifact for the Coursera course. Substantive
content changes flow through the course's authoring workflow, not GitHub
PRs. Bug reports and reproductions of broken-on-clean-clone issues are
welcome — open an issue on
[paiml/postgres-from-zero](https://github.com/paiml/postgres-from-zero/issues)
with the output of `make verify` and your platform.

Local sanity checklist before submitting an issue:

    make nuke && make up && make pagila    # full reset
    make verify                            # all four counts pass?
    make test                              # 38/38 tests pass?
    make lint                              # clippy clean?

## License

MIT. See the `license` field in `crates/postgres-reports/Cargo.toml`. The
SQL example files are derived from the Pagila project; refer to
[devrimgunduz/pagila](https://github.com/devrimgunduz/pagila) for its terms.
