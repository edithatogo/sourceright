param(
    [int]$CoverageMinimum = 90,
    [switch]$SkipCoverage
)

$ErrorActionPreference = 'Stop'

cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
cargo test --locked

if (-not $SkipCoverage) {
    cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines $CoverageMinimum
}
