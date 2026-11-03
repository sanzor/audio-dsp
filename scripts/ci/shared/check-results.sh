#!/usr/bin/env bash
# Gate job: fail if any upstream job failed or was cancelled (skipped is fine).
# Usage: NEEDS_JSON='${{ toJSON(needs) }}' check-results.sh
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
[[ -n "${NEEDS_JSON:-}" ]] || die "NEEDS_JSON is empty"
python3 - <<'PY'
import json, os, sys
needs = json.loads(os.environ["NEEDS_JSON"])
for name, job in needs.items():
    print(f"{name}: {job['result']}")
bad = [n for n, j in needs.items() if j["result"] in ("failure", "cancelled")]
if bad:
    sys.exit(f"error: failed: {', '.join(bad)}")
print("\n==> all lanes passed or were skipped")
PY
