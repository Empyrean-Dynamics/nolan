//! Guards that every crate-version reference in `README.md` agrees with the
//! crate's actual version (`CARGO_PKG_VERSION`, i.e. the `version` field in
//! `Cargo.toml`).
//!
//! The published crate is `hyperjet`; the README shows two install snippets —
//! the bare dependency (`hyperjet = "X.Y.Z"`) and the internal alias
//! (`nolan = { package = "hyperjet", version = "X.Y.Z" }`). Both hardcode a
//! version string that a version bump can silently leave stale (the README was
//! caught pinned to 1.11 while the crate had already moved to 1.13).
//!
//! CI already checks Cargo.toml against CITATION.cff (`rust.yml`) and the tag
//! against both (`release.yml`), but nothing checked the README. This test
//! closes that gap and runs in the normal `cargo test` CI leg — no separate
//! shell step is needed.
//!
//! Dependency version requirements are matched at `major.minor` precision: a
//! Cargo requirement of `"1.13"` or `"1.13.0"` both accept 1.13.x, so a patch
//! bump need not touch the README, but a major or minor bump must.

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;

    /// Reduce a dotted version string to its `major.minor` prefix, e.g.
    /// `"1.13.0"` -> `"1.13"`, `"1.13"` -> `"1.13"`. Panics on a string with
    /// no minor component so a malformed reference fails loudly rather than
    /// passing silently.
    fn major_minor(version: &str) -> String {
        let mut parts = version.split('.');
        let major = parts.next().unwrap_or_default();
        let minor = parts
            .next()
            .unwrap_or_else(|| panic!("version {version:?} has no minor component"));
        format!("{major}.{minor}")
    }

    /// Return the contents of the quoted string immediately following the
    /// first occurrence of `needle` (which must end in a `"`) on `line`.
    fn quoted_after<'a>(line: &'a str, needle: &str) -> Option<&'a str> {
        let start = line.find(needle)? + needle.len();
        let rest = &line[start..];
        let end = rest.find('"')?;
        Some(&rest[..end])
    }

    #[test]
    fn readme_dependency_versions_match_crate_version() {
        let readme_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("README.md");
        let readme = fs::read_to_string(&readme_path)
            .unwrap_or_else(|e| panic!("failed to read {}: {e}", readme_path.display()));

        let crate_version = env!("CARGO_PKG_VERSION");
        let want = major_minor(crate_version);

        // (human-readable label, extracted version) for every crate version
        // reference found in the README.
        let mut refs: Vec<(&str, &str)> = Vec::new();

        for line in readme.lines() {
            // Bare dependency form: `hyperjet = "X.Y.Z"`.
            if let Some(v) = quoted_after(line.trim_start(), "hyperjet = \"") {
                refs.push(("hyperjet = \"…\"", v));
            }

            // Aliased dependency form:
            // `nolan = { package = "hyperjet", version = "X.Y.Z" }`.
            if line.contains("package = \"hyperjet\"")
                && let Some(v) = quoted_after(line, "version = \"")
            {
                refs.push(("nolan = { package = \"hyperjet\", version = \"…\" }", v));
            }
        }

        assert!(
            !refs.is_empty(),
            "no crate version references found in {} — the extraction patterns \
             have drifted from the README's install snippets; update this test",
            readme_path.display()
        );

        for (label, found) in &refs {
            assert_eq!(
                major_minor(found),
                want,
                "README version reference `{label}` is \"{found}\" (major.minor \
                 {}) but the crate version is \"{crate_version}\" (major.minor \
                 {want}); update README.md to match Cargo.toml",
                major_minor(found)
            );
        }
    }
}
