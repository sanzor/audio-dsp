#!/usr/bin/env bash
# Local equivalent of the CI build-image job: production image (vite build served by nginx).
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
log "docker build ${FRONTEND_IMAGE}"
docker build -t "${FRONTEND_IMAGE}" -f "${FRONTEND_DIR}/deploy/Dockerfile" "${FRONTEND_DIR}"
