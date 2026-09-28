# 发布流程

SeraphineMusic 从代码到用户收到更新的完整链路。

```mermaid
flowchart TD
    subgraph manual["手动"]
        A["更新版本 + CHANGELOG<br/>pnpm set-ver"]
        B["打 tag 触发发布<br/>git tag v0.2.0"]
    end

    subgraph ci["CI 自动"]
        C["三平台构建 + 签名<br/>提取 changelog → releaseBody"]
        D["创建 Release + latest.json<br/>上传产物 + 生成更新清单"]
    end

    subgraph user["用户端"]
        E["检查更新 → 安装 → 重启<br/>弹窗展示 changelog"]
    end

    A --> B
    B --> C
    C --> D
    D --> E

    style B fill:#F2F7FF,stroke:#4B3FE3,color:#1A1759
```

## 阶段说明

- **手动**：`pnpm set-ver` 统一版本号，更新 CHANGELOG.md，打 tag（`v` 前缀）触发 CI。
- **CI 自动**：三平台矩阵构建并签名，从 CHANGELOG 提取对应版本段作为 Release body，生成 `latest.json`。
- **用户端**：应用检查 `latest.json` 对比版本，有新版则弹窗展示 changelog，下载校验签名后安装并重启。
