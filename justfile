# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

watch:
    cargo watch -s 'just doc' -s 'just fmt'

# Build package
build:
    cargo hack --feature-powerset build --verbose

# Runs clippy on the sources
check:
    cargo hack --feature-powerset clippy --locked -- -D warnings

# Runs unit tests
test:
    cargo hack --feature-powerset --skip=extension-module test --locked

# Build documentation
doc:
    RUSTDOCFLAGS="-Dwarnings -Z unstable-options --enable-index-page" cargo +nightly doc --all-features

# Format code
fmt *args:
    cargo fmt {{args}}
