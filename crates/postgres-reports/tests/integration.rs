use postgres_reports::*;

fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/pagila".to_string())
}

#[tokio::test]
async fn customers_report_matches_contracts() {
    let pool = pool(&database_url()).await.expect("connect");
    let rows = top_customers(&pool, 10).await.expect("query");
    assert_eq!(rows.len(), 10);
    assert!(rows.iter().all(|r| r.name.contains(' ')));
}

#[tokio::test]
async fn films_report_matches_contracts() {
    let pool = pool(&database_url()).await.expect("connect");
    let rows = top_films(&pool, 5).await.expect("query");
    assert_eq!(rows.len(), 5);
    assert!(rows[0].rental_count >= rows[4].rental_count);
}

#[tokio::test]
async fn actors_report_matches_contracts() {
    let pool = pool(&database_url()).await.expect("connect");
    let rows = top_actors(&pool, 3).await.expect("query");
    assert_eq!(rows.len(), 3);
    assert!(rows[0].film_count >= 1);
}

#[tokio::test]
async fn run_customers_returns_json_array() {
    let pool = pool(&database_url()).await.expect("connect");
    let opts = RunOptions {
        report: Report::Customers,
        limit: 3,
        out: None,
        database_url: database_url(),
    };
    let json = run(&pool, &opts).await.expect("run");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed.as_array().expect("array").len(), 3);
}

#[tokio::test]
async fn run_films_writes_out_file() {
    let pool = pool(&database_url()).await.expect("connect");
    let path = std::env::temp_dir().join(format!("pg-films-{}.json", std::process::id()));
    let opts = RunOptions {
        report: Report::Films,
        limit: 2,
        out: Some(path.clone()),
        database_url: database_url(),
    };
    let json = run(&pool, &opts).await.expect("run");
    let on_disk = std::fs::read_to_string(&path).expect("read");
    assert_eq!(json, on_disk);
    std::fs::remove_file(&path).ok();
}

#[tokio::test]
async fn run_actors_returns_json_array() {
    let pool = pool(&database_url()).await.expect("connect");
    let opts = RunOptions {
        report: Report::Actors,
        limit: 4,
        out: None,
        database_url: database_url(),
    };
    let json = run(&pool, &opts).await.expect("run");
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed.as_array().expect("array").len(), 4);
}
