//! # postgres-reports
//!
//! Three Sakila/Pagila analytics reports built on `sqlx::PgPool`.
//!
//! ## Provable contracts
//!
//! Every public report function in this crate proves a runtime contract
//! before returning. If the database disagrees with the schema's stated
//! invariants, the process aborts loudly instead of silently shipping
//! corrupt JSON downstream.
//!
//! | Contract                          | Where it's proved          |
//! |-----------------------------------|----------------------------|
//! | `row-count == limit`              | [`top_customers`], [`top_films`], [`top_actors`] |
//! | `top row count >= 1`              | same                       |
//! | `ORDER BY DESC monotonic`         | same                       |
//! | `customer_id > 0` + `name has space` | [`top_customers`]       |
//! | `film_id > 0` + `title non-empty` | [`top_films`]              |
//! | `actor_id > 0` + first/last non-empty | [`top_actors`]         |
//!
//! Each successful proof emits `contract: <name> OK` on stderr so a
//! screencast or CI run can show the contract held.

use anyhow::{Context, Result};
use serde::Serialize;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};
use std::path::{Path, PathBuf};

/// Which Top-N report to render. Used by the CLI and reachable as a library
/// type so `run` can be exercised without spawning the binary.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Report {
    Customers,
    Films,
    Actors,
}

/// Inputs to a single CLI invocation. Mirrors the clap-derived struct in
/// `main.rs` but is decoupled from clap so library tests don't need to
/// construct `clap::Parser` state.
#[derive(Clone, Debug)]
pub struct RunOptions {
    pub report: Report,
    pub limit: i64,
    pub out: Option<PathBuf>,
    pub database_url: String,
}

/// Run a single report against `pool`, return the rendered JSON, and
/// optionally persist it to `out`. The caller is responsible for printing
/// the JSON to stdout.
///
/// Provable contract: the JSON returned is well-formed (parseable as a
/// `serde_json::Value` array). Asserted on every call.
///
/// # Example
///
/// ```no_run
/// # async fn doctest() -> anyhow::Result<()> {
/// use postgres_reports::{pool, run, Report, RunOptions};
/// let opts = RunOptions {
///     report: Report::Customers,
///     limit: 5,
///     out: None,
///     database_url: "postgres://postgres:postgres@localhost:5432/pagila".into(),
/// };
/// let pool = pool(&opts.database_url).await?;
/// let json = run(&pool, &opts).await?;
/// assert!(json.starts_with('['));
/// # Ok(()) }
/// ```
pub async fn run(pool: &PgPool, opts: &RunOptions) -> Result<String> {
    let json = match opts.report {
        Report::Customers => serde_json::to_string_pretty(&top_customers(pool, opts.limit).await?)?,
        Report::Films => serde_json::to_string_pretty(&top_films(pool, opts.limit).await?)?,
        Report::Actors => serde_json::to_string_pretty(&top_actors(pool, opts.limit).await?)?,
    };
    assert_well_formed_json(&json);
    if let Some(path) = opts.out.as_deref() {
        write_to_path(path, &json)?;
    }
    Ok(json)
}

fn write_to_path(path: &Path, body: &str) -> Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("failed to create directory {parent:?}"))?;
    }
    std::fs::write(path, body).with_context(|| format!("failed to write {path:?}"))?;
    Ok(())
}

