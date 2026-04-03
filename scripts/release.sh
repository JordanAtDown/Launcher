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

# Vérifier que le dépôt est propre (pas de changements non commités)
if [ -n "$(git status --porcelain)" ]; then
    echo "Erreur : des changements ne sont pas commités."
    echo "  Commite tes modifications avant de lancer la release :"
    echo "    git add <fichiers>"
    echo "    git commit -m \"fix: ...\""
    echo "    git push"
    echo ""
    git status --short
    exit 1
fi

echo "[release] Bump version -> $VERSION"
sed -i "s/^version = \"[0-9]*\.[0-9]*\.[0-9]*\"/version = \"$VERSION\"/" Cargo.toml

echo "[release] Validation..."
cargo check --quiet

echo "[release] Mise a jour du CHANGELOG..."
if command -v git-cliff &>/dev/null; then
    git-cliff --tag "v$VERSION" -o CHANGELOG.md
    git add Cargo.toml Cargo.lock CHANGELOG.md
else
    echo "  [WARN] git-cliff non trouve — CHANGELOG.md non mis a jour"
    echo "  Pour l'installer : cargo install git-cliff"
    git add Cargo.toml Cargo.lock
fi

echo "[release] Commit..."
git commit -m "chore: bump version to $VERSION"

echo "[release] Tag v$VERSION..."
git tag "v$VERSION"

echo "[release] Push..."
git push
git push origin "v$VERSION"

echo ""
echo "[release] Termine ! GitHub Actions publie la release automatiquement."
echo "  Suivi : https://github.com/JordanAtDown/Launcher/actions"
