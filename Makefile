PROJECT = modmail_rs

PRISMA_SCHEMA_PATH = prisma/schema.prisma
MIGRATION_PATH = migrations/init.sql

all: build

build:
	cargo build --release

run:
	cargo run

test:
	cargo test

watch:
	cargo watch -x 'run'

clean:
	cargo clean

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

migrate:
	rm -f db/db.sqlite
	touch db/db.sqlite
	rm -f migrations/*
	sqlx migrate add $(NAME)
	FILE=$$(find migrations -name "*$(NAME).sql"); \
	if [ -n "$$FILE" ]; then \
		bunx prisma migrate diff --from-empty --to-schema-datamodel prisma/schema.prisma --script > "$$FILE"; \
	else \
		echo "Erreur : fichier de migration introuvable pour $(NAME)"; \
		exit 1; \
	fi
	sqlx migrate run

.PHONY: all build run test clean fmt clippy
