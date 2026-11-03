#!/usr/bin/env bash
# Point an environment manifest at a new image, commit and push. Run inside the config-repo checkout.
# Usage: gitops-update-image.sh <environment> <manifest-file> <image-repo> <tag>
#   e.g. gitops-update-image.sh dev rust-api-deployment.yaml ghcr.io/owner/repo abc123
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
[[ $# -eq 4 ]] || die "usage: $0 <environment> <manifest-file> <image-repo> <tag>"
env_name="$1" manifest="$2" image="${3,,}:$4"

file="environments/${env_name}/${manifest}"
[[ -f "${file}" ]] || die "manifest not found: ${file}"

log "Setting ${file} -> ${image}"
sed -i "s#image: ghcr.io/.*:.*#image: ${image}#g" "${file}"

git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"
git add "${file}"
if git diff --cached --quiet; then
  echo "No changes to commit."; exit 0
fi
git commit -m "Update ${env_name}/${manifest} to ${image}"

# Backend and frontend deploys can race on the same repo: rebase and retry.
for attempt in 1 2 3; do
  git push origin HEAD:main && exit 0
  log "push rejected (attempt ${attempt}), rebasing"
  git pull --rebase origin main
done
die "could not push to config repo after 3 attempts"
