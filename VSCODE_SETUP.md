# VSCode Rust 开发环境配置指南

## 🚨 问题：Cargo.toml 没有语法高亮？

**原因**：缺少 Rust 和 TOML 扩展

## ✅ 快速解决（3 步）

### 1️⃣ 安装必需扩展

VSCode 会自动提示你安装推荐的扩展（右下角弹窗）：

点击 **"Install All"** 或者手动安装：

#### 必装扩展（核心）

1. **rust-analyzer**

   - 📦 扩展 ID: `rust-lang.rust-analyzer`
   - 🎯 作用：Rust 语言支持、代码补全、错误检查
   - 💡 相当于 Python 的 Pylance

2. **Even Better TOML**
   - 📦 扩展 ID: `tamasfe.even-better-toml`
   - 🎯 作用：TOML 文件语法高亮（Cargo.toml）
   - 💡 解决你当前的问题

#### 推荐扩展（增强）

3. **crates**

   - 📦 扩展 ID: `serayuzgur.crates`
   - 🎯 作用：显示依赖最新版本、内联文档

4. **CodeLLDB**
   - 📦 扩展 ID: `vadimcn.vscode-lldb`
   - 🎯 作用：Rust 调试器

### 2️⃣ 手动安装方法

如果没有自动提示，手动安装：

1. 打开 VSCode
2. 按 `Cmd+Shift+X` (macOS) 或 `Ctrl+Shift+X` (Windows/Linux)
3. 搜索 `rust-analyzer`
4. 点击 **Install**
5. 重复安装 `Even Better TOML`

### 3️⃣ 重新加载 VSCode

安装后：

- 按 `Cmd+Shift+P` / `Ctrl+Shift+P`
- 输入 `Reload Window`
- 回车

## 🎨 安装后效果

### Cargo.toml 现在会显示：

```toml
[package]                          # 紫色
name = "rust-web"          # 字符串是绿色
version = "0.1.0"                 # 数字是橙色

[dependencies]                     # 紫色
tokio = { version = "1.42.0" }    # 语法高亮，可折叠
```

### Rust 代码会有：

- ✅ 语法高亮
- ✅ 代码补全
- ✅ 错误提示（红色波浪线）
- ✅ 函数跳转（按住 Cmd/Ctrl 点击）
- ✅ 类型提示（鼠标悬停）

## 🔧 已配置的功能

我已经为你创建了 `.vscode/` 配置：

### 自动功能

1. **保存时自动格式化** - 代码自动对齐
2. **保存时运行 Clippy** - 自动检查代码规范
3. **智能代码补全** - Tab 键补全
4. **类型提示** - 鼠标悬停显示类型
5. **错误实时显示** - 边写边检查

### 快捷键

| 功能       | macOS            | Windows/Linux |
| ---------- | ---------------- | ------------- |
| 格式化代码 | `Shift+Option+F` | `Shift+Alt+F` |
| 跳转到定义 | `Cmd+点击`       | `Ctrl+点击`   |
| 查找引用   | `Shift+F12`      | `Shift+F12`   |
| 重命名符号 | `F2`             | `F2`          |
| 快速修复   | `Cmd+.`          | `Ctrl+.`      |

## 📝 使用示例

### 1. Cargo.toml 依赖提示

当你在 `Cargo.toml` 中输入：

```toml
[dependencies]
tokio = "1.42.0"
       ↑ 鼠标悬停会显示最新版本
```

### 2. Rust 代码补全

```rust
use axum::   ← 输入后会自动提示可用模块
    routing  ← 选择后自动补全
    Router   ← 继续补全
```

### 3. 错误提示

```rust
let x = "hello";
x = 5;  // ❌ 红色波浪线：cannot assign twice to immutable variable
```

鼠标悬停会显示详细错误和建议。

### 4. 快速修复

光标放在错误上，按 `Cmd+.` / `Ctrl+.`：

```rust
let x = "hello";  // 💡 建议：Add 'mut' to make it mutable
```

## 🚀 验证安装

### 测试 1：打开 Cargo.toml

1. 打开 `Cargo.toml`
2. 应该看到彩色语法高亮
3. 鼠标悬停在依赖版本上，显示最新版本

### 测试 2：打开 main.rs

1. 打开 `src/main.rs`
2. 输入 `tokio::` - 应该有自动补全
3. 鼠标悬停在 `Runtime` 上 - 显示类型信息

### 测试 3：运行代码

按 `F5` 或点击右上角的 ▶️ 按钮：

- 应该能看到运行选项
- 选择 `cargo run` 或 `cargo run --release`

## 🔍 故障排查

### 问题 1：rust-analyzer 没有工作

**解决**：

```bash
# 确保 Rust 已安装
rustc --version

# 重启 rust-analyzer
Cmd+Shift+P → "rust-analyzer: Restart Server"
```

### 问题 2：依然没有语法高亮

**解决**：

1. 检查文件扩展名是否正确（`.toml` / `.rs`）
2. 右下角点击语言模式，选择 `TOML` 或 `Rust`
3. 重启 VSCode

### 问题 3：代码补全不工作

**解决**：

```bash
# 确保项目能编译
cargo check

# 清理后重新构建
cargo clean
cargo check
```

### 问题 4：rust-analyzer 太慢

**解决**：

```bash
# 在项目根目录创建 rust-toolchain.toml
echo 'channel = "stable"' > rust-toolchain.toml

# 或者在设置中排除 target 目录（已配置）
```

## ⚙️ 高级配置（可选）

### 自定义 rust-analyzer

编辑 `.vscode/settings.json`：

```json
{
  // 使用 clippy 代替默认检查
  "rust-analyzer.checkOnSave.command": "clippy",

  // 启用所有 features
  "rust-analyzer.cargo.features": "all",

  // 内联类型提示
  "rust-analyzer.inlayHints.typeHints.enable": true,

  // 保存时自动导入缺失的包
  "rust-analyzer.imports.granularity.group": "module"
}
```

### 调试配置

创建 `.vscode/launch.json`：

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug",
      "cargo": {
        "args": ["build", "--bin=rust-web"]
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## 📚 更多资源

- [rust-analyzer 文档](https://rust-analyzer.github.io/)
- [VSCode Rust 开发指南](https://code.visualstudio.com/docs/languages/rust)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)

## ✅ 检查清单

- [ ] 安装了 `rust-analyzer` 扩展
- [ ] 安装了 `Even Better TOML` 扩展
- [ ] `Cargo.toml` 有语法高亮
- [ ] Rust 代码有自动补全
- [ ] 保存时自动格式化
- [ ] 错误实时显示

**全部勾选？恭喜，环境配置完成！** 🎉

---

**遇到问题？** 查看 [LOGGING.md](LOGGING.md) 和 [README.md](README.md)
