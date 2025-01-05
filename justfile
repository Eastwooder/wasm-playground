set shell := ["powershell.exe", "-c"]

build: generate-doc build-guest build-host

clippy: clippy-guest clippy-host

fmt: fmt-guest fmt-host

clean:
    cargo clean --manifest-path guest-logic/Cargo.toml

generate-doc:
    wit-bindgen markdown wit --out-dir target

build-guest:
    cargo build --target wasm32-wasip2 --manifest-path guest-logic/Cargo.toml

clippy-guest:
    cargo clippy --manifest-path guest-logic/Cargo.toml

inspect-guest:
    wasm-tools component wit target/wasm32-wasip2/debug/guest_logic.wasm

fmt-guest:
    cargo fmt --manifest-path guest-logic/Cargo.toml

build-host:
    cargo build --manifest-path host-processor/Cargo.toml

clippy-host:
    cargo clippy --manifest-path host-processor/Cargo.toml

run-host:
    cargo run --manifest-path host-processor/Cargo.toml

fmt-host:
    cargo fmt --manifest-path host-processor/Cargo.toml