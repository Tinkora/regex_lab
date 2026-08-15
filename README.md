# regex_lab

[![CI](https://github.com/Tinkora/regex_lab/actions/workflows/test.yml/badge.svg)](https://github.com/Tinkora/regex_lab/actions/workflows/test.yml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](./LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](./CONTRIBUTING.md)

浏览器原生的正则表达式测试实验室 —— 实时匹配、捕获组提取、替换预览、多语言代码生成，全部在 WASM 中运行。

## ✨ 特性

- 🔍 **实时匹配** —— 输入即反馈，正则模式验证和匹配结果实时更新
- 🎯 **捕获组提取** —— 高亮显示匹配文本，表格展示所有捕获组和命名组
- ✏️ **替换预览** —— 先预览替换效果，确认后再复制结果
- 🌐 **多语言代码生成** —— 一键生成 JS/Python/Rust/Go 等效正则代码片段
- 🏳️ **完整标志支持** —— i (忽略大小写)、m (多行)、s (点匹配换行)、U (非贪婪)、x (忽略空白)、u (Unicode)
- 🔒 **隐私优先** —— 所有处理在浏览器内完成，测试文本绝不离开本地

## 🚀 快速开始

```bash
# 克隆
git clone https://github.com/Tinkora/regex_lab.git
cd regex_lab

# 构建 Web WASM
wasm-pack build --target web crates/regex_lab_web

# 启动
cp crates/regex_lab_web/pkg/* crates/regex_lab_web/static/pkg/
cd crates/regex_lab_web/static && python3 -m http.server 8080
```

浏览器打开 `http://localhost:8080`

## 📂 项目结构

| 组件 | 说明 | 状态 |
|------|------|------|
| `regex_lab_core` | 正则引擎、匹配/替换/分割逻辑、错误类型 | ✅ |
| `regex_lab_web` | WASM 桥接 + HTML 交互界面 | ✅ |
| `skills/` | Agent Skill 定义 (MCP tools) | ✅ |

## 🔧 开发

```bash
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p regex_lab_web --target wasm32-unknown-unknown
```

## 📄 文档

- [产品规格](docs/product_spec.zh-CN.md)

## 🤝 社区

- [贡献指南](./CONTRIBUTING.md)
- [行为准则](./CODE_OF_CONDUCT.md)
- [安全策略](./SECURITY.md)
- [更新日志](./CHANGELOG.md)

## Support

If regex_lab saves you time, support Tinkora on [Ko-fi](https://ko-fi.com/tinkora).
Support is optional and never affects access or issue priority.

See [SUPPORT.md](./SUPPORT.md) for questions, bug reports, and security reports.

## 📜 License

MIT © [Tinkora](https://github.com/Tinkora)
