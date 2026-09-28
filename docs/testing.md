# 验证流程与质量检查

## 前端测试（Vitest）

测试框架：Vitest 4.x + happy-dom + @vue/test-utils

### 运行测试

```bash
pnpm test                     # 单次运行所有测试
```

### 测试文件约定

- 测试文件独立放在 `tests/` 目录，目录结构**镜像** `src/`（如 `tests/utils/music.spec.ts` 对应 `src/utils/music.ts`）
- 测试范围：`tests/**/*.spec.ts`（见 `vitest.config.ts`）
- 测试环境使用 `happy-dom`
- 源码模块引用统一使用 `@/` 别名（如 `from '@/stores/music'`），避免相对导入层级混乱
- **构造测试夹具时如不确定类型，必须去查类型声明，不要猜测**：项目全局类型定义在 [src/types/global.d.ts](../src/types/global.d.ts)（如 `LyricLine`、`PlayingMusic`、`ListMusic`、`UserInfo`、`Invoke` 等），枚举定义在 [src/utils/params.ts](../src/utils/params.ts)（如 `LyricFormat`、`LyricTransMode`、`PlayingMode`、`PlayingQuality` 等）。夹具字段缺失或字段名错误会导致类型断言失效和隐性 bug

### 测试覆盖层级

| 层级            | 文件                                                                                                                                                                                                                                                      | 说明           |
| --------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------- |
| **工具函数**    | `tests/utils/music.spec.ts`、`tests/utils/params.spec.ts`、`tests/utils/tools.spec.ts`、`tests/utils/hooks.spec.ts`、`tests/utils/icons.spec.ts`                                                                                                          | 纯函数测试     |
| **Store**       | `tests/stores/music.spec.ts`、`tests/stores/user.spec.ts`、`tests/stores/list.spec.ts`、`tests/stores/lyric.spec.ts`、`tests/stores/setting.spec.ts`、`tests/stores/context-menu.spec.ts`、`tests/stores/refresh.spec.ts`、`tests/stores/updater.spec.ts` | 状态管理测试   |
| **组件**        | `tests/components/ProgressRange.spec.ts`、`tests/components/VirtualList.spec.ts`、`tests/components/ContextMenu.spec.ts`、`tests/components/Modal.spec.ts`、`tests/components/SvgIcon.spec.ts`、`tests/components/SelectModal.spec.ts`                    | UI 组件测试    |
| **Composables** | `tests/composables/useMiniPlayerBridge.spec.ts`、`tests/composables/useDesktopLyricBridge.spec.ts`                                                                                                                                                        | 子窗口桥接测试 |
| **子窗口**      | `tests/windows/desktop-lyric/desktop-lyric.spec.ts`、`tests/windows/desktop-lyric/stores/desktop-lyric.spec.ts`、`tests/windows/mini-player/mini-player.spec.ts`                                                                                          | 子窗口业务测试 |
| **集成测试**    | `tests/stores/integration.spec.ts`（N1：选歌↔歌词↔播放↔会员联动）、`tests/stores/integration-n2.spec.ts`（N2：快捷键↔切歌↔歌词↔更新通知链）                                                                                                               | 跨模块集成测试 |

### 播放状态机测试要点

- `tests/stores/music.spec.ts` 覆盖：初始默认值、Play/Pause/Stop 标志、setMusic 加载流程、5 种播放模式、playPrevOrNext 手动切歌、进度拖拽、MAX_RETRY_COUNT 失败重试
- 使用 `fake timers` 避免定时器死锁
- Pinia ref 比较使用 `toStrictEqual` 而非 `toBe`

## Rust 后端测试

### 运行测试

```bash
cd src-tauri && cargo test
```

### 测试策略

- 纯函数直接调用（如 `get_decode_content`、`SearchType` Display 变体）
- 枚举 serde 验证（如 `Quality` 序列化/反序列化）
- 数据结构边界检查（如 `LoginCellphoneData` 必填字段、`fakem` 长度约束）
- 常量约束通过源码文本反射验证（如 `LITE_T1_KEY`/`IV` 常量）
- 编译时验证 command 符号存在性

## 类型检查

```bash
pnpm build              # vue-tsc --noEmit && vite build
```

`vue-tsc --noEmit` 执行完整类型检查，`tsconfig.json` 启用 `strict: true`。

## 代码风格

### Oxlint / Oxfmt

```bash
pnpm lint-format               # oxlint --fix && oxfmt
```

Oxlint 配置（`.oxlintrc.json`）：

- Vue essential + TypeScript recommended 规则经 `@oxlint/migrate` 迁移而来
- `no-unused-vars` 设为 `warn`，默认 correctness 类别开启
- 原配置关闭的 `vue/multi-word-component-names` 等规则在 Oxlint 中默认未启用
- 排除 `src-tauri/**`（Rust 代码）

Oxfmt 配置（`.oxfmtrc.json`）：

- 内置 import 排序（`sortImports`），`@/` 别名归为 internal 分组
- 内置 Tailwind 类名排序（`sortTailwindcss`）
- 与 Prettier 输出兼容（printWidth 100 / 无分号 / 单引号 / 无尾逗号）

## 构建验证

```bash
pnpm tauri build        # 完整构建（前端 + Rust + 打包）
```

Release profile 优化：`opt-level = "s"` + `lto = true` + `codegen-units = 1` + `strip = true`。
