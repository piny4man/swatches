# Tablero private Blueprint adapter (NUKE-72)

## Revisions and private branch

- Public Tablero base: `cd92b4d40c7336d5dd88a826c221dbff44d5f974`, recorded in `base-revision`.
- Blueprint library/workflow base: `a552fa978dafa50a655a3ecde85d5c38c83e92fd` (merged NUKE-71); no library changes needed.
- Private review branch: `alberto/nuke-72-integrate-blueprint-themes-into-tablero` in `piny4man/blueprint`.
- `adapter.patch` includes the manifest, application lockfile, implementation, test source, and fixture. Apply it only to the pinned base. The local dependency points four levels up from `crates/tablero` to this private repo; patched Tablero has `publish = false`.

Use this integration's eventual reviewed Blueprint commit to reproduce its exact patch. The public Tablero repository and its default branch contain no Blueprint dependency. Publication/upstreaming remains a separate decision under [the private strategy](../../docs/private-integration.md).

## Prepare and build

From this private repository's root, with normal GitHub authentication:

```sh
bash integration/tablero/prepare.sh
cargo build --locked --manifest-path .work/tablero/Cargo.toml -p tablero
cargo test --locked --manifest-path .work/tablero/Cargo.toml -p tablero -- --test-threads=1
cargo clippy --locked --manifest-path .work/tablero/Cargo.toml -p tablero --all-targets -- -D warnings
cargo fmt --manifest-path .work/tablero/Cargo.toml --all --check
```

The preparation script rejects existing destinations rather than overwriting work. For another clean application, use `bash integration/tablero/prepare.sh tablero-verify` and substitute that name in the Cargo paths. It clones the public app, checks out the pinned SHA detached, disables its push URL, and checks/applies the private patch. All worktrees/build output remain ignored under `.work/`.

Native build requirements are Tablero's existing Linux dependencies: Rust/Cargo, a C toolchain, pkg-config, libclang for bindgen, Wayland, xkbcommon, PipeWire/SPA, and libudev development files. No extra native libraries are added. `--offline` works after the public Cargo dependencies are cached; first-time setup may need network access.

## Opt in

Add to the app's existing `$XDG_CONFIG_HOME/tablero/config.toml` (or `$HOME/.config/tablero/config.toml`):

```toml
[appearance]
theme_file = "~/dev/blueprint/themes/blueprint.toml"
```

Adjust that example to your private checkout. Absolute paths and `~/` are supported; other relative paths resolve against the app config's directory, never the launch directory. No environment-variable or `~user` expansion occurs. Symlink paths are preserved. `config.example.toml` beside this document is a minimal loadout whose relative theme path assumes it remains here; adjust the path when copying it elsewhere.

The executable uses the standard config location (there is no `--config` flag). For isolated visual testing, place an opt-in config at `<scratch>/tablero/config.toml` and run `XDG_CONFIG_HOME=<scratch> RUST_LOG=info .work/tablero/target/debug/tablero` in a Wayland session. Avoid running two status bars simultaneously during visual comparison.

Existing explicit colors/font family intentionally mask the shared values. Remove only the overrides you want to inherit. Removing `[appearance]` restores standalone resolution on the next valid save. An empty file during reload is rejected; use a nonempty document such as `height = 32` to deliberately return to defaults. Old public binaries reject the new section: remove it before switching back to an unpatched build.

## Mapping and precedence

| Blueprint v1 value | Tablero fallback |
| --- | --- |
| `colors.background` | `theme.background` |
| `colors.foreground` | `theme.foreground` |
| `colors.accent` | `theme.accent` |
| `font.family` | `font.family` |

The adapter inserts absent fields into the raw TOML before typed default filling. Precedence remains defaults → shared theme → explicit app theme/font → existing bar/widget/state/monitor specificity. An explicit value equal to the old default is still explicit. Shared RGB becomes opaque Tablero RGB; explicit RGBA overrides keep their alpha. Font size and geometry remain app-owned. Installed font lookup/fallback remains cosmic-text's existing behavior; theme parsing does not verify font installation.

Blueprint's `muted`, `selection_background`, and `selection_foreground` are validated as part of its complete v1 document but have no direct Tablero config targets in this adapter. Existing widget semantic colors (battery tiers/charging/warnings, power profiles, tray attention), workspace contrast calculations, and tray disabled/hover derivations remain application-owned. This is the scoped shared background/foreground/accent/font integration, not a complete semantic-state redesign.

