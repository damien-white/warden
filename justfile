# Automatically load '.env' file (if present) during development
# NOTE: This is not for production
set dotenv-load := true
# Uncomment the next line to enable positional arguments.
#set positional-arguments := true

#export RUST_BACKTRACE := "full"

_default:
    @just --list

# Create an optimized 'release' build
@build:
    cargo build --release

# Format, lint and check that project compiles
@compile:
    cargo fmt --all
    cargo clippy -- -D warnings

# Format the project with rustfmt
@format:
    cargo fmt --all
    cargo clippy -- --D warnings

# Quickly format and run linter
@lint:
    cargo clippy && echo "   *** [Linter finished successfully] ***"

# Run code-quality and CI-related tasks locally
@pre-commit:
    cargo fmt --all -- --check
    cargo clippy -- --D warnings
    cargo test

# Run tests with 'nocapture' and 'quiet' flags set
@test:
    cargo test -- --nocapture --quiet

# Run tests single-threaded for concurrency-related debugging
@test-debug:
    cargo test -- --test-threads=1 --nocapture
