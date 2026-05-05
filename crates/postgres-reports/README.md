# postgres-reports

Three Sakila/Pagila analytics reports built on `sqlx::PgPool`.
The crate's public API is in `src/lib.rs`; `main.rs` is a thin clap wrapper.

## Build & run

From repo root:

    make up && make pagila
    cargo run --release -- --report customers --limit 5
    cargo run --release -- --report films     --limit 5
    cargo run --release -- --report actors    --limit 5

## Test

    cargo test --release    # requires Pagila loaded; reads $DATABASE_URL
