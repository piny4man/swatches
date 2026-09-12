# NUKE-70 review

The existing desktop tools express appearance differently. Swatches provides a stable input contract without coupling their renderers or overriding their standalone defaults.

This foundation includes complete version-1 themes, strict RGB/font parsing, source-aware file errors, a typed explicit-override helper, deterministic path resolution, an example theme, and contract tests. Publication is disabled. This NUKE-70 record predates the rename from Blueprint to Swatches.

Intentional changes from the initial audit proposal: the schema uses `version = 1` as agreed in the subsequent design discussion; muted and selection colors are explicit rather than derived. Font size, geometry, alpha and motion are deferred.

Follow-up PRs should separately integrate Tablero, Hyprburst and Crabture. No existing app configs are changed by this package. Runtime reload, font discovery and rendering belong to those adapters.

The GitHub repository is private. No license or registry publication should be added without deciding those separately. This foundation is proposed through a draft PR for NUKE-70.

## Verification

Verified with Rust 1.98.1 on Linux:

- `cargo test`: 9 integration tests and 1 documentation test passed.
- `cargo clippy --all-targets --locked -- -D warnings`: passed.
- `cargo fmt --check`: passed.

Cargo.lock is included. No GUI integration or laptop performance test has been run. Registry name availability could not be verified (the registry API request returned HTTP 403); nothing has been published or reserved.
