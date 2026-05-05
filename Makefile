.PHONY: help up down pagila psql demo capstone verify test coverage fmt lint clean nuke

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
