.PHONY: help up down pagila psql demo capstone verify test coverage fmt lint clean nuke \
        demo-1-fundamentals demo-2-joins demo-3-pagila demo-4-rust demo-all

DC := docker compose
PSQL := $(DC) exec -T postgres psql -U postgres -d pagila

help:
	@echo "Postgres From Zero — companion repo"
	@echo ""
	@echo "  make up        — start postgres in docker"
	@echo "  make down      — stop postgres (preserves the volume)"
	@echo "  make pagila    — load the Pagila sample database (idempotent)"
	@echo "  make psql      — drop into a psql shell against pagila"
	@echo "  make demo      — run the SQL example walkthrough"
	@echo "  make capstone  — build and run the Rust postgres-reports binary"
	@echo "  make verify    — assert all headline counts (CI smoke test)"
	@echo ""
	@echo "  Standalone demos (one script per concept):"
	@echo "    make demo-1-fundamentals — sql/01-fundamentals/*.sql"
	@echo "    make demo-2-joins        — sql/02-joins/*.sql"
	@echo "    make demo-3-pagila       — sql/pagila-analytics/*.sql"
	@echo "    make demo-4-rust         — Rust capstone binary, 3 contract-enforced reports"
	@echo "    make demo-all            — run every demo in sequence"
	@echo ""
	@echo "  make test      — cargo test for the Rust crate"
	@echo "  make coverage  — cargo llvm-cov (100% line gate)"
	@echo "  make fmt lint  — cargo fmt && cargo clippy"
	@echo "  make clean     — stop containers; preserves data"
	@echo "  make nuke      — clean + drop the postgres volume"

up:
	@$(DC) up -d
	@echo "Waiting for postgres to accept connections…"
	@for i in $$(seq 1 30); do \
		$(DC) exec -T postgres pg_isready -U postgres -q && exit 0; \
		sleep 1; \
	done; \
	echo "postgres did not come up in 30s"; exit 1

down:
	@$(DC) down

pagila: up
	@bash scripts/load-pagila.sh

psql: pagila
	@$(DC) exec postgres psql -U postgres -d pagila

demo: pagila
	@bash scripts/tour.sh

capstone: pagila
	@cargo run --release --bin postgres-reports -- --report customers --limit 5
	@cargo run --release --bin postgres-reports -- --report films     --limit 5
	@cargo run --release --bin postgres-reports -- --report actors    --limit 5

verify: pagila
	@bash scripts/verify.sh

demo-1-fundamentals: pagila
	@bash scripts/demo-1-fundamentals.sh

demo-2-joins: pagila
	@bash scripts/demo-2-joins.sh

demo-3-pagila: pagila
	@bash scripts/demo-3-pagila.sh

demo-4-rust: pagila
	@bash scripts/demo-4-rust.sh

demo-all: pagila
	@bash scripts/demo-all.sh

test: pagila
	@DATABASE_URL=postgres://postgres:postgres@localhost:5432/pagila cargo test --release

coverage: pagila
	@DATABASE_URL=postgres://postgres:postgres@localhost:5432/pagila \
		cargo llvm-cov --workspace --fail-under-lines 100 --summary-only

coverage-html: pagila
	@DATABASE_URL=postgres://postgres:postgres@localhost:5432/pagila \
		cargo llvm-cov --workspace --html
	@echo "report: target/llvm-cov/html/index.html"

bench:
	@cargo bench --bench contract_assertions

fmt:
	@cargo fmt --all

lint:
	@cargo clippy --all-targets -- -D warnings

clean: down

nuke:
	@$(DC) down -v
	@cargo clean
