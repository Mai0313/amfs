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

🧠 完全跑在自己机器上的 AI agent 记忆存储工具. 记下东西, 之后用语意找回来, 不必为每一笔记忆付钱给云端服务.

其他语言: [English](README.md) | [繁體中文](README.zh-TW.md) | [简体中文](README.zh-CN.md)

## 🚧 当前状态

早期开发中. 下面列出的命令界面已经定案, 但底下的功能都还没实现, 每个 subcommand 现在都会直接报告 not implemented. 存储格式与 embedding 后端仍在设计, 在 `0.1.0` 之前都可能有破坏性变更.

## 📦 安装

一个独立的 binary, 从你手边已经有的包管理工具安装即可.

```bash
cargo install amfs                    # Rust
npm install -g amfs                   # Node.js
uv tool install agent-memory-fs       # Python
```

或者不安装直接执行:

```bash
uvx --from agent-memory-fs amfs --help
```

> **PyPI 名称的注意事项.** `amfs` 在 PyPI 上已经被另一个不相干的项目占用, 所以 Python 发行版的名字是 `agent-memory-fs`, 但装好之后的命令仍然是 `amfs`. 请不要执行 `uvx amfs`, 那会装到别人的包.

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

## 🛠️ 开发

**系统需求:** Rust 1.95 以上 (项目使用 Edition 2024). toolchain 已经钉在 `rust-toolchain.toml`, `rustup` 会自动装好对应版本.

```bash
make fmt            # rustfmt + clippy (先自动修, 再以 deny warnings 检查)
make test           # 测试 (所有目标)
make test-verbose   # 测试 (所有目标与详细输出)
make coverage       # 生成 LCOV 覆盖率报告
make build          # 构建 (debug)
make release        # 构建 (release, 锁定依赖)
make run            # 运行 release binary
make clean          # 清理构建产物与缓存
make package        # 构建 crate 包 (允许 dirty)
make help           # 查看可用目标
```

### 测试组织

项目遵循 Rust 官方的[测试组织惯例](https://doc.rust-lang.org/book/ch11-03-test-organization.html):

- **Unit tests**: 放在 `src/` 里面, 跟被测试的代码摆在一起, 用 `#[cfg(test)] mod tests { ... }` 包起来, 可以访问 private items.
- **Integration tests**: 放在项目根目录的 [tests/](tests/) 里, 每个文件会被编译成独立的 crate, 只能使用 public API:
    - [tests/cli.rs](tests/cli.rs) — 直接驱动编译出来的 binary, 涵盖参数解析, `--help` 与 exit code.
    - [tests/version.rs](tests/version.rs) — `build.rs` 在构建时注入的 version metadata.

运行所有测试: `make test` (或 `cargo test --all`).

### 版本信息

`amfs --version` 会显示动态的构建信息: git tag (没有 tag 时用 `Cargo.toml` 的版本), 自该 tag 以来的 commit 数, 简短 commit hash, 工作目录有未提交变更时的 `dirty` 标记, 以及构建时使用的 Rust 与 Cargo 版本. 这些都由 `build.rs` 在构建时嵌入.

```
amfs 0.1.25-2-gf4ae332-dirty
```

## 🐳 Docker

```bash
docker build -f docker/Dockerfile --target prod -t amfs:latest .
docker run --rm amfs:latest --help
```

镜像也会推送到 `ghcr.io/mai0313/amfs`.

## 🧩 发行构建

`build_release.yml` 会在推送 `v*` tag 时构建各平台的 release binary, 上传到 GitHub Release, 并发布到 crates.io, npm 与 PyPI.

目标平台:

- x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu
- x86_64-apple-darwin, aarch64-apple-darwin
- x86_64-pc-windows-msvc, aarch64-pc-windows-msvc

资产命名:

- `amfs-v<version>-<platform>.tar.gz` (所有平台)
- `amfs-v<version>-<platform>.zip` (Windows 另附)

三个 registry 的发行名称统一声明在 `build_release.yml` 的开头.

## 🔁 CI/CD

### 主要工作流程

- 测试 (`test.yml`): 构建与测试, 生成 LCOV 覆盖率报告并上传 artifact
- 质量 (`code-quality-check.yml`): pre-commit hooks + rustfmt 检查 + clippy (拒绝警告)
- 构建与发行 (`build_release.yml`): 在 `v*` tag 构建多平台 binary, 发布 GitHub Release 与 crates.io / npm / PyPI
- 镜像 (`build_image.yml`): 在 `main` 与 `v*` tag 推送至 GHCR

### 其他自动化

- 自动标签 (`auto_labeler.yml`): 依分支名称与文件变更自动为 PR 加标签
- 代码扫描 (`code_scan.yml`): 多层安全扫描 (GitLeaks, Trufflehog, CodeQL)
- 发布草稿 (`release_drafter.yml`): 自动生成 release notes
- 语义化 PR (`semantic-pull-request.yml`): 检查 PR 标题格式
- Dependabot 依赖更新

## 🤝 贡献

请看 [CONTRIBUTING.md](.github/CONTRIBUTING.md). 简单说: PR 标题遵循 Conventional Commits, 提交 PR 前先在本机跑过

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

## 📄 许可证

MIT, 详见 `LICENSE`.
