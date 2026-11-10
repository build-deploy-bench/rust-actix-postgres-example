#!/bin/sh
set -eu

# --locked fails the build instead of changing Cargo.lock, like npm ci.
install() {
  printf '%s\n' '::bench::install_start'
  cargo fetch --locked
  printf '%s\n' '::bench::install_done'
}

build() {
  printf '%s\n' '::bench::build_start'
  cargo build --release --locked
  printf '%s\n' '::bench::build_done'
}

case "${1:-all}" in
  install) install ;;
  build) build ;;
  all) install; build ;;
  *) printf 'Usage: %s {install|build|all}\n' "$0" >&2; exit 2 ;;
esac
