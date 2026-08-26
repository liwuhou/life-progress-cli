# Product

<!-- impeccable:product-schema 1 -->

## Platform

web

## Users

个人用户在终端中输入或复用个人 profile，查看自己的生命进度。

## Product Purpose

通过共享的个人 profile 持久化生日、性别和国家信息，并将寿命统计转化为已用天数、剩余天数和进度百分比。

## Positioning

以终端优先的 Rust/Cargo 工作流，将可复用的个人档案与即时生命进度计算结合起来。

## Operating Context

用户从仓库根目录运行 Cargo CLI。无生日参数或子命令时，应用读取 `~/.config/life_progress/profile.toml`；显式 CLI 参数可用于单次覆盖。国家数据由本地缓存、Wikipedia 抓取或 bundled fallback 提供。

## Capabilities and Constraints

- 支持 `search`、`view` 与生命进度计算。
- profile 使用 v1 TOML 契约：`schema_version`、`birthday`、可选 `gender`、规范 `nation`。
- profile 缺失与 profile 无效必须区分；无效数据必须以可恢复错误拒绝。
- 保持 Rust 2021、Cargo、同步执行和可脚本化 CLI 输出。

## Evidence on Hand

- CLI 用法与示例：`README.md`。
- 架构、开发命令与数据流：`AGENTS.md`、`CLAUDE.md`。
- 启动 profile 行为测试：`tests/profile_startup.rs`。
- 生命预期 fallback 数据：`crates/lifespan-crawler/default_expectancy.json`。

## Product Principles

- 个人数据应可复用且有明确版本。
- 终端工作流应直接、快速、可验证。
- 校验失败时宁可明确报错，也不使用隐式 fallback 个人数据。

## Accessibility & Inclusion

当前无额外产品特定可访问性要求；保持清晰的纯文本输出与可脚本化命令行为。
