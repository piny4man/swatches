#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

cargo fmt --check
cargo test --locked --all-targets
cargo test --doc --locked
cargo clippy --locked --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --locked --no-deps

package_list=$(mktemp)
work_dir=$(mktemp -d)
trap 'rm -f "$package_list"; rm -rf "$work_dir"' EXIT HUP INT TERM

cargo package --allow-dirty --locked
cargo package --allow-dirty --locked --list > "$package_list"

while IFS= read -r packaged_file; do
    case "$packaged_file" in
        .cargo_vcs_info.json | Cargo.lock | Cargo.toml | Cargo.toml.orig | \
        CHANGELOG.md | LICENSE-APACHE | LICENSE-MIT | README.md | \
        examples/* | src/* | tests/* | themes/swatches.toml)
            ;;
        *)
            printf 'unexpected file in crate archive: %s\n' "$packaged_file" >&2
            exit 1
            ;;
    esac
done < "$package_list"

archive="target/package/swatches-0.1.0.crate"
test -f "$archive"
tar -xzf "$archive" -C "$work_dir"
package_root="$work_dir/swatches-0.1.0"
test -f "$package_root/themes/swatches.toml"
test ! -e "$package_root/integration"
test ! -e "$package_root/docs"

mkdir "$work_dir/consumer"
cat > "$work_dir/consumer/Cargo.toml" <<EOF
[package]
name = "swatches-package-consumer"
version = "0.0.0"
edition = "2021"
publish = false

[workspace]

[dependencies]
swatches = { path = "$package_root" }
EOF
mkdir "$work_dir/consumer/src"
cat > "$work_dir/consumer/src/main.rs" <<'EOF'
use swatches::{AppearancePatch, Theme};

fn main() -> Result<(), swatches::Error> {
    let theme = Theme::parse(include_str!("../../swatches-0.1.0/themes/swatches.toml"))?;
    let appearance = theme.appearance().with_overrides(&AppearancePatch::default());
    assert_eq!(appearance.accent.channels(), [128, 212, 255]);
    Ok(())
}
EOF
CARGO_TARGET_DIR="$work_dir/target" cargo run --manifest-path "$work_dir/consumer/Cargo.toml"

cargo publish --allow-dirty --locked --dry-run
