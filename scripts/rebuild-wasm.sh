#!/usr/bin/env bash
# Rebuild the wasm bundle that ships in src/wasm/generated/.
#
# wasm-pack writes a whole npm package into its --out-dir, but the runtime
# only consumes four files, so we build into a scratch directory and copy
# just those.
#
# wasm-opt runs from a pinned Binaryen (fetched with npx, since the `binaryen`
# npm package ships wasm-opt as a Node program), never from whatever
# wasm-pack downloads: an older wasm-opt once miscompiled wasm-bindgen output.
# AJISAI_WASM_OPT=0 skips the optimizer.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
out_dir="${repo_root}/src/wasm/generated"
scratch_dir="$(mktemp -d)"
trap 'rm -rf "${scratch_dir}"' EXIT
crate="ajisai_core"

if ! command -v wasm-pack >/dev/null 2>&1; then
  echo "rebuild-wasm: wasm-pack is not installed." >&2
  echo "  Install with:  bash ./scripts/install-wasm-pack.sh" >&2
  exit 1
fi

cd "${repo_root}/rust"
# The bindings sit behind the `wasm` Cargo feature so the native build never
# pulls in wasm-bindgen. wasm-pack passes args after `--` to cargo.
wasm-pack build --target web --out-dir "${scratch_dir}" --no-opt -- --features wasm

if [[ "${AJISAI_WASM_OPT:-1}" == "1" ]]; then
  BINARYEN_VERSION="132.0.0"
  wasm_opt=(npx --yes -p "binaryen@${BINARYEN_VERSION}" wasm-opt)
  echo "rebuild-wasm: optimizing with $("${wasm_opt[@]}" --version)."
  # The feature flags are the ones rustc's wasm32 target emits by default;
  # wasm-opt must be told, or it rejects the module.
  "${wasm_opt[@]}" -Oz \
    --enable-bulk-memory --enable-nontrapping-float-to-int --enable-sign-ext \
    --enable-mutable-globals --enable-reference-types --enable-multivalue \
    "${scratch_dir}/${crate}_bg.wasm" -o "${scratch_dir}/${crate}_bg.wasm"
else
  echo "rebuild-wasm: AJISAI_WASM_OPT=0; shipping the unoptimized module."
fi

mkdir -p "${out_dir}"
for f in "${crate}.js" "${crate}.d.ts" "${crate}_bg.wasm" "${crate}_bg.wasm.d.ts"; do
  cp "${scratch_dir}/${f}" "${out_dir}/${f}"
done

echo "rebuild-wasm: refreshed ${out_dir}"
