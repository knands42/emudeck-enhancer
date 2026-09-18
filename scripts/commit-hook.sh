#!/usr/bin/env bash
set -euo pipefail

echo "Running cargo fmt..."
cargo fmt

echo "Re-staging formatted files..."
git add -u