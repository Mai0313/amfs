<div align="center" markdown="1">

# AMFS — Agent Memory File System

[![Crates.io](https://img.shields.io/crates/v/amfs?logo=rust&style=flat-square&color=E05D44)](https://crates.io/crates/amfs)
[![Crates.io Downloads](https://img.shields.io/crates/d/amfs?logo=rust&style=flat-square)](https://crates.io/crates/amfs)
[![npm version](https://img.shields.io/npm/v/amfs?logo=npm&style=flat-square&color=CB3837)](https://www.npmjs.com/package/amfs)
[![npm downloads](https://img.shields.io/npm/dt/amfs?logo=npm&style=flat-square)](https://www.npmjs.com/package/amfs)
[![PyPI version](https://img.shields.io/pypi/v/agent-memory-fs?logo=python&style=flat-square&color=3776AB)](https://pypi.org/project/agent-memory-fs/)
[![PyPI downloads](https://img.shields.io/pypi/dm/agent-memory-fs?logo=python&style=flat-square)](https://pypi.org/project/agent-memory-fs/)
[![rust](https://img.shields.io/badge/Rust-stable-orange?logo=rust&logoColor=white&style=flat-square)](https://www.rust-lang.org/)
[![tests](https://img.shields.io/github/actions/workflow/status/Mai0313/amfs/test.yml?label=tests&logo=github&style=flat-square)](https://github.com/Mai0313/amfs/actions/workflows/test.yml)
[![code-quality](https://img.shields.io/github/actions/workflow/status/Mai0313/amfs/code-quality-check.yml?label=code-quality&logo=github&style=flat-square)](https://github.com/Mai0313/amfs/actions/workflows/code-quality-check.yml)
[![license](https://img.shields.io/badge/License-MIT-green.svg?labelColor=gray&style=flat-square)](https://github.com/Mai0313/amfs/tree/main?tab=License-1-ov-file)
[![PRs](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](https://github.com/Mai0313/amfs/pulls)

</div>

🧠 A memory store for AI agents that runs entirely on your own machine. Remember things, then find them back by meaning instead of by keyword — without paying a hosted service per memory.

Other Languages: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🚧 Status

Early development. The command surface below is settled, but nothing behind it is implemented yet — every subcommand currently exits with a "not implemented" error. The storage format and the embedding backends are still being designed, so expect breaking changes until `0.1.0`.

## 📦 Installation

A single self-contained binary, distributed through whichever package manager you already have.

```bash
cargo install amfs                    # Rust
npm install -g amfs                   # Node.js
uv tool install agent-memory-fs       # Python
```

Or run it without installing:

```bash
uvx --from agent-memory-fs amfs --help
```

> **Note on the PyPI name.** The `amfs` name was already taken on PyPI by an unrelated project, so the Python distribution is published as `agent-memory-fs`. The command it installs is still `amfs`. Do not run `uvx amfs` — that resolves to somebody else's package.

Prebuilt binaries for macOS, Linux, and Windows are also attached to every [release](https://github.com/Mai0313/amfs/releases).

## 🚀 Usage

```bash
amfs add "Wei prefers Traditional Chinese in code reviews" --user-id wei

amfs search "what language does Wei want reviews in?" --user-id wei
amfs search "code review preferences" --limit 5

amfs list --user-id wei
amfs get <id>
amfs update <id> "Wei prefers Traditional Chinese, English for commit messages"
amfs delete <id>
```

Run `amfs --help` or `amfs <command> --help` for the full set of flags.

## 🧭 How It Works

Memories live in a local file-backed store; searching embeds the query and compares it against the stored memories, so `search` finds things that mean the same thing rather than things that share a word.

Embedding backends are pluggable. Google's Gemini embedding model is the one used during development, and running fully offline against a local model is a goal, not a promise — see [Status](#-status).

## 🛠️ Development

**Requirements:** Rust 1.95 or newer (the project uses Edition 2024). The toolchain is pinned in `rust-toolchain.toml`, so `rustup` picks it up automatically.

```bash
make fmt            # rustfmt + clippy (auto-fix, then deny warnings)
make test           # cargo test (all targets)
make test-verbose   # cargo test (all targets with verbose output)
make coverage       # generate LCOV coverage report
make build          # cargo build (debug)
make release        # cargo build --release --locked
make run            # run the release binary
make clean          # clean build artifacts and caches
make package        # build crate package (allow dirty)
make help           # list targets
```

### Testing Layout

This project follows Rust's idiomatic [test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html):

- **Unit tests** live next to the code they verify inside `src/`, wrapped in a `#[cfg(test)] mod tests { ... }` block. They can exercise private items.
- **Integration tests** live in the top-level [tests/](tests/) directory. Each file is compiled as its own crate and may only use the public API:
    - [tests/cli.rs](tests/cli.rs) — drives the compiled binary: argument parsing, `--help`, exit codes.
    - [tests/version.rs](tests/version.rs) — build-time version metadata plumbed through `build.rs`.

Run everything with `make test` (or `cargo test --all`).

### Version Information

`amfs --version` reports dynamic build metadata: the git tag (or the `Cargo.toml` version when there is no tag), commits since that tag, the short commit hash, a `dirty` marker for uncommitted changes, plus the Rust and Cargo versions used to build it. All of it is embedded at build time by `build.rs`.

```
amfs 0.1.25-2-gf4ae332-dirty
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t amfs:latest .
docker run --rm amfs:latest --help
```

Images are also published to `ghcr.io/mai0313/amfs`.

## 🧩 Release Builds

`build_release.yml` builds release binaries on tags matching `v*`, uploads them to the GitHub Release, and publishes to crates.io, npm, and PyPI.

Targets:

- x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu
- x86_64-apple-darwin, aarch64-apple-darwin
- x86_64-pc-windows-msvc, aarch64-pc-windows-msvc

Asset naming:

- `amfs-v<version>-<platform>.tar.gz` (all platforms)
- `amfs-v<version>-<platform>.zip` (Windows additionally)

The three distribution names are declared once, at the top of `build_release.yml`.

## 🔁 CI/CD Workflows

### Main Workflows

- Tests (`test.yml`): cargo build/test + generate LCOV coverage report and upload artifact
- Code Quality (`code-quality-check.yml`): pre-commit hooks + rustfmt check + clippy (deny warnings)
- Build and Release (`build_release.yml`): multi-platform binaries, GitHub Release, crates.io / npm / PyPI publish on tags `v*`
- Publish Docker Image (`build_image.yml`): push to GHCR on `main` and tags `v*`

### Additional Automation

- Auto Labeler (`auto_labeler.yml`): automatically label PRs based on branch names and file changes
- Code Scan (`code_scan.yml`): multi-layer security scanning (GitLeaks, Trufflehog secret scanning, CodeQL code analysis)
- Release Drafter (`release_drafter.yml`): auto-generate release notes
- Semantic PR (`semantic-pull-request.yml`): enforce PR title format
- Dependabot dependency updates

## 🤝 Contributing

See [CONTRIBUTING.md](.github/CONTRIBUTING.md). In short: Conventional Commits for PR titles, and before opening a PR run

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## 📄 License

MIT — see `LICENSE`.
