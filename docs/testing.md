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
- **构造测试夹具时如不确定类型，必须去查类型声明，不要猜测**：项目全局类型定义在 [src/types/global.d.ts](../src/types/global.d.ts)（如 `LyricLine`、`PlayingMusic`、`ListMusic`、`UserInfo`、`Invoke` 等），枚举与常量定义在 [src/constants/](../src/constants/)（如 `LyricFormat`、`LyricTransMode`、`PlayingMode`、`PlayingQuality` 等，经 `src/constants/index.ts` 统一导出）。夹具字段缺失或字段名错误会导致类型断言失效和隐性 bug

### 测试覆盖层级

| 层级            | 文件                                                                                                                                                                                                                                                                                       | 说明           |
| --------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------- |
| **工具函数**    | `tests/utils/music.spec.ts`、`tests/utils/tools.spec.ts`、`tests/utils/icons.spec.ts`                                                                                                                                                                                                      | 纯函数测试     |
| **常量/枚举**   | `tests/constants/app.spec.ts`、`tests/constants/auth.spec.ts`、`tests/constants/list.spec.ts`、`tests/constants/lyric.spec.ts`、`tests/constants/lyricColor.spec.ts`、`tests/constants/playback.spec.ts`、`tests/constants/system.spec.ts`、`tests/constants/window.spec.ts`               | 常量定义测试   |
| **Store**       | `tests/stores/music.spec.ts`、`tests/stores/user.spec.ts`、`tests/stores/list.spec.ts`、`tests/stores/lyric.spec.ts`、`tests/stores/setting.spec.ts`、`tests/stores/context-menu.spec.ts`、`tests/stores/refresh.spec.ts`（覆盖 `useStateStore` 全局状态）、`tests/stores/updater.spec.ts` | 状态管理测试   |
| **组件**        | `tests/components/ProgressRange.spec.ts`、`tests/components/VirtualList.spec.ts`、`tests/components/ContextMenu.spec.ts`、`tests/components/Modal.spec.ts`、`tests/components/SvgIcon.spec.ts`、`tests/components/SelectModal.spec.ts`                                                     | UI 组件测试    |
| **Composables** | `tests/composables/useMiniPlayerBridge.spec.ts`、`tests/composables/useDesktopLyricBridge.spec.ts`、`tests/composables/useObserver.spec.ts`                                                                                                                                                | 组合式函数测试 |
| **测试基建**    | `tests/helpers/infra.spec.ts`（夹具/工厂 `factories.ts`、mock 工具 `mocks.ts`）                                                                                                                                                                                                            | 测试基础设施   |
| **子窗口**      | `tests/windows/desktop-lyric/desktop-lyric.spec.ts`、`tests/windows/desktop-lyric/stores/desktop-lyric.spec.ts`、`tests/windows/mini-player/mini-player.spec.ts`                                                                                                                           | 子窗口业务测试 |
| **集成测试**    | `tests/stores/integration.spec.ts`（N1：选歌↔歌词↔播放↔会员联动）、`tests/stores/integration-n2.spec.ts`（N2：快捷键↔切歌↔歌词↔更新通知链）                                                                                                                                                | 跨模块集成测试 |

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
pnpm lint:fix                 # oxlint --fix
pnpm fmt                      # oxfmt 格式化
pnpm lint:type                # oxlint --type-aware 类型感知检查
```

Oxlint 配置（`.oxlintrc.json`）：

- 启用 `vue` 与 `typescript` 插件，规则取默认集
- `ignorePatterns` 排除 `src-tauri/`（Rust 代码）

Oxfmt 配置（`.oxfmtrc.json`）：

- `singleQuote` 单引号
- 内置 import 排序（`sortImports`），`@/` 别名归为 internal 分组
- 内置 Tailwind 类名排序（`sortTailwindcss`，作用于 `clsx`/`cn` 函数）
- `sortPackageJson` 排序 package.json 脚本
- `ignorePatterns` 排除 `.github/`、`src-tauri/`

## 构建验证

```bash
pnpm tauri:build        # 完整构建（scripts/tauri-build.cjs，读取 env 配置）
```

Release profile 优化：`opt-level = "s"` + `lto = true` + `codegen-units = 1` + `panic = "abort"` + `strip = true`。
