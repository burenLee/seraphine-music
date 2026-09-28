# Changelog

本项目所有显著变更都会记录在此文件中。

格式基于 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [0.2.0] - 2026-09-29

### 新增

- 在线更新功能：支持检查更新、下载与安装，应用内自动升级
- 主题跟随系统：外观主题支持跟随系统设置
- 用户配置缓存清除：设置中可一键清理本地缓存

### 变更

- 后端架构重构，拆分为 app / http / music / plugins / utils 分层结构
- 重构更新检测与下载逻辑，优化交互体验
- 引入统一日志系统，规范错误与状态记录

### 技术

- 工具链由 ESLint / Prettier 迁移至 oxlint / oxfmt
- 完善单元测试与集成测试基础设施
