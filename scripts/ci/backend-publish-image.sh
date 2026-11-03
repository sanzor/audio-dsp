#!/usr/bin/env bash
# Tag ${CI_IMAGE} and push it to a registry under one or more tags.
# Usage: backend-publish-image.sh <image-repo> <tag> [<tag>...]
#   e.g. backend-publish-image.sh ghcr.io/owner/repo "$GITHUB_SHA" main
source "$(dirname "${BASH_SOURCE[0]}")/_lib.sh"
[[ $# -ge 2 ]] || { echo "usage: $0 <image-repo> <tag> [<tag>...]" >&2; exit 2; }

repo="${1,,}"; shift   # registries require lowercase
for tag in "$@"; do
  tag="${tag//\//-}"   # branch names like feat/x aren't valid tags
  log "push ${repo}:${tag}"
  docker tag "${CI_IMAGE}" "${repo}:${tag}"
  docker push "${repo}:${tag}"
done
