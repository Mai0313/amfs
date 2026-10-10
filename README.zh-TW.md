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

## 🐳 Docker

```bash
docker run --rm ghcr.io/mai0313/amfs:latest --help
```

每個 release 也會以自己的 `v<version>` tag 發佈.

## 🛠️ 開發

開發環境設定, 測試, 程式碼慣例, CI 與發行流程都寫在 [CONTRIBUTING.md](./.github/CONTRIBUTING.md).

## 📄 授權

MIT, 詳見 `LICENSE`.
