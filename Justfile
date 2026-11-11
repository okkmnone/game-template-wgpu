set windows-shell := ["cmd.exe", "/c"]

_default:
    just --list

fmt:
    cargo fmt

debug: fmt
    cargo run

release:
    cargo run --release
