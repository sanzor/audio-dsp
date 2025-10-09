#!/usr/bin/env bash
set -euo pipefail

echo "🧹 Cleaning up old pre-commit environments..."
pre-commit clean || true

echo "📦 Installing pre-commit hooks..."
pre-commit install --install-hooks

echo "✅ Installed hooks:"
# Show what’s installed by reading .pre-commit-config.yaml
grep -E "^- id:" .pre-commit-config.yaml || echo "No hooks found in config."

echo
echo "🧪 Running all hooks against the entire repo to verify setup..."
if pre-commit run --all-files; then
  echo "🎉 Pre-commit successfully set up and verified!"
else
  echo "⚠️ Some hooks failed during verification."
  echo "👉 You can fix issues and rerun: pre-commit run --all-files"
  exit 1
fi