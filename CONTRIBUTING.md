# Contributing to nolan

nolan (published on crates.io as
[`hyperjet`](https://crates.io/crates/hyperjet)) welcomes contributions:
bug reports, feature requests, documentation improvements, and pull
requests.

## Filing issues

- **Bugs** — [open an issue](https://github.com/Empyrean-Dynamics/nolan/issues).
  Include a minimal reproducer (a small `Jet<N,...>` example with the unexpected
  output is ideal), the platform, and the version (`hyperjet::version()`).
- **Feature requests** — same tracker. Numerical-methods proposals should
  cite the reference (paper, textbook) they're drawn from.
- **Questions and discussion** —
  [GitHub Discussions](https://github.com/Empyrean-Dynamics/nolan/discussions).

## Pull requests

For anything beyond a small fix or a documentation tweak, open an issue
first so we can discuss the approach. It's easier to talk through direction
before code than to unwind a wrong direction at review.

Before submitting, verify locally:

```
cargo fmt -- --check
cargo clippy --all-targets --all-features
cargo test
```

CI also runs `cargo publish --dry-run` on every PR to catch packaging
breakage (missing files in `include = `, broken manifest) before tag
time. If you add files that need to ship with the published crate —
e.g., a new `docs/` asset referenced from rustdoc — add them to the
`include = [...]` list in `Cargo.toml`.

The minimum supported Rust version is pinned in `Cargo.toml` (`rust-version`).
Use the stable toolchain.

## AI-assisted contributions

We accept code written with AI assistance including code that is largely
or entirely AI-generated. 

**Disclosure is mandatory.** If an AI model contributed materially to a
commit, the commit message must include a `Co-Authored-By:` trailer naming
each model. Git recognises this trailer and GitHub renders it on the
commit page so the provenance is visible to reviewers and downstream
users.

Example (single model):

```
Add Jet2 differentiate1 convenience wrapper

[body…]

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
```

Example (multiple models):

```
Refactor mat_solve to use scaled partial pivoting

[body…]

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
Co-Authored-By: GPT-5 <noreply@openai.com>
```

A few clarifications on what the disclosure does and doesn't require:

- **Material contribution.** Routine line-by-line autocomplete (the
  editor finishing a line you were already typing) doesn't require a
  trailer. If a model wrote a function body, designed an algorithm,
  drafted a docstring, or proposed an approach you adopted, that counts
  as material.
- **Name the actual model, not the wrapper.** `Claude Opus 4.7`, not
  `Cursor`. `GPT-5`, not `Copilot` or `Aider`. If multiple models
  contributed to one commit, add one trailer per model.
- **Include the model version.** "Claude" or "GPT" alone isn't enough —
  models change behaviour across versions, and the version is the part
  that's actually load-bearing for reproducibility.
- **You remain the human author.** The `Author:` field is you. The
  trailer signals "an AI model also worked on this." You are
  responsible for the contribution's correctness, for verifying that
  the AI-generated code does what it claims, and for ensuring the
  contribution can be licensed under BSD-3-Clause (see License below).

If you're unsure whether a particular use of AI assistance crosses the
"material" threshold, err on the side of adding the trailer. Over-
disclosure costs nothing; under-disclosure is what we're trying to
avoid.

## License

nolan is BSD-3-Clause. By submitting a contribution you agree to license
it under the same terms, and you confirm that you have the right to do
so — including for any AI-generated portions of the contribution.
