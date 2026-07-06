set dotenv-load := false

default:
    just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --exclude rs-dean-cube-smoke --exclude rs-dean-bevy-scenes --all-targets -- -D warnings

test:
    cargo nextest run --workspace --exclude rs-dean-cube-smoke --exclude rs-dean-bevy-scenes

doctest:
    cargo test --workspace --exclude rs-dean-cube-smoke --exclude rs-dean-bevy-scenes --doc

dev:
    cargo xtask dev

stories:
    cargo xtask stories

cube-smoke:
    cargo xtask cube-smoke

doctor:
    cargo xtask doctor

build:
    cargo xtask build

gate:
    cargo xtask gate

check:
    cargo xtask gate

five-phase-pass:
    cargo xtask five-phase-pass

docs-sweep:
    cargo xtask docs-sweep

bootstrap:
    cargo install --locked cargo-nextest
    cargo install --locked cargo-deny
    cargo install --locked cargo-machete
    cargo install --locked trunk
