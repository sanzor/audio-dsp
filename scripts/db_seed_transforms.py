#!/usr/bin/env python3
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT_DIR = Path(__file__).resolve().parent.parent
DB_PSQL = ROOT_DIR / "scripts" / "db_psql.sh"
MANIFEST_PATH = ROOT_DIR / "database" / "seeds" / "transforms_manifest.json"
COMPOSITES_MANIFEST_PATH = ROOT_DIR / "database" / "seeds" / "composites_manifest.json"
SOURCE_DIR = ROOT_DIR / "database" / "seeds" / "transform_wasm"

# All seed transforms are simple main-signal in/out effects — no sidechain
# inputs or summed ("many") ports in this catalog.
DEFAULT_PORT_KIND = "program"
DEFAULT_PORT_CARDINALITY = "single"


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


def sql_jsonb(value: object) -> str:
    return sql_text(json.dumps(value)) + "::jsonb"


def resolve_wasm(binary_name: str) -> Path:
    binary_path = SOURCE_DIR / binary_name
    if not binary_path.exists():
        raise FileNotFoundError(f"Missing transform binary: {binary_path}")
    return binary_path


def resolve_source(binary_name: str) -> Path:
    """The .rs source alongside the compiled binary, matched by basename
    (bin/gain.wasm -> gain.rs) — the human-authored source the Creator's
    Save bucket (transform_draft.source_code) should show for a catalog
    transform, kept in sync with the compiled Publish-bucket binary."""
    source_path = SOURCE_DIR / (Path(binary_name).stem + ".rs")
    if not source_path.exists():
        raise FileNotFoundError(f"Missing transform source: {source_path}")
    return source_path


