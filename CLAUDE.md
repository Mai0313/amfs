# amfs

**amfs** (Agent Memory File System) is a local-first memory store for AI agents, shipped as a single Rust binary. Agents `add` memories and get them back with `search`, which matches by meaning through an embedding model rather than by keyword. The point of the project is to do this on the user's own machine instead of paying a hosted service per memory.

The project is in early development: the CLI surface is settled, but every subcommand still returns `Error::NotImplemented`. Storage format and embedding backends are undecided, so do not invent one without being asked to.

## Layout

```
src/lib.rs          Crate root: build metadata helpers, re-exports `cli`
src/cli.rs          clap definitions (Cli, Command, Error) and dispatch
src/main.rs         Thin binary entry point, maps errors to an exit code
build.rs            Embeds git tag / commit / toolchain versions at build time
tests/              Integration tests, one crate per file
cli/nodejs/         npm wrapper that shells out to the platform binary
cli/python/         PyPI wrapper that shells out to the platform binary
```

The two wrappers under `cli/` contain no logic. They locate a prebuilt binary under `binaries/<platform>/` and forward `argv` to it. CI drops the release binaries into those directories before publishing.

## Distribution Names Differ Per Registry

This is the single easiest thing to get wrong:

| Target           | Name              |
| ---------------- | ----------------- |
| binary / command | `amfs`            |
| crates.io        | `amfs`            |
| npm              | `@mai0313/amfs`   |
| PyPI             | `agent-memory-fs` |

Only crates.io could take the short name. `amfs` was already registered on PyPI by an unrelated project, and npm refuses the unscoped `amfs` outright because its typosquatting check finds it too similar to `memfs`, `fs`, `ms`, and friends — that one is a registry policy, not a name that might free up later.

The command both wrappers install is still `amfs`, which is why `cli/python/pyproject.toml` needs `[tool.uv.build-backend] module-name = "amfs"`. Without it the backend derives the module name from the distribution name and the build fails.

`build_release.yml` declares `BIN_NAME` and `PYPI_NAME` at the workflow level. Use those instead of `github.event.repository.name`; the repository name is deliberately not a package name. The `env` context is **not** available inside `strategy.matrix`, so the npm package matrix spells the names out as literals.

npm publishing runs over OIDC trusted publishing, which cannot create a package that does not exist yet. Any new npm name has to be bootstrapped once with an access token before a tagged release can publish it.

## Versioning

`Cargo.toml`, `package.json`, and `pyproject.toml` all carry a placeholder version. The real version comes from the git tag: `build_release.yml` derives it from `v*` and rewrites each manifest before publishing. Do not bump versions by hand in a PR.

At runtime `amfs --version` reports richer information (commits since tag, short hash, `dirty` marker, toolchain versions) because `build.rs` injects it through `BUILD_VERSION`.

## Developer Workflow

```bash
make fmt       # rustfmt + clippy --fix, then clippy -D warnings
make test      # cargo test --all
make release   # cargo build --release --locked
make help      # list targets
```

Run `make fmt` and `make test` before every commit. CI runs `uvx pre-commit run -a`, `cargo fmt --all -- --check`, and `cargo clippy --all-targets --all-features -- -D warnings`; all three must be clean.

Pre-commit only sees tracked files, so `git add` a new file before running the hooks or it will pass locally and fail in CI.

## Code Conventions

- **Errors**: return `Result` with a concrete error type. Do not swallow an error or paper over it with a fallback that is less reliable than the path it replaces. Avoid `unwrap()` / `expect()` outside tests.
- **Unsafe**: every `unsafe` block needs a `// SAFETY:` comment.
- **Docs**: public items carry `///` doc comments. Keep comments short and about intent, not mechanics.
- **Tests**: unit tests sit next to the code in `#[cfg(test)] mod tests`; integration tests go under `tests/`, one file per topic. Reach the binary through `env!("CARGO_BIN_EXE_amfs")` rather than adding a test-harness dependency.
- **MSRV**: `rust-version` in `Cargo.toml`. Do not raise it without discussion.

## Git Conventions

Commit messages and PR titles are **English only** and follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>[optional scope]: <description>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`, `perf`, `revert`. PR titles are validated by `semantic-pull-request.yml` and become the squash-merge commit message.

## GitHub Actions Conventions

When editing `.github/workflows/*.yml`:

- **Job keys** in order: `name`, `needs`, `runs-on`, `if`
- **Step keys** in order: `name`, `id`, `continue-on-error`, `if`, `uses`, `with`, `env`, `shell` / `run` (keep `shell` directly above `run`)
- Do not define an environment variable that is used exactly once; inline the expression instead.

## Documentation

`README.md`, `README.zh-TW.md`, and `README.zh-CN.md` describe the same thing in three languages and must be updated together. Preserve the badge block; only the URLs inside it change.

Anything published to GitHub, including issues, PRs, and review comments, is written in English.
