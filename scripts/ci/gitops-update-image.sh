#!/usr/bin/env bash
# Point an environment's manifest at a new image, commit and push. Run inside the config-repo checkout.
# Usage: gitops-update-image.sh <environment> <image-repo> <tag>
source "$(dirname "${BASH_SOURCE[0]}")/_lib.sh"
[[ $# -eq 3 ]] || { echo "usage: $0 <environment> <image-repo> <tag>" >&2; exit 2; }
env_name="$1" image="${2,,}:$3"

file="environments/${env_name}/rust-api-deployment.yaml"
[[ -f "${file}" ]] || { echo "manifest not found: ${file}" >&2; exit 1; }

log "Setting ${file} -> ${image}"
sed -i "s#image: ghcr.io/.*:.*#image: ${image}#g" "${file}"

git config user.name "github-actions[bot]"
git config user.email "github-actions[bot]@users.noreply.github.com"
git add "${file}"
if git diff --cached --quiet; then
  echo "No changes to commit."
else
  git commit -m "Update ${env_name} image to ${image}"
  git push origin main
fi
