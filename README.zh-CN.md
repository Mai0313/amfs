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

🧠 完全跑在自己机器上的 AI agent 记忆存储工具. 记下东西, 之后用语意找回来, 不必为每一笔记忆付钱给云端服务.

其他语言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🚧 当前状态

早期开发中. 下面列出的命令界面已经定案, 但底下的功能都还没实现, 每个 subcommand 现在都会直接报告 not implemented. 存储格式与 embedding 后端仍在设计, 在 `0.1.0` 之前都可能有破坏性变更.

## 📦 安装

一个独立的 binary, 从你手边已经有的包管理工具安装即可.

```bash
cargo install amfs                    # Rust
npm install -g @mai0313/amfs          # Node.js
uv tool install agent-memory-fs       # Python
```

或者不安装直接执行:

```bash
uvx --from agent-memory-fs amfs --help
```

> **包名称因 registry 而异, 但命令都一样.** 不管从哪里装, 拿到的命令都是 `amfs`. 只有 crates.io 能用短名字: `amfs` 在 PyPI 上已经被另一个不相干的项目注册, 而 npm 认为未加 scope 的 `amfs` 跟现有包名太相似, 直接拒绝. 特别注意不要执行 `uvx amfs`, 那会装到别人的包.

macOS, Linux 与 Windows 的预编译 binary 也附在每个 [release](https://github.com/Mai0313/amfs/releases) 里.

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

完整的参数请看 `amfs --help` 或 `amfs <command> --help`.

## 🧭 运作方式

记忆存在本机的文件里. 搜索时会把查询字符串转成 embedding, 再跟已存的记忆比对, 所以 `search` 找的是意思相近的东西, 而不是刚好有相同字词的东西.

Embedding 后端是可替换的. 开发期间使用 Google Gemini 的 embedding model, 完全离线跑本地 model 是目标而非承诺, 细节请看[当前状态](#-%E5%BD%93%E5%89%8D%E7%8A%B6%E6%80%81).

## 🐳 Docker

```bash
docker run --rm ghcr.io/mai0313/amfs:latest --help
```

每个 release 也会以自己的 `v<version>` tag 发布.

## 🛠️ 开发

开发环境配置, 测试, 代码规范, CI 与发行流程都写在 [CONTRIBUTING.md](./.github/CONTRIBUTING.md).

## 📄 许可证

MIT, 详见 `LICENSE`.