`Config::from_toml_str` continues to work for standalone documents. Theme selection requires `load_from_path`/`load_for_reload`, providing the base directory explicitly instead of using an implicit working directory.

## Reload contract

- The existing 500ms timer polls metadata for app config, active theme, and any newly selected candidate theme. Metadata includes device/inode, size, mtime and ctime, catching atomic replacement, same-mtime saves, deletion/recreation, and symlink retargeting. File contents are read when needed, not on every unchanged tick.
- Each change must settle for 400ms. Both documents are fully parsed and validated, then their metadata is checked again before exposing a candidate to the UI. Startup-to-event-loop edits are also checked on the initial poll.
- Invalid/missing selected themes are startup errors. Bad reloads log once per observed edit and keep the complete previous UI/config; no popup is closed and no dashboard is rebuilt for a failed candidate.
- Selecting a nonexistent/invalid new theme keeps the old state while watching the new path too. Creating/fixing that theme recovers without editing app config again. Successful selection switches discard the old dependency; removing opt-in stops theme watching.
- A changed valid config uses the existing all-output reload path, closes tooltip/tray surfaces and pending tray requests, rebuilds per-output dashboards, replays latest producer snapshots (including per-monitor titles), and adds a fresh clock tick. Identical effective configs do not rebuild.
- Producer processes retain their existing lifecycle. Newly enabled producer-backed modules may require a restart; theme edits themselves require none.

## Tooltip and tray review

Tooltip and locally rendered tray menus receive each output's resolved render settings, including the shared font and theme colors. An accepted reload closes existing popups; reopen/hover again to see refreshed settings and measurements. Invalid edits leave the current popups intact. Pending tray menu state is cleared on accepted reload so a late response cannot reopen the old menu.

Existing popup opacity rules remain: a transparent bar uses the opaque fallback background; otherwise popup alpha is at least `0xF0`. Bar background overrides therefore affect popups too. External tray-owned windows/context menus and supplied tray icon artwork are not recolored by Tablero. Attention/disabled/hover colors follow the existing application rules described above.

### Desktop checks still required before merge

1. Run the private build with opt-in and test actual font availability/readability.
2. Edit only the shared theme's background/foreground/accent/font; confirm all intended outputs update, including 1x and scaled monitors, while explicit overrides remain.
3. Open a tooltip and tray menu, save a valid theme, and reopen them. Check updated colors/font, bounds, opacity, and click targets. Repeat with a bad save and confirm current UI stays intact.
4. Switch to a missing theme, create it, and confirm recovery. Test an editor's atomic-save workflow and removal of opt-in.
5. Check workspace/title/volume/tray content remains after reload without waiting for producers to emit again.

## Recorded automated validation — 9 September 2026

- Fresh public clone at pinned SHA: `git apply --check` and patch application passed via `prepare.sh tablero-verify`.
- Fresh patched app: `cargo build --offline --locked ... -p tablero` passed.
- Fresh patched suite: **593 tests passed**, including 509 library tests, integration/binary tests and a doctest, with `--test-threads=1`.
- New coverage: no-theme parity, explicit-old-default preservation, RGBA/bar/widget/state/monitor/font precedence, source diagnostics, theme-only output pixels, snapshot replay, atomic replacement, same-mtime edit, symlink retargeting, deletion/recreation, bad-save recovery and failed new-theme selection recovery.
- Clippy across all Tablero targets with `-D warnings`: passed. Formatting and diff whitespace checks: passed. Shell syntax validation for `prepare.sh`: passed.
- A parallel verification run hit the existing `command::tests::spawn_run_program_expands_tilde_before_preflight` test with Linux `Text file busy` while spawning a newly written script. The complete serial rerun passed; no unrelated command code was changed.
- Separate unpatched public clone at the same SHA: locked offline build passed; tracked files remained clean. Original `/home/piny4/dev/tablero` remains clean on `main`.
- Environment: Rust 1.98.1; pkg-config versions Wayland 1.26.0, PipeWire 1.6.8, libudev 261, xkbcommon 1.13.2.

Automated renderer/config tests do not substitute for the live Wayland/desktop checks above. No running desktop configuration was changed, no installation was performed, and no live GUI verification is claimed.

## Refreshing the private patch

Edit only the ignored patched checkout. Mark any new intended files with `git add -N <paths>` there, then regenerate using `git diff --binary --output=<absolute-blueprint-root>/integration/tablero/adapter.patch` from that checkout. Inspect the complete file inventory, run `git diff --check`, and repeat clean preparation/build verification. Do not change `base-revision` without deliberately rebasing and revalidating the adapter.
