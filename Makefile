.PHONY: build
build:
	cargo build

.PHONY: setup
setup:
	cargo install graphql_client_cli

.PHONY: generate
generate:
	graphql-client generate graphql/query.graphql --schema-path graphql/schema.docs.graphql -o ./src
