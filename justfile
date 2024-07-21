# Set shell for Windows OSs:
development_features := "bevy/dynamic_linking,bevy/file_watcher,bevy/embedded_watcher"
cargo_cmd := if os_family() == "windows" { "cargo" } else { "mold -run cargo"}

alias r := run
alias ch := check
alias cl := clippy
alias f := fmt
alias l := lint-all
alias t := test

run:
    {{cargo_cmd}} run --features={{development_features}}

example ARGS:
    {{cargo_cmd}} run --features={{development_features}} --example {{ARGS}}

check: 
    {{cargo_cmd}} check --all-targets 

check-all: 
    {{cargo_cmd}} check --all-targets --all

check-release: 
    {{cargo_cmd}} check --all-targets --all --release

fmt:
    {{cargo_cmd}} fmt --all

lint-all: lint check-release clippy-release

lint: fmt check check-all clippy 

clippy:
    {{cargo_cmd}} clippy --all-targets --all

clippy-release:
    {{cargo_cmd}} clippy --all-targets --all --release

test:
    {{cargo_cmd}} nextest run --features={{development_features}} --all

test-all:
    {{cargo_cmd}} nextest run --features={{development_features}} --all --all-targets

build:
    {{cargo_cmd}} build --features={{development_features}} --timings
