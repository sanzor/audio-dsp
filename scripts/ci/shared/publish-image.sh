#!/usr/bin/env bash
# Tag a local image and push it under one or more tags.
# Usage: publish-image.sh <local-image> <image-repo> <tag> [<tag>...]
#   e.g. publish-image.sh audio-dsp-api:ci ghcr.io/owner/repo "$GITHUB_SHA" main
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
[[ $# -ge 3 ]] || die "usage: $0 <local-image> <image-repo> <tag> [<tag>...]"

local_image="$1" repo="${2,,}"; shift 2   # registries require lowercase
for tag in "$@"; do
  tag="${tag//\//-}"                       # branch names like feat/x aren't valid tags
  log "push ${repo}:${tag}"
  docker tag "${local_image}" "${repo}:${tag}"
  docker push "${repo}:${tag}"
done
