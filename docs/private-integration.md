# Private integration strategy (NUKE-71)

## Decision

Keep Swatches and adapter patches in this private repository. Build patched applications in disposable, ignored checkouts under `.work/`. Keep public application branches and their release dependency graphs unchanged during this phase.

Each integration PR targets this private repository and contains the application base revision, adapter patch, lockfile changes and validation notes. The user merges manually. No extra private forks or new repository are needed to start.

Swatches continues to use `publish = false`. Patched application manifests must also disable publication. Nothing in this strategy publishes a crate, changes repository visibility, or uploads application binaries.

## Private checkout and local dependency

Clone this private repo using the owner's normal GitHub authentication (for example `gh repo clone piny4man/swatches`). Credentials stay with Git/gh; never place tokens in Cargo.toml, a URL, or a patch.

Use the reviewed Swatches commit for a reproducible build. A local path dependency then avoids registry publication and extra private Git fetching during Cargo resolution. Record the Swatches commit, application base SHA, applied patch and application Cargo.lock together in each integration's validation notes.

There is no public registry fallback for `swatches`. A missing local checkout is an actionable setup error. An optional Cargo feature would not make an inaccessible private dependency an acceptable public manifest dependency: dependency resolution/packaging still needs consideration. We therefore keep the entire adapter change private for now.

## Disposable app layout

From the Swatches root, check out an app at the full SHA recorded in its integration record. For example:

```sh
mkdir -p .work
git clone --no-checkout https://github.com/piny4man/tablero.git .work/tablero
git -C .work/tablero checkout --detach "$(cat integration/tablero/base-revision)"
git -C .work/tablero remote rename origin upstream
git -C .work/tablero remote set-url --push upstream DISABLED
git -C .work/tablero apply --check ../../integration/tablero/adapter.patch
git -C .work/tablero apply ../../integration/tablero/adapter.patch
cargo build --locked --manifest-path .work/tablero/Cargo.toml -p tablero
```

The example requires a fresh `.work/tablero` path. Do not delete an existing checkout containing work. The disabled push URL is a convenience safeguard, not an access-control boundary. Never push private adapter branches or patches to a public repo, gist, issue or PR.

The `.work` layout makes relative dependencies stable:

| App manifest | Swatches dependency path |
|---|---|
| `.work/tablero/crates/tablero/Cargo.toml` | `../../../..` |
| `.work/hyprburst/Cargo.toml` | `../..` |
| `.work/crabture/Cargo.toml` | `../..` |

The adapter patch adds a dependency such as:

```toml
swatches = { path = "../../../.." }
```

That line belongs under the owning crate's dependencies, with `publish = false` in its package table. Preserve all unrelated upstream metadata and licenses. This path exists only in the private integration checkout and must not enter a public release manifest.

## Patch records and updates

For each app, store `integration/<app>/base-revision`, `adapter.patch` and `README.md`. Include new source files and changes to Cargo.lock in the patch. Review the diff against the pinned base; patches that contain only tracked-file edits can accidentally omit new files, so explicitly check the file inventory before recording a patch.

Validate `git apply --check` and a locked build from a fresh checkout. Compare upstream diffs on base-version updates and deliberately regenerate the patch; do not apply with automatic conflict guessing. Record native library requirements separately. Do not commit `.work/`, build caches, generated binaries or local credentials.

The [Tablero integration](../integration/tablero/README.md) is supplied by NUKE-72. Hyprburst and Crabture records will be created by NUKE-73 and NUKE-74. NUKE-71 proves the dependency/build arrangement with the separate consumer below, not the app adapters.

## Executable proof

`integration/private-consumer` is an independent Cargo project with its own lockfile and `publish = false`. It imports this private library as `swatches`, loads the theme from disk, and checks shared appearance, explicit override precedence and standalone defaults.

From the Swatches root:

```sh
cargo run --locked --manifest-path integration/private-consumer/Cargo.toml
cargo clippy --locked --manifest-path integration/private-consumer/Cargo.toml -- -D warnings
cargo fmt --manifest-path integration/private-consumer/Cargo.toml --check
```

Add `--offline` once dependencies are cached to demonstrate that Cargo does not need private Git or registry publication for Swatches. A fresh machine still needs ordinary public dependencies from crates.io. The consumer is intentionally not a GUI app and does not prove font installation, rendering or laptop performance.

## Preserving public builds

During this issue no application source, manifest, lockfile or public branch is changed. The existing standalone public build remains based on its existing dependencies. That preservation is structural; it is not a claim that a full public GUI build was executed here.

Adapter PRs must demonstrate that all their dependency changes live in the private patch and that no public branch is modified. Testing happens locally; do not configure public CI to fetch the private library. Private CI can be considered later, with access configured deliberately.

## Later upstream integration

Before moving an adapter upstream, decide separately whether to publish an appropriately licensed Swatches crate, expose a public interchange interface, or continue keeping integrations private. Verify the registry name and licensing only when publication is actually chosen. Existing GPL application licensing is preserved and requires consideration before distributing modified builds.

Moving an adapter upstream requires a reproducible dependency source accessible to public contributors, normal public CI and package checks, documented configuration migration, and user authorization for any private source disclosure. The current roadmap does not authorize publication or a visibility change.

## Rename validation - 12 September 2026

- The `swatches` package and `swatches-private-consumer` metadata report publishing disabled.
- The consumer's locked build and run, Clippy with warnings denied, and formatting check passed. Runtime output confirmed shared accent `#80D4FF`, explicit accent `#010203`, and standalone defaults preserved.
- Cargo metadata reports `swatches` as a local path dependency with no registry or Git source.

## Historical validation - 8 September 2026

Validated 8 September 2026 on Linux with Rust 1.98.1 against the project's merged
foundation commit `3166fb054a595fb9a86443d225b428a64f8eadb4`, when it was named Blueprint (library source unchanged).

- Separate consumer `cargo run --offline --locked`: passed; output confirms shared accent `#80D4FF`, explicit accent `#010203`, and standalone defaults preserved.
- Consumer Clippy with `--offline --locked` and warnings denied: passed.
- Consumer formatting check: passed.
- Cargo metadata reports a local path dependency with no registry/Git source for the then-named Blueprint package and publishing disabled for the consumer.
- Local Tablero, Hyprburst and Crabture tracked trees match their checked-out HEADs; no public repository writes were made.

This validates the independent consumer path. App patch application, full native builds and runtime desktop behavior will be validated in NUKE-72 through NUKE-74. A fresh private `gh` clone on the user's laptop has not been tested here.
