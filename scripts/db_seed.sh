#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SEEDS_DIR="${ROOT_DIR}/database/seeds"
DB_PSQL="${ROOT_DIR}/scripts/db_psql.sh"

required_seeds=(
  "users.sql:users"
  "projects.sql:projects"
)

optional_seeds=(
  "tier_configs.sql:tier_configs"
  "products.sql:products"
)

applied=()
skipped=()

table_exists() {
  local table_name="$1"
  local exists

  exists="$(bash "${DB_PSQL}" -tAc "SELECT to_regclass('public.${table_name}') IS NOT NULL" | tr -d '[:space:]')"
  [[ "${exists}" == "t" ]]
}

apply_seed() {
  local file_name="$1"
  local table_name="$2"
  local required="$3"
  local file_path="${SEEDS_DIR}/${file_name}"

  if [[ ! -f "${file_path}" ]]; then
    if [[ "${required}" == "required" ]]; then
      echo "Missing required seed file: ${file_path}" >&2
      exit 1
    fi
    skipped+=("${file_name} (missing file)")
    return
  fi

  if ! table_exists "${table_name}"; then
    if [[ "${required}" == "required" ]]; then
      echo "Required seed table '${table_name}' does not exist in the current schema." >&2
      exit 1
    fi
    echo "Skipping ${file_name}: table '${table_name}' is not present in the current schema."
    skipped+=("${file_name} (missing table ${table_name})")
    return
  fi

  echo "Applying ${file_name}..."
  bash "${DB_PSQL}" -v ON_ERROR_STOP=1 -f - < "${file_path}"
  applied+=("${file_name}")
}

for spec in "${required_seeds[@]}"; do
  IFS=":" read -r file_name table_name <<< "${spec}"
  apply_seed "${file_name}" "${table_name}" required
done

for spec in "${optional_seeds[@]}"; do
  IFS=":" read -r file_name table_name <<< "${spec}"
  apply_seed "${file_name}" "${table_name}" optional
done

if ((${#applied[@]} == 0)); then
  echo "No seed files were applied."
else
  printf 'Applied seed files: %s\n' "${applied[*]}"
fi

if ((${#skipped[@]} > 0)); then
  printf 'Skipped seed files: %s\n' "${skipped[*]}"
fi
