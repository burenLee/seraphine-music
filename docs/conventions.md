# 关键约定

## 共享层与业务层

- 共享基础层代码放在 `src/components/`、`src/utils/`、`src/types/`、`src/styles/`，三窗口通用
- 各窗口业务代码放在 `src/windows/[window-name]/`

## Commit 规范

- 遵循 Conventional Commits 风格
- 使用 `Fixes`/`Closes`/`Resolves` + `#<issue-number>` 自动关闭 issue

## ADR

- 架构决策使用 ADR 文档记录，编号从 `0001` 开始，存放在 `docs/adr/`

## 版本同步

- `package.json`、`src-tauri/Cargo.toml`、`src-tauri/tauri.conf.json` 三处版本号保持一致
- 使用 `pnpm set-ver *` 同步版本
