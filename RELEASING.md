# Releasing Swatches

This document is operational and is intentionally excluded from the public
crate archive.

## Distribution decisions

- Version 0.1.0 is licensed under `MIT OR Apache-2.0`. Both complete license
  texts are included in the package.
- Cargo metadata points to `https://github.com/piny4man/swatches`. The
  repository is public. Application adapters belong in the consuming
  applications, not in this repository.
- API documentation is hosted by docs.rs from the crates.io package.
- `cargo info swatches --registry crates-io` and the crates.io API found no
  crate with the exact name on 2026-09-12. Recheck immediately before release.
- The explicit Cargo include list excludes `.work/`, `.github/`, local build
  output, and this operational document.

## Attribution review

Reviewed for 0.1.0 on 2026-09-12. The package contains the Swatches source,
tests, generic example, example theme, README, changelog, and license texts. It
does not bundle third-party source, fonts, artwork, or application patches.
Dependencies are referenced through crates.io rather than redistributed. No
additional `NOTICE` attribution is currently required. Repeat this review when
package contents or dependencies change.

## Release checklist

1. Confirm `https://github.com/piny4man/swatches` is publicly readable.
2. Confirm the `swatches` name is still available with `cargo search swatches`
   and the crates.io website. Do not publish an empty reservation package.
3. Replace `Unreleased` in `CHANGELOG.md` with the release date.
4. Run `cargo +1.85 test --locked --all-targets` to verify the minimum supported
   Rust version.
5. Run `./scripts/check-release.sh` on stable Rust.
6. Review `target/package/swatches-0.1.0.crate` and
   `target/package/swatches-0.1.0/` after Cargo's package verification.
7. Review the generated rustdoc, README links, metadata, both licenses, and this
   attribution assessment.
8. Commit and obtain review. This repository's user merges pull requests
   manually.
9. Only in the separate release task, run `cargo publish --locked` and verify a
   fresh consumer using `swatches = "0.1.0"` from crates.io.

`cargo publish` is intentionally outside NUKE-95.