fn assert_well_formed_json(body: &str) {
    let parsed: serde_json::Value =
        serde_json::from_str(body).expect("run-output: rendered JSON must parse");
    assert!(parsed.is_array(), "run-output: rendered JSON must be an array");
    eprintln!("contract: run-output OK");
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopCustomer {
    pub customer_id: i32,
    pub name: String,
    pub rental_count: i64,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopFilm {
    pub film_id: i32,
    pub title: String,
    pub rental_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopActor {
    pub actor_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub film_count: i64,
}

/// Open a tokio-friendly Postgres connection pool, capped at 4 connections.
///
/// Provable contract: returns an error (not a panic) for unreachable URLs.
///
/// # Example
///
/// ```no_run
/// # async fn doctest() -> anyhow::Result<()> {
/// use postgres_reports::pool;
/// let pool = pool("postgres://postgres:postgres@localhost:5432/pagila").await?;
/// drop(pool);
/// # Ok(()) }
/// ```
pub async fn pool(database_url: &str) -> Result<PgPool> {
    PgPoolOptions::new()
        .max_connections(4)
        .connect(database_url)
        .await
        .with_context(|| format!("failed to connect to {database_url}"))
}

const SQL_TOP_CUSTOMERS: &str = "\
    SELECT c.customer_id, \
           (c.first_name || ' ' || c.last_name) AS name, \
           COUNT(r.rental_id)::BIGINT          AS rental_count, \
           c.email \
    FROM customer c \
    LEFT JOIN rental r ON r.customer_id = c.customer_id \
    GROUP BY c.customer_id, c.first_name, c.last_name, c.email \
    ORDER BY rental_count DESC \
    LIMIT $1";

const SQL_TOP_FILMS: &str = "\
    SELECT f.film_id, f.title, COUNT(r.rental_id)::BIGINT AS rental_count \
    FROM film f \
    LEFT JOIN inventory i ON i.film_id = f.film_id \
    LEFT JOIN rental r    ON r.inventory_id = i.inventory_id \
    GROUP BY f.film_id, f.title \
    ORDER BY rental_count DESC \
    LIMIT $1";

const SQL_TOP_ACTORS: &str = "\
    SELECT a.actor_id, a.first_name, a.last_name, \
           COUNT(DISTINCT fa.film_id)::BIGINT AS film_count \
    FROM actor a \
    LEFT JOIN film_actor fa ON fa.actor_id = a.actor_id \
    GROUP BY a.actor_id, a.first_name, a.last_name \
    ORDER BY film_count DESC \
    LIMIT $1";

/// Top-N customers by rental count.
///
/// Provable contracts: `customers-row-count`, `customers-top-nonzero`,
/// `customers-monotonic`, `customers-row-shape`.
///
/// # Example
///
/// ```no_run
/// # async fn doctest() -> anyhow::Result<()> {
/// use postgres_reports::{pool, top_customers};
/// let pool = pool("postgres://postgres:postgres@localhost:5432/pagila").await?;
/// let rows = top_customers(&pool, 5).await?;
/// assert_eq!(rows.len(), 5);
/// # Ok(()) }
/// ```
pub async fn top_customers(pool: &PgPool, limit: i64) -> Result<Vec<TopCustomer>> {
    let rows: Vec<TopCustomer> = sqlx::query_as(SQL_TOP_CUSTOMERS)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    assert_contracts_customers(&rows, limit as usize);
    Ok(rows)
}

/// Top-N films by rental count.
///
/// Provable contracts: `films-row-count`, `films-top-nonzero`,
/// `films-monotonic`, `films-row-shape`.
///
/// # Example
///
/// ```no_run
/// # async fn doctest() -> anyhow::Result<()> {
/// use postgres_reports::{pool, top_films};
/// let pool = pool("postgres://postgres:postgres@localhost:5432/pagila").await?;
/// let rows = top_films(&pool, 3).await?;
/// assert!(rows.iter().all(|r| !r.title.is_empty()));
/// # Ok(()) }
/// ```
pub async fn top_films(pool: &PgPool, limit: i64) -> Result<Vec<TopFilm>> {
    let rows: Vec<TopFilm> = sqlx::query_as(SQL_TOP_FILMS)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    assert_contracts_films(&rows, limit as usize);
    Ok(rows)
}

/// Top-N actors by distinct film count.
///
/// Provable contracts: `actors-row-count`, `actors-top-nonzero`,
/// `actors-monotonic`, `actors-row-shape`.
///
/// # Example
///
/// ```no_run
/// # async fn doctest() -> anyhow::Result<()> {
/// use postgres_reports::{pool, top_actors};
/// let pool = pool("postgres://postgres:postgres@localhost:5432/pagila").await?;
/// let rows = top_actors(&pool, 2).await?;
/// assert!(rows[0].film_count >= rows[1].film_count);
/// # Ok(()) }
/// ```
pub async fn top_actors(pool: &PgPool, limit: i64) -> Result<Vec<TopActor>> {
    let rows: Vec<TopActor> = sqlx::query_as(SQL_TOP_ACTORS)
        .bind(limit)
        .fetch_all(pool)
        .await?;
    assert_contracts_actors(&rows, limit as usize);
    Ok(rows)
}

/// Shared invariants that hold for every Top-N report regardless of row type:
/// the row count matches the requested limit, the leading row's metric is
/// non-trivial, and the metric is monotonically non-increasing.
fn assert_topn_invariants<T>(rows: &[T], limit: usize, label: &str, metric: impl Fn(&T) -> i64) {
    assert_eq!(rows.len(), limit, "{label}-row-count: expected {limit}, got {}", rows.len());
    eprintln!("contract: {label}-row-count OK");

    assert!(metric(&rows[0]) >= 1, "{label}-top-nonzero: leading row metric must be >= 1");
    eprintln!("contract: {label}-top-nonzero OK");

    for w in rows.windows(2) {
        assert!(metric(&w[0]) >= metric(&w[1]), "{label}-monotonic: ORDER BY DESC violated");
    }
    eprintln!("contract: {label}-monotonic OK");
}

fn assert_contracts_customers(rows: &[TopCustomer], limit: usize) {
    assert_topn_invariants(rows, limit, "customers", |r| r.rental_count);
    for r in rows {
        assert!(r.customer_id > 0, "customers-row-shape: customer_id must be positive (got {})", r.customer_id);
        assert!(r.name.contains(' '), "customers-row-shape: name must be 'First Last' (got {:?})", r.name);
    }
    eprintln!("contract: customers-row-shape OK");
}

fn assert_contracts_films(rows: &[TopFilm], limit: usize) {
    assert_topn_invariants(rows, limit, "films", |r| r.rental_count);
    for r in rows {
        assert!(r.film_id > 0, "films-row-shape: film_id must be positive (got {})", r.film_id);
        assert!(!r.title.is_empty(), "films-row-shape: title must be non-empty");
    }
    eprintln!("contract: films-row-shape OK");
}

fn assert_contracts_actors(rows: &[TopActor], limit: usize) {
    assert_topn_invariants(rows, limit, "actors", |r| r.film_count);
    for r in rows {
        assert!(r.actor_id > 0, "actors-row-shape: actor_id must be positive (got {})", r.actor_id);
        assert!(!r.first_name.is_empty() && !r.last_name.is_empty(),
                "actors-row-shape: first/last name must be non-empty");
    }
    eprintln!("contract: actors-row-shape OK");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn customer(id: i32, name: &str, count: i64) -> TopCustomer {
        TopCustomer { customer_id: id, name: name.into(), rental_count: count, email: None }
    }
    fn film(id: i32, title: &str, count: i64) -> TopFilm {
        TopFilm { film_id: id, title: title.into(), rental_count: count }
    }
    fn actor(id: i32, first: &str, last: &str, count: i64) -> TopActor {
        TopActor { actor_id: id, first_name: first.into(), last_name: last.into(), film_count: count }
    }

    #[test]
    fn customers_contracts_pass_on_well_formed_rows() {
        let rows = vec![customer(1, "ALPHA BETA", 5), customer(2, "GAMMA DELTA", 3)];
        assert_contracts_customers(&rows, 2);
    }

    #[test]
    #[should_panic(expected = "customers-row-count")]
    fn customers_contracts_fail_on_wrong_length() {
        assert_contracts_customers(&[customer(1, "A B", 5)], 2);
    }

    #[test]
    #[should_panic(expected = "customers-top-nonzero")]
    fn customers_contracts_fail_on_zero_top() {
        assert_contracts_customers(&[customer(1, "A B", 0)], 1);
    }

    #[test]
    #[should_panic(expected = "customers-monotonic")]
    fn customers_contracts_fail_on_disordered_rows() {
        let rows = vec![customer(1, "A B", 3), customer(2, "C D", 5)];
        assert_contracts_customers(&rows, 2);
    }

    #[test]
    #[should_panic(expected = "customers-row-shape")]
    fn customers_contracts_fail_on_nonpositive_id() {
        assert_contracts_customers(&[customer(0, "A B", 5)], 1);
    }

    #[test]
    #[should_panic(expected = "customers-row-shape")]
    fn customers_contracts_fail_on_unsplit_name() {
        assert_contracts_customers(&[customer(1, "ALPHA", 5)], 1);
    }

    #[test]
    fn films_contracts_pass_on_well_formed_rows() {
        let rows = vec![film(1, "BUCKET BROTHERHOOD", 34), film(2, "ROCKETEER MOTHER", 33)];
        assert_contracts_films(&rows, 2);
    }

    #[test]
    #[should_panic(expected = "films-row-count")]
    fn films_contracts_fail_on_wrong_length() {
        assert_contracts_films(&[film(1, "T", 5)], 2);
    }

    #[test]
    #[should_panic(expected = "films-top-nonzero")]
    fn films_contracts_fail_on_zero_top() {
        assert_contracts_films(&[film(1, "T", 0)], 1);
    }

    #[test]
    #[should_panic(expected = "films-monotonic")]
    fn films_contracts_fail_on_disordered_rows() {
        assert_contracts_films(&[film(1, "T1", 3), film(2, "T2", 5)], 2);
    }

    #[test]
    #[should_panic(expected = "films-row-shape")]
    fn films_contracts_fail_on_nonpositive_id() {
        assert_contracts_films(&[film(0, "T", 5)], 1);
    }

    #[test]
    #[should_panic(expected = "films-row-shape")]
    fn films_contracts_fail_on_empty_title() {
        assert_contracts_films(&[film(1, "", 5)], 1);
    }

    #[test]
    fn actors_contracts_pass_on_well_formed_rows() {
        let rows = vec![actor(1, "GINA", "DEGENERES", 42), actor(2, "WALTER", "TORN", 41)];
        assert_contracts_actors(&rows, 2);
    }

    #[test]
    #[should_panic(expected = "actors-row-count")]
    fn actors_contracts_fail_on_wrong_length() {
        assert_contracts_actors(&[actor(1, "A", "B", 5)], 2);
    }

    #[test]
    #[should_panic(expected = "actors-top-nonzero")]
    fn actors_contracts_fail_on_zero_top() {
        assert_contracts_actors(&[actor(1, "A", "B", 0)], 1);
    }

    #[test]
    #[should_panic(expected = "actors-monotonic")]
    fn actors_contracts_fail_on_disordered_rows() {
        assert_contracts_actors(&[actor(1, "A", "B", 3), actor(2, "C", "D", 5)], 2);
    }

    #[test]
    #[should_panic(expected = "actors-row-shape")]
    fn actors_contracts_fail_on_nonpositive_id() {
        assert_contracts_actors(&[actor(0, "A", "B", 5)], 1);
    }

    #[test]
    #[should_panic(expected = "actors-row-shape")]
    fn actors_contracts_fail_on_empty_first_name() {
        assert_contracts_actors(&[actor(1, "", "B", 5)], 1);
    }

    #[test]
    #[should_panic(expected = "actors-row-shape")]
    fn actors_contracts_fail_on_empty_last_name() {
        assert_contracts_actors(&[actor(1, "A", "", 5)], 1);
    }

    #[tokio::test]
    async fn pool_returns_error_for_unreachable_url() {
        let result = pool("postgres://nobody@127.0.0.1:1/none").await;
        assert!(result.is_err(), "unreachable URL must return Err, not panic");
    }

    #[test]
    fn write_to_path_creates_parent_directories() {
        let tmp = std::env::temp_dir().join(format!("pg-reports-{}", std::process::id()));
        let nested = tmp.join("a").join("b").join("out.json");
        write_to_path(&nested, "[]").expect("write should succeed");
        assert_eq!(std::fs::read_to_string(&nested).expect("read"), "[]");
        std::fs::remove_dir_all(&tmp).ok();
    }

    #[test]
    fn write_to_path_handles_bare_filename() {
        let tmp = std::env::temp_dir().join(format!("pg-bare-{}.json", std::process::id()));
        write_to_path(&tmp, "[1]").expect("write should succeed");
        assert_eq!(std::fs::read_to_string(&tmp).expect("read"), "[1]");
        std::fs::remove_file(&tmp).ok();
    }

    #[test]
    fn write_to_path_errors_on_unwritable_target() {
        // `Path::new("")` has no parent, so the create_dir_all branch is
        // skipped — and `std::fs::write("", ...)` then fails. This covers
        // the false arm of the `if let Some(parent) = ...` filter.
        let result = write_to_path(Path::new(""), "[]");
        assert!(result.is_err(), "empty path must surface a write error");
    }

    #[test]
    fn write_to_path_surfaces_create_dir_all_failure() {
        // Pre-create a regular file, then ask write_to_path to write
        // *underneath* it. `create_dir_all` fails because the parent path
        // already exists as a non-directory — exercising the
        // `with_context` closure on the create_dir_all branch.
        let blocker = std::env::temp_dir().join(format!("pg-block-{}", std::process::id()));
        std::fs::write(&blocker, "i am a file, not a directory").expect("seed blocker");
        let target = blocker.join("inner.json");
        let result = write_to_path(&target, "[]");
        assert!(result.is_err(), "create_dir_all must fail under a regular file");
        let msg = format!("{:?}", result.expect_err("create_dir_all should have failed"));
        assert!(msg.contains("failed to create directory"), "unexpected error: {msg}");
        std::fs::remove_file(&blocker).ok();
    }

    #[test]
    fn assert_well_formed_json_accepts_arrays() {
        assert_well_formed_json("[]");
        assert_well_formed_json("[{\"a\":1}]");
    }

    #[test]
    #[should_panic(expected = "run-output")]
    fn assert_well_formed_json_rejects_objects() {
        assert_well_formed_json("{\"a\":1}");
    }

    #[test]
    #[should_panic(expected = "run-output")]
    fn assert_well_formed_json_rejects_garbage() {
        assert_well_formed_json("not json");
    }
}
