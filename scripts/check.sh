#!/usr/bin/env sh
# Runs the same checks as CI. Rust runs inside the official Docker image, so
# the host needs only Docker, a JDK 17+ and an Android SDK (for mobile/).
# Usage: scripts/check.sh [desktop|spikes|mobile]   (no argument = all)
set -eu
root=$(cd "$(dirname "$0")/.." && pwd)

rust() { # rust <dir> <commands>
  docker run --rm \
    -v gw-cargo-registry:/usr/local/cargo/registry -v gw-rustup:/usr/local/rustup \
    -v "$root":/w -w "/w/$1" rust:1.100.0 \
    sh -c "trap 'chown -R $(id -u):$(id -g) /w/$1' EXIT; $2"
}

# Linux can't link Windows binaries, so `check`/`clippy` cover the Windows code
# and `test` covers the portable code. CI builds the real .exe on windows-latest.
win=--target=x86_64-pc-windows-msvc

desktop() {
  rust desktop "cargo fmt --check && cargo clippy --all-targets -- -D warnings \
    && cargo clippy $win --all-targets -- -D warnings && cargo test"
}

spikes() {
  for d in "$root"/spikes/*/; do
    n=$(basename "$d")
    echo "== spikes/$n"
    rust "spikes/$n" "cargo fmt --check && cargo clippy $win --all-targets -- -D warnings && cargo test"
  done
}

mobile() {
  (cd "$root/mobile" && ./gradlew --no-daemon lint testDebugUnitTest assembleDebug)
}

case "${1:-all}" in
  desktop) desktop ;;
  spikes) spikes ;;
  mobile) mobile ;;
  all) desktop; spikes; mobile ;;
  *) echo "usage: $0 [desktop|spikes|mobile]" >&2; exit 2 ;;
esac
