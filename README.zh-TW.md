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

🧠 完全跑在自己機器上的 AI agent 記憶儲存工具. 記下東西, 之後用語意找回來, 不必為每一筆記憶付錢給雲端服務.

其他語言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🚧 目前狀態

早期開發中. 下面列出的指令介面已經定案, 但底下的功能都還沒實作, 每個 subcommand 現在都會直接回報 not implemented. 儲存格式與 embedding 後端仍在設計, 在 `0.1.0` 之前都可能有破壞性變更.

## 📦 安裝

一個獨立的 binary, 從你手邊已經有的套件管理工具安裝即可.

```bash
cargo install amfs                    # Rust
npm install -g @mai0313/amfs          # Node.js
uv tool install agent-memory-fs       # Python
```

或者不安裝直接執行:

```bash
uvx --from agent-memory-fs amfs --help
```

> **套件名稱因 registry 而異, 但指令都一樣.** 不管從哪裡裝, 拿到的指令都是 `amfs`. 只有 crates.io 能用短名字: `amfs` 在 PyPI 上已經被另一個不相干的專案註冊, 而 npm 認為未加 scope 的 `amfs` 跟現有套件名太相似, 直接拒絕. 特別注意不要執行 `uvx amfs`, 那會裝到別人的套件.

macOS, Linux 與 Windows 的預先建置 binary 也附在每個 [release](https://github.com/Mai0313/amfs/releases) 裡.

## 🚀 使用方式

```bash
amfs add "Wei prefers Traditional Chinese in code reviews" --user-id wei

amfs search "what language does Wei want reviews in?" --user-id wei
amfs search "code review preferences" --limit 5

amfs list --user-id wei
amfs get <id>
amfs update <id> "Wei prefers Traditional Chinese, English for commit messages"
amfs delete <id>
```

完整的參數請看 `amfs --help` 或 `amfs <command> --help`.

## 🧭 運作方式

記憶存在本機的檔案裡. 搜尋時會把查詢字串轉成 embedding, 再跟已存的記憶比對, 所以 `search` 找的是意思相近的東西, 而不是剛好有相同字詞的東西.

Embedding 後端是可抽換的. 開發期間使用 Google Gemini 的 embedding model, 完全離線跑本地 model 是目標而非承諾, 細節請看[目前狀態](#-%E7%9B%AE%E5%89%8D%E7%8B%80%E6%85%8B).

## 🛠️ 開發

**系統需求:** Rust 1.95 以上 (專案使用 Edition 2024). toolchain 已經釘在 `rust-toolchain.toml`, `rustup` 會自動裝好對應版本.

```bash
make fmt            # rustfmt + clippy (先自動修, 再以 deny warnings 檢查)
make test           # 測試 (所有目標)
make test-verbose   # 測試 (所有目標與詳細輸出)
make coverage       # 產生 LCOV 覆蓋率報告
make build          # 建置 (debug)
make release        # 建置 (release, 鎖定依賴)
make run            # 執行 release binary
make clean          # 清理建置產物與快取
make package        # 建立 crate 套件 (允許 dirty)
make help           # 檢視可用目標
```

### 測試組織

專案遵循 Rust 官方的[測試組織慣例](https://doc.rust-lang.org/book/ch11-03-test-organization.html):

- **Unit tests**: 放在 `src/` 裡面, 跟被測試的程式碼擺在一起, 用 `#[cfg(test)] mod tests { ... }` 包起來, 可以存取 private items.
- **Integration tests**: 放在專案根目錄的 [tests/](tests/) 裡, 每個檔案會被編譯成獨立的 crate, 只能使用 public API:
    - [tests/cli.rs](tests/cli.rs) — 直接驅動編譯出來的 binary, 涵蓋參數解析, `--help` 與 exit code.
    - [tests/version.rs](tests/version.rs) — `build.rs` 在建置時注入的 version metadata.

執行所有測試: `make test` (或 `cargo test --all`).

### 版本資訊

`amfs --version` 會顯示動態的建置資訊: git tag (沒有 tag 時用 `Cargo.toml` 的版本), 自該 tag 以來的 commit 數, 簡短 commit hash, 工作目錄有未提交變更時的 `dirty` 標記, 以及建置時使用的 Rust 與 Cargo 版本. 這些都由 `build.rs` 在建置時嵌入.

```
amfs 0.1.25-2-gf4ae332-dirty
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t amfs:latest .
docker run --rm amfs:latest --help
```

映像也會推送到 `ghcr.io/mai0313/amfs`.

## 🧩 發行建置

`build_release.yml` 會在推送 `v*` tag 時建置各平台的 release binary, 上傳到 GitHub Release, 並發佈到 crates.io, npm 與 PyPI.

目標平台:

- x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu
- x86_64-apple-darwin, aarch64-apple-darwin
- x86_64-pc-windows-msvc, aarch64-pc-windows-msvc

資產命名:

- `amfs-v<version>-<platform>.tar.gz` (所有平台)
- `amfs-v<version>-<platform>.zip` (Windows 另附)

各 registry 的發行名稱寫死在 `build_release.yml` 裡, 不是從 repo 名推導出來的.

## 🔁 CI/CD

### 主要工作流程

- 測試 (`test.yml`): 建置與測試, 生成 LCOV 覆蓋率報告並上傳 artifact
- 品質 (`code-quality-check.yml`): pre-commit hooks + rustfmt 檢查 + clippy (拒絕警告)
- 建置與發行 (`build_release.yml`): 在 `v*` tag 建置多平台 binary, 發佈 GitHub Release 與 crates.io / npm / PyPI
- 映像 (`build_image.yml`): 在 `main` 與 `v*` tag 推送至 GHCR

### 其他自動化

- 自動標籤 (`auto_labeler.yml`): 依分支名稱與檔案變更自動為 PR 加標籤
- 程式碼掃描 (`code_scan.yml`): 多層安全掃描 (GitLeaks, Trufflehog, CodeQL)
- 發佈草稿 (`release_drafter.yml`): 自動生成 release notes
- 語義化 PR (`semantic-pull-request.yml`): 檢查 PR 標題格式
- Dependabot 依賴更新

## 🤝 貢獻

請看 [CONTRIBUTING.md](.github/CONTRIBUTING.md). 簡單說: PR 標題遵循 Conventional Commits, 送出 PR 前先在本機跑過

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## 📄 授權

MIT, 詳見 `LICENSE`.
