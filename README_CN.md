# browser-use

一个轻量级的 Rust 浏览器自动化库，基于 Chrome DevTools Protocol (CDP) 实现。

## ✨ 特性亮点

- **无需 Node.js 依赖** - 纯 Rust 实现，通过 CDP 直接控制浏览器
- **轻量快速** - 无需沉重的运行时，开销极小
- **MCP 集成** - 内置 Model Context Protocol 服务器，支持 AI 驱动的自动化
- **简洁 API** - 易于使用的工具集，涵盖常见浏览器操作

## 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
browser-use = "0.1.0"
```

## 快速开始

```rust
use browser_use::browser::BrowserSession;

// 启动浏览器并导航
let session = BrowserSession::launch(Default::default())?;
session.navigate("https://example.com", None)?;

// 提取 DOM，包含索引化的交互元素
let dom = session.extract_dom()?;
```

## MCP 服务器

运行内置的 MCP 服务器，实现 AI 驱动的自动化：

```bash
# 无头模式
cargo run --bin mcp-server

# 可视化浏览器
cargo run --bin mcp-server -- --headed
```

## 功能

- 导航、点击、输入、截图、提取内容
- DOM 提取，包含索引化的交互元素
- 支持 CSS 选择器或数字索引方式定位元素
- 线程安全的浏览器会话管理

## 环境要求

- Rust 1.70+
- 已安装 Chrome 或 Chromium 浏览器

## 致谢

本项目灵感来源于 [agent-infra/mcp-server-browser](https://github.com/bytedance/UI-TARS-desktop/tree/main/packages/agent-infra/mcp-servers/browser) 并参考了其实现。

## 许可证

MIT
