# Contributing Guide

Thank you for your interest in contributing to this Rust project. This document describes how to set up the development environment, the conventions used by the project, and the workflow expected for issues and pull requests.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Ways to Contribute](#ways-to-contribute)
- [Reporting Issues](#reporting-issues)
- [Development Setup](#development-setup)
- [Local Workflow](#local-workflow)
- [Testing](#testing)
- [Branching Model](#branching-model)
- [Commit Convention](#commit-convention)
- [Pull Request Process](#pull-request-process)
- [Code Review](#code-review)
- [Coding Standards](#coding-standards)
- [CI and Releases](#ci-and-releases)
- [Security Reports](#security-reports)
- [Licensing](#licensing)

## Code of Conduct

All contributors are expected to behave professionally and respectfully. Personal attacks, harassment, and discriminatory language are not tolerated. By participating, you agree to uphold a welcoming environment for everyone.

## Ways to Contribute

- Reporting bugs and reproducible issues
- Proposing or implementing new features
- Improving documentation, examples, and tutorials
- Reviewing pull requests and providing constructive feedback
- Suggesting tooling, performance, or security improvements

## Reporting Issues

Before opening a new issue:

1. Search existing issues to avoid duplicates.
2. Confirm the problem reproduces on the latest release or `main`.
3. Use the appropriate issue template.

Please include:

- A clear, descriptive title
- Rust toolchain version (`rustc --version`), OS, and architecture
- Project version, commit hash, or release tag
- Minimal reproduction steps, ideally a small Cargo project
- Expected vs. actual behavior
- Full stack traces, panics, or compiler output

## Development Setup

This project pins the Rust toolchain via `rust-toolchain.toml`. With [`rustup`](https://rustup.rs/) installed, the correct toolchain is selected automatically.

```bash
# Install rustup (one-time setup)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone your fork
git clone https://github.com/<your-username>/<repo>.git
cd <repo>

# Verify the toolchain (rustup will install the pinned version on first build)
rustc --version
cargo --version

# Build the project
cargo build
```

## Local Workflow

Common tasks are exposed via the `Makefile`. Run `make help` to list all targets. Frequently used ones:

```bash
make fmt       # Run rustfmt and clippy --fix, then deny warnings
make build     # Build debug binary
make release   # Build release binary
make test      # Run the test suite
make package   # Produce a publishable .crate
make clean     # Remove build artifacts and caches
```

Recommended commands directly via `cargo`:

```bash
cargo fmt --all                                 # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint with warnings as errors
cargo test --all                                # Run all tests
cargo doc --no-deps --open                      # Build and view documentation
```

Always run `make fmt` and `make test` before opening a pull request.

Build and run the Docker image locally:

```bash
docker build -f docker/Dockerfile --target prod -t amfs:latest .
docker run --rm amfs:latest --help
```

## Testing

The layout follows the Rust Book's [test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html):

- Unit tests live in `#[cfg(test)] mod tests` blocks within source files and can exercise private items.
- Integration tests live under `tests/`. Each file is compiled as its own crate and may only use the public API.
- Documentation tests are encouraged for public APIs.
- New behavior must be covered by tests. Bug fixes should include a regression test.

Useful commands:

```bash
cargo test --all                                # Run all tests
cargo test --test <integration_test>            # Run a specific integration test
cargo test --doc                                # Run only documentation tests
cargo test -- --nocapture                       # Show output from passing tests
cargo bench                                     # Run benchmarks (if defined)
```

## Branching Model

- `main` is the default branch and must always be releasable.
- Feature branches: `feat/<short-description>`
- Bug fix branches: `fix/<short-description>`
- Documentation branches: `docs/<short-description>`

## Commit Convention

Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/) and **must be written in English**.

Format:

```
<type>(<optional scope>): <short summary>

<optional body>

<optional footer>
```

Allowed types:

| Type       | Purpose                                                 |
| ---------- | ------------------------------------------------------- |
| `feat`     | A new feature                                           |
| `fix`      | A bug fix                                               |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `docs`     | Documentation-only changes                              |
| `perf`     | Performance improvement                                 |
| `style`    | Formatting or stylistic changes                         |
| `test`     | Adding or correcting tests                              |
| `chore`    | Build, tooling, or auxiliary changes                    |
| `ci`       | Continuous integration changes                          |
| `revert`   | Reverting a previous commit                             |

Append `!` after the type or include `BREAKING CHANGE:` in the footer to indicate a breaking change. Reference issues with `Closes #123` or `Refs #123`.

## Pull Request Process

1. Ensure your branch is up to date with the target branch.
2. Run `make fmt` and `make test` locally; both must pass.
3. Ensure CI checks pass on the pull request.
4. Use a descriptive title following the commit convention; it is validated by **semantic-pull-request**.
5. Fill out the pull request template, including motivation, summary, and testing notes.
6. Link related issues and design documents.
7. Mark the PR as **draft** while still in progress.
8. Request review only after self-review and a green CI.

Pull requests are typically merged via **squash merge** to keep history linear.

## Code Review

- Address all review comments or explain why a change is not needed.
- Keep discussions technical, focused, and respectful.
- Resolve conversations only after the concern has been addressed.

## Coding Standards

- **Formatting**: enforced by `rustfmt` (`cargo fmt --all`)
- **Linting**: `clippy` must pass with `-D warnings`
- **Documentation**: every public item should carry a `///` doc comment with examples where appropriate
- **Errors**: prefer `Result<T, E>` and concrete error types (`thiserror`, `anyhow`); avoid `unwrap()` / `expect()` in library code
- **Unsafe**: any `unsafe` block must be accompanied by a `// SAFETY:` comment justifying the invariants
- **MSRV**: `rust-version` in `Cargo.toml`; do not raise the minimum supported Rust version without discussion

Prefer clarity over cleverness, and avoid unrelated refactors in feature or fix pull requests.

## CI and Releases

CI workflows live in [`.github/workflows/`](./workflows/).

Pushing a `v*` tag runs `build_release.yml`: it builds the release binaries for every target in its matrix, uploads them to the GitHub Release, and publishes to crates.io, npm, and PyPI. Distribution names are pinned in `build_release.yml`, not derived from the repository name. `build_image.yml` pushes the Docker image to `ghcr.io/mai0313/amfs` on `main` and on `v*` tags.

`amfs --version` reports build metadata embedded by `build.rs`: the git tag (or the `Cargo.toml` version when there is no tag), commits since that tag, the short commit hash, and a `dirty` marker for uncommitted changes.

```
amfs 0.1.25-2-gf4ae332-dirty
```

## Security Reports

Please **do not** report security vulnerabilities through public issues. Refer to [`SECURITY.md`](./SECURITY.md) for the responsible disclosure process.

## Licensing

By contributing, you agree that your contributions will be licensed under the project's license (see [`LICENSE`](../LICENSE)). Ensure that you have the right to submit any code, content, or assets you contribute.
