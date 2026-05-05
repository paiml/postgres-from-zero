//! Spawn the `postgres-reports` binary so `main.rs` is exercised by
//! `cargo test` (and therefore counted by `cargo llvm-cov`).
//!
//! Pre-condition: `make pagila` has loaded the database.

use std::process::Command;

fn binary_path() -> &'static str {
    env!("CARGO_BIN_EXE_postgres-reports")
}

fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/pagila".to_string())
}

#[test]
fn binary_prints_customers_json() {
    let output = Command::new(binary_path())
        .args(["--report", "customers", "--limit", "2"])
        .env("DATABASE_URL", database_url())
        .output()
        .expect("spawn binary");
    assert!(output.status.success(), "binary failed: {:?}", output);
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("parse stdout JSON");
    assert_eq!(parsed.as_array().expect("array").len(), 2);
}

#[test]
fn binary_prints_films_json() {
    let output = Command::new(binary_path())
        .args(["--report", "films", "--limit", "1"])
        .env("DATABASE_URL", database_url())
        .output()
        .expect("spawn binary");
    assert!(output.status.success(), "binary failed: {:?}", output);
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("parse stdout JSON");
    assert_eq!(parsed.as_array().expect("array").len(), 1);
}

#[test]
fn binary_prints_actors_json_with_default_limit() {
    let output = Command::new(binary_path())
        .args(["--report", "actors"])
        .env("DATABASE_URL", database_url())
        .output()
        .expect("spawn binary");
    assert!(output.status.success(), "binary failed: {:?}", output);
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    let parsed: serde_json::Value = serde_json::from_str(stdout.trim()).expect("parse stdout JSON");
    assert_eq!(parsed.as_array().expect("array").len(), 10);
}

#[test]
fn binary_writes_out_file() {
    let path = std::env::temp_dir().join(format!("pg-bin-out-{}.json", std::process::id()));
    let output = Command::new(binary_path())
        .args(["--report", "customers", "--limit", "1", "--out"])
        .arg(&path)
        .env("DATABASE_URL", database_url())
        .output()
        .expect("spawn binary");
    assert!(output.status.success(), "binary failed: {:?}", output);
    let on_disk = std::fs::read_to_string(&path).expect("read out file");
    let parsed: serde_json::Value = serde_json::from_str(&on_disk).expect("parse on-disk JSON");
    assert_eq!(parsed.as_array().expect("array").len(), 1);
    std::fs::remove_file(&path).ok();
}

#[test]
fn binary_errors_on_unreachable_database() {
    let output = Command::new(binary_path())
        .args(["--report", "customers", "--limit", "1"])
        .env("DATABASE_URL", "postgres://nobody@127.0.0.1:1/none")
        .output()
        .expect("spawn binary");
    assert!(
        !output.status.success(),
        "binary should fail with unreachable DB"
    );
}

#[test]
fn binary_help_runs() {
    let output = Command::new(binary_path())
        .arg("--help")
        .output()
        .expect("spawn binary");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("Sakila"));
}

#[test]
fn binary_version_runs() {
    let output = Command::new(binary_path())
        .arg("--version")
        .output()
        .expect("spawn binary");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8");
    assert!(stdout.contains("postgres-reports"));
}
