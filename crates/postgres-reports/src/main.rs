//! `postgres-reports` — clap-driven entrypoint for the `run` library function.
//!
//! Provable contract: every successful invocation prints a JSON array to
//! stdout. The contract is asserted by [`postgres_reports::run`] before this
//! binary ever sees the bytes.

use anyhow::Result;
use clap::{Parser, ValueEnum};
use postgres_reports::{pool, run, Report, RunOptions};

#[derive(Parser, Debug)]
#[command(name = "postgres-reports", version, about = "Sakila reports via sqlx + Postgres")]
struct Cli {
    #[arg(long, value_enum, default_value_t = ReportArg::Customers)]
    report: ReportArg,

    #[arg(long, default_value_t = 10)]
    limit: i64,

    #[arg(long)]
    out: Option<std::path::PathBuf>,

    #[arg(long, env = "DATABASE_URL",
          default_value = "postgres://postgres:postgres@localhost:5432/pagila")]
    database_url: String,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum ReportArg {
    Customers,
    Films,
    Actors,
}

impl From<ReportArg> for Report {
    fn from(value: ReportArg) -> Self {
        match value {
            ReportArg::Customers => Report::Customers,
            ReportArg::Films => Report::Films,
            ReportArg::Actors => Report::Actors,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let opts = RunOptions {
        report: cli.report.into(),
        limit: cli.limit,
        out: cli.out,
        database_url: cli.database_url,
    };
    let pool = pool(&opts.database_url).await?;
    let json = run(&pool, &opts).await?;
    println!("{json}");
    Ok(())
}
