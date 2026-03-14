#!/usr/bin/env bash
# Usage: bash scripts/release.sh [version]
# Lance le workflow complet de release : bump version, commit, tag, push.
# GitHub Actions prend le relais et publie la release (~2-3 min).
set -euo pipefail

VERSION="${1:-}"

if [ -z "$VERSION" ]; then
    read -rp "Version a deployer (ex: 0.2.0) : " VERSION
fi

if [[ ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Erreur : version invalide '$VERSION' (attendu: X.Y.Z)"
    exit 1
fi

echo "[release] Bump version -> $VERSION"
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$VERSION\"/" Cargo.toml

echo "[release] Validation..."
cargo check --quiet

echo "[release] Commit..."
git add Cargo.toml Cargo.lock
git commit -m "chore: bump version to $VERSION"

echo "[release] Tag v$VERSION..."
git tag "v$VERSION"

echo "[release] Push..."
git push
git push origin "v$VERSION"

echo ""
echo "[release] Termine ! GitHub Actions publie la release automatiquement."
echo "  Suivi : https://github.com/JordanAtDown/Launcher/actions"