def build_sql(transform: dict, wasm_path: Path, source_path: Path) -> str:
    wasm_hex = wasm_path.read_bytes().hex()
    source_code = source_path.read_text()

    port_rows = []
    for port in transform["ports"]:
        port_rows.append(
            f"(t_id, {sql_text(port['name'])}, {sql_text(port['direction'])}, "
            f"{sql_number(port['port_order'])}, {sql_text(port.get('description'))}, "
            f"{sql_text(port.get('kind', DEFAULT_PORT_KIND))}, "
            f"{sql_text(port.get('cardinality', DEFAULT_PORT_CARDINALITY))})"
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
    INSERT INTO transform (name, description, icon, kind, owner_user_id, is_default)
    VALUES (
        {sql_text(transform['name'])},
        {sql_text(transform.get('description'))},
        {sql_text(transform.get('icon'))},
        'primitive',
        (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
        true
    )
    ON CONFLICT (name) DO UPDATE
    SET description = EXCLUDED.description,
        icon = EXCLUDED.icon,
        is_default = true
    RETURNING transform_id INTO t_id;

    DELETE FROM transform_port WHERE transform_id = t_id;
    INSERT INTO transform_port (transform_id, name, direction, port_order, description, kind, cardinality)
    VALUES {", ".join(port_rows)};

    DELETE FROM transform_param WHERE transform_id = t_id;
    INSERT INTO transform_param (transform_id, name, param_order, default_value, min_value, max_value, description)
    VALUES {", ".join(param_rows)};

    INSERT INTO transform_binary (transform_id, wasm_bytecode, source)
    VALUES (t_id, decode('{wasm_hex}', 'hex'), {sql_text(source_ref)})
    ON CONFLICT (transform_id) DO UPDATE
    SET wasm_bytecode = EXCLUDED.wasm_bytecode,
        source = EXCLUDED.source,
        updated_at = now();

    INSERT INTO transform_draft (transform_id, source_code, name, description, ports, params)
    VALUES (
        t_id,
        {sql_text(source_code)},
        {sql_text(transform['name'])},
        {sql_text(transform.get('description'))},
        {sql_jsonb(transform['ports'])},
        {sql_jsonb(transform['params'])}
    )
    ON CONFLICT (transform_id) DO UPDATE
    SET source_code = EXCLUDED.source_code,
        name = EXCLUDED.name,
        description = EXCLUDED.description,
        ports = EXCLUDED.ports,
        params = EXCLUDED.params,
        updated_at = now();
END
$seed$;
"""


def seed_transforms() -> None:
    manifest = json.loads(MANIFEST_PATH.read_text())
    statements: list[str] = []

    for transform in manifest:
        wasm_path = resolve_wasm(transform["binary"])
        source_path = resolve_source(transform["binary"])
        statements.append(build_sql(transform, wasm_path, source_path))

    sql = "BEGIN;\n" + "\n".join(statements) + "\nCOMMIT;\n"
    run_checked(["bash", str(DB_PSQL), "-v", "ON_ERROR_STOP=1", "-f", "-"], stdin=sql)


# ─── Composite transforms ──────────────────────────────────────────────────
# A composite has no wasm binary of its own — it's a graph of already-seeded
# primitive transforms (see CompositeGraphDefinition in
# domain/db/transform_snapshot.rs and composite_validator.rs). Seeding one
# means writing the same rows the real save-draft/publish flow would:
# transform (kind='composite'), transform_draft.graph_definition (what the
# Creator's composite canvas loads), transform_port (the exposed ports other
# graphs wire against), and transform_composite (the published graph the
# Editor/runtime resolves). Node transform_ids aren't known until the leaf
# primitives above are actually seeded, so they're resolved by name here
# rather than hardcoded.


def table_exists(table_name: str) -> bool:
    out = run_checked(
        ["bash", str(DB_PSQL), "-tAc", f"SELECT to_regclass('public.{table_name}') IS NOT NULL"]
    )
    return out.strip() == "t"


def resolve_transform_ids(names: set[str]) -> dict[str, int]:
    if not names:
        return {}
    name_list = ", ".join(sql_text(n) for n in sorted(names))
    out = run_checked(
        [
            "bash",
            str(DB_PSQL),
            "-tAc",
            f"SELECT COALESCE(jsonb_object_agg(name, transform_id), '{{}}'::jsonb) FROM transform WHERE name IN ({name_list})",
        ]
    )
    return json.loads(out.strip())


def leaf_port(primitives_by_name: dict[str, dict], transform_name: str, port_name: str) -> dict:
    transform = primitives_by_name.get(transform_name)
    if transform is None:
        raise KeyError(f"composite seed references unknown primitive '{transform_name}'")
    for port in transform["ports"]:
        if port["name"] == port_name:
            return port
    raise KeyError(f"primitive '{transform_name}' has no port named '{port_name}'")


def build_composite_sql(composite: dict, primitives_by_name: dict[str, dict], leaf_ids: dict[str, int]) -> str:
    node_transform_name = {n["node_id"]: n["transform_name"] for n in composite["nodes"]}

    # graph_definition.nodes uses the node_kind-tagged Input/Output-node model
    # (migration 0020_composite_io_nodes), not the older separate
    # `exposed_ports` list — see CompositeNode in
    # backend/domain/src/db/transform_snapshot.rs (internally tagged on
    # `node_kind`, no `exposed_ports` key at all). The manifest's own
    # `exposed_ports` list stays a convenient authoring shape here; it's
    # compiled below into literal Input/Output nodes plus the edges wiring
    # them to their leaf, exactly like migration 0020's one-time backfill did
    # by hand for "Vocal Chain" — this makes that translation generic for any
    # composite in the manifest instead of relying on a one-off migration.
    leaf_nodes = [
        {"node_kind": "leaf", "node_id": n["node_id"], "transform_id": leaf_ids[n["transform_name"]]}
        for n in composite["nodes"]
    ]
    next_io_node_id = max((n["node_id"] for n in composite["nodes"]), default=0) + 1

    io_nodes = []
    io_edges = []
    exposed_ports = []
    for order, exposed in enumerate(composite["exposed_ports"]):
        leaf_name = node_transform_name[exposed["node_id"]]
        source_port = leaf_port(primitives_by_name, leaf_name, exposed["port_name"])
        direction = source_port["direction"]
        io_node_id = next_io_node_id
        next_io_node_id += 1

        io_node_kind = "input" if direction == "input" else "output"
        io_nodes.append({"node_kind": io_node_kind, "node_id": io_node_id, "name": exposed["exposed_name"]})
        if direction == "input":
            # External signal flows into the leaf's input port.
            io_edges.append(
                {
                    "from_node_id": io_node_id,
                    "from_port": "signal",
                    "to_node_id": exposed["node_id"],
                    "to_port": exposed["port_name"],
                }
            )
        else:
            # The leaf's output port flows out to the composite boundary.
            io_edges.append(
                {
                    "from_node_id": exposed["node_id"],
                    "from_port": exposed["port_name"],
                    "to_node_id": io_node_id,
                    "to_port": "signal",
                }
            )

        exposed_ports.append(
            {
                "name": exposed["exposed_name"],
                "direction": direction,
                "port_order": order,
                "description": None,
                "kind": source_port.get("kind", DEFAULT_PORT_KIND),
                "cardinality": source_port.get("cardinality", DEFAULT_PORT_CARDINALITY),
            }
        )

    graph_definition = {
        "nodes": leaf_nodes + io_nodes,
        "edges": composite["edges"] + io_edges,
    }

    port_rows = []
    for port in exposed_ports:
        port_rows.append(
            f"(t_id, {sql_text(port['name'])}, {sql_text(port['direction'])}, "
            f"{sql_number(port['port_order'])}, {sql_text(port['description'])}, "
            f"{sql_text(port['kind'])}, {sql_text(port['cardinality'])})"
        )

    graph_jsonb = sql_jsonb(graph_definition)
    ports_jsonb = sql_jsonb(exposed_ports)

    return f"""
DO $seed$
DECLARE
    t_id BIGINT;
BEGIN
    INSERT INTO transform (name, description, icon, kind, owner_user_id, is_default)
    VALUES (
        {sql_text(composite['name'])},
        {sql_text(composite.get('description'))},
        {sql_text(composite.get('icon'))},
        'composite',
        (SELECT user_id FROM users WHERE email = 'admin@gmail.com'),
        true
    )
    ON CONFLICT (name) DO UPDATE
    SET description = EXCLUDED.description,
        icon = EXCLUDED.icon,
        is_default = true
    RETURNING transform_id INTO t_id;

    INSERT INTO transform_draft (transform_id) VALUES (t_id) ON CONFLICT (transform_id) DO NOTHING;

    UPDATE transform_draft
    SET graph_definition = {graph_jsonb},
        ports = {ports_jsonb},
        name = {sql_text(composite['name'])},
        description = {sql_text(composite.get('description'))},
        updated_at = now()
    WHERE transform_id = t_id;

    DELETE FROM transform_port WHERE transform_id = t_id;
    INSERT INTO transform_port (transform_id, name, direction, port_order, description, kind, cardinality)
    VALUES {", ".join(port_rows)};

    INSERT INTO transform_composite (transform_id, graph_definition)
    VALUES (t_id, {graph_jsonb})
    ON CONFLICT (transform_id) DO UPDATE
    SET graph_definition = EXCLUDED.graph_definition,
        updated_at = now();
END
$seed$;
"""


def seed_composites() -> None:
    if not COMPOSITES_MANIFEST_PATH.exists():
        return
    if not table_exists("transform_composite"):
        print("Skipping composite transform seeds: table 'transform_composite' is not present.")
        return

    primitives_by_name = {t["name"]: t for t in json.loads(MANIFEST_PATH.read_text())}
    composites = json.loads(COMPOSITES_MANIFEST_PATH.read_text())
    if not composites:
        return

    leaf_names: set[str] = set()
    for composite in composites:
        leaf_names.update(n["transform_name"] for n in composite["nodes"])
    leaf_ids = resolve_transform_ids(leaf_names)

    missing = leaf_names - leaf_ids.keys()
    if missing:
        raise RuntimeError(f"composite seed references leaf transform(s) not found in the database: {sorted(missing)}")

    statements = [build_composite_sql(c, primitives_by_name, leaf_ids) for c in composites]
    sql = "BEGIN;\n" + "\n".join(statements) + "\nCOMMIT;\n"
    run_checked(["bash", str(DB_PSQL), "-v", "ON_ERROR_STOP=1", "-f", "-"], stdin=sql)


def main() -> int:
    try:
        seed_transforms()
        seed_composites()
    except Exception as exc:  # noqa: BLE001
        print(f"Failed to seed transforms: {exc}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
