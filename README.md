<div align="center" markdown="1">

# AMFS — Agent Memory File System

[![Crates.io](https://img.shields.io/crates/v/amfs?logo=rust&style=flat-square&color=E05D44)](https://crates.io/crates/amfs)
[![Crates.io Downloads](https://img.shields.io/crates/d/amfs?logo=rust&style=flat-square)](https://crates.io/crates/amfs)
[![npm version](https://img.shields.io/npm/v/@mai0313/amfs?logo=npm&style=flat-square&color=CB3837)](https://www.npmjs.com/package/@mai0313/amfs)
[![npm downloads](https://img.shields.io/npm/dt/@mai0313/amfs?logo=npm&style=flat-square)](https://www.npmjs.com/package/@mai0313/amfs)
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
npm install -g @mai0313/amfs          # Node.js
uv tool install agent-memory-fs       # Python
```

Or run it without installing:

```bash
uvx --from agent-memory-fs amfs --help
```

> **The package name differs per registry, the command does not.** Whichever one you install, you get `amfs`. Only crates.io could take the short name: `amfs` was already registered on PyPI by an unrelated project, and npm rejects the unscoped `amfs` as too similar to existing package names. In particular, do not run `uvx amfs` — that resolves to somebody else's package.

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

## 🐳 Docker

```bash
docker run --rm ghcr.io/mai0313/amfs:latest --help
```

Each release is also published under its own `v<version>` tag.

## 🛠️ Development

Contributor setup, tests, code conventions, CI, and the release process live in [CONTRIBUTING.md](./.github/CONTRIBUTING.md).

## 📄 License

MIT — see `LICENSE`.
