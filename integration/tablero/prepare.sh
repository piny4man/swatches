#!/usr/bin/env bash
set -euo pipefail

# Run with bash; optional name permits a second, fresh verification checkout.
root=$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)
name=${1:-tablero}
if [[ ! "$name" =~ ^tablero(-[a-zA-Z0-9]+)*$ ]]; then
    printf 'Expected checkout name tablero or tablero-<suffix>\n' >&2
    exit 1
fi
destination="$root/.work/$name"
if [[ -e "$destination" || -L "$destination" ]]; then
    printf 'Checkout already exists: %s (choose a fresh suffix)\n' "$destination" >&2
    exit 1
fi
IFS= read -r revision < "$root/integration/tablero/base-revision"
mkdir -p "$root/.work"
git clone --no-checkout https://github.com/piny4man/tablero.git "$destination"
git -C "$destination" checkout --detach "$revision"
git -C "$destination" remote rename origin upstream
git -C "$destination" remote set-url --push upstream DISABLED
git -C "$destination" apply --check "$root/integration/tablero/adapter.patch"
git -C "$destination" apply "$root/integration/tablero/adapter.patch"
printf 'Prepared %s\nBuild: cargo build --locked --manifest-path "%s/Cargo.toml" -p tablero\n' "$destination" "$destination"
