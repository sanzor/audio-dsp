#!/usr/bin/env bash
# Local equivalent of the CI build-image job (CI uses docker/build-push-action for layer caching).
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
log "docker build ${BACKEND_IMAGE}"
docker build -t "${BACKEND_IMAGE}" -f "${BACKEND_DIR}/api/Dockerfile" "${BACKEND_DIR}"
