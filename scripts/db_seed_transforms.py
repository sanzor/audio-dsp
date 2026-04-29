#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
DB_PSQL = ROOT_DIR / "scripts" / "db_psql.sh"
MANIFEST_PATH = ROOT_DIR / "database" / "seeds" / "transforms_manifest.json"
SOURCE_DIR = ROOT_DIR / "database" / "seeds" / "transform_wasm"


def run_checked(cmd: list[str], *, stdin: str | None = None) -> str:
    proc = subprocess.run(
        cmd,
        cwd=ROOT_DIR,
        input=stdin,
        text=True,
        capture_output=True,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or "command failed")
    return proc.stdout


def sql_text(value: str | None) -> str:
    if value is None:
        return "NULL"
    return "'" + value.replace("'", "''") + "'"


def sql_number(value: float | int | None) -> str:
    if value is None:
        return "NULL"
    return str(value)


def resolve_wasm(binary_name: str) -> Path:
    binary_path = SOURCE_DIR / binary_name
    if not binary_path.exists():
        raise FileNotFoundError(f"Missing transform binary: {binary_path}")
    return binary_path


def build_sql(transform: dict, wasm_path: Path) -> str:
    wasm_hex = wasm_path.read_bytes().hex()

    port_rows = []
    for port in transform["ports"]:
        port_rows.append(
            f"(t_id, {sql_text(port['name'])}, {sql_text(port['direction'])}, "
            f"{sql_number(port['port_order'])}, {sql_text(port.get('description'))})"
        )

    param_rows = []
    for param in transform["params"]:
        param_rows.append(
            f"(t_id, {sql_text(param['name'])}, {sql_number(param['param_order'])}, "
            f"{sql_number(param['default_value'])}, {sql_number(param.get('min_value'))}, "
            f"{sql_number(param.get('max_value'))}, {sql_text(param.get('description'))})"
        )

    source_ref = f"transform_wasm/{transform['binary']}"

    return f"""
DO $seed$
DECLARE
    t_id BIGINT;
BEGIN
    INSERT INTO transforms (name, description, icon)
    VALUES ({sql_text(transform['name'])}, {sql_text(transform.get('description'))}, {sql_text(transform.get('icon'))})
    ON CONFLICT (name) DO UPDATE
    SET description = EXCLUDED.description,
        icon = EXCLUDED.icon
    RETURNING transform_id INTO t_id;

    DELETE FROM transform_ports WHERE transform_id = t_id;
    INSERT INTO transform_ports (transform_id, name, direction, port_order, description)
    VALUES {", ".join(port_rows)};

    DELETE FROM transform_params WHERE transform_id = t_id;
    INSERT INTO transform_params (transform_id, name, param_order, default_value, min_value, max_value, description)
    VALUES {", ".join(param_rows)};

    INSERT INTO transform_binaries (transform_id, wasm_bytecode, source)
    VALUES (t_id, decode('{wasm_hex}', 'hex'), {sql_text(source_ref)})
    ON CONFLICT (transform_id) DO UPDATE
    SET wasm_bytecode = EXCLUDED.wasm_bytecode,
        source = EXCLUDED.source,
        updated_at = now();
END
$seed$;
"""


def seed_transforms() -> None:
    manifest = json.loads(MANIFEST_PATH.read_text())
    statements: list[str] = []

    for transform in manifest:
        wasm_path = resolve_wasm(transform["binary"])
        statements.append(build_sql(transform, wasm_path))

    sql = "BEGIN;\n" + "\n".join(statements) + "\nCOMMIT;\n"
    run_checked([str(DB_PSQL), "-v", "ON_ERROR_STOP=1", "-f", "-"], stdin=sql)


def main() -> int:
    try:
        seed_transforms()
    except Exception as exc:  # noqa: BLE001
        print(f"Failed to seed transforms: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
