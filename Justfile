set dotenv-load := false

default:
    just --list

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --exclude rs-dean-game --exclude rs-dean-bevy-scenes --exclude rs-dean-ui-bevy-stories --all-targets -- -D warnings

test:
    cargo nextest run --workspace --exclude rs-dean-game --exclude rs-dean-bevy-scenes --exclude rs-dean-ui-bevy-stories

doctest:
    cargo test --workspace --exclude rs-dean-game --exclude rs-dean-bevy-scenes --exclude rs-dean-ui-bevy-stories --doc

dev:
    cargo xtask dev

game:
    cargo xtask game

stories:
    cargo xtask stories

ui-bevy-stories:
    cargo xtask ui-bevy-stories

cube-smoke:
    cargo xtask cube-smoke

doctor:
    cargo xtask doctor

build:
    cargo xtask build

pages:
    cargo xtask pages

ui-book:
    cargo xtask gen-ui-book

blocks-book:
    cargo xtask gen-block-book

block-issues:
    cargo xtask gen-block-issues

static-analysis:
    cargo xtask static-analysis

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
    cargo install --locked mdbook
    cargo install --locked trunk
    cargo xtask install-tailwindcss
