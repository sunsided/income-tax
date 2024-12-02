[private]
help:
    @just --list --unsorted

# Builds the project using cargo
build:
    cargo build

# Runs the tests
test:
    cargo test --lib --no-default-features
    cargo test --lib --all-features
    cargo test --doc --no-default-features
    cargo test --doc --all-features
    cargo doc

# Builds and opens the documenation
doc:
    cargo doc --open

# Runs code coverage
codecov PROFILE="ci":
    cargo llvm-cov nextest --all-features --lcov --output-path lcov.info "--profile={{ PROFILE }}"
