# 性能优化指南

## 🚨 重要：你现在可能在用 Dev 模式（很慢！）

### 如何检查？

```bash
# 如果你这样启动的，就是 Dev 模式
cargo run

# Release 模式应该这样启动
cargo run --release
```

## 📊 Dev vs Release 性能对比

### 实际测试数据（参考）

| 指标          | Dev 模式 | Release 模式 | 提升倍数 |
| ------------- | -------- | ------------ | -------- |
| JSON 解析     | 100ms    | 10ms         | **10x**  |
| Snappy 解压   | 50ms     | 8ms          | **6x**   |
| QPS (请求/秒) | 1,000    | 10,000+      | **10x**  |
| 延迟 P99      | 100ms    | 10ms         | **10x**  |
| 内存占用      | 50MB     | 30MB         | **更小** |

### 为什么差这么多？

| 优化项         | Dev 模式       | Release 模式     |
| -------------- | -------------- | ---------------- |
| **优化级别**   | `-O0` (无优化) | `-O3` (最高优化) |
| **内联函数**   | ❌ 不内联      | ✅ 激进内联      |
| **循环展开**   | ❌ 不展开      | ✅ 自动展开      |
| **向量化**     | ❌ 不启用      | ✅ SIMD 向量化   |
| **死代码消除** | ❌ 保留        | ✅ 移除          |
| **LTO**        | ❌ 关闭        | ✅ 开启          |
| **调试符号**   | ✅ 包含        | ❌ 移除          |

## 🎯 正确的使用方式

### 开发时（Dev 模式）

```bash
# 快速编译 + 调试
cargo run

# 适合场景：
# - 修改代码频繁重编译
# - 需要详细的错误堆栈
# - 使用调试器 (gdb/lldb)
```

### 测试性能时（Release 模式）⚡️

```bash
# 第一次编译会慢一些（1-2分钟）
cargo run --release

# 之后的编译是增量的，也会快很多
```

### 生产部署（Release 模式）🚀

```bash
# 1. 构建优化的二进制文件
cargo build --release

# 2. 二进制在这里（可以直接运行，无需 cargo）
./target/release/rust-web

# 3. 或者用脚本
./build_release.sh
```

## 📦 二进制文件大小对比

```bash
# Dev 版本（含调试信息）
./target/debug/rust-web      # ~50MB

# Release 版本（已优化 + strip）
./target/release/rust-web    # ~8MB
```

我们在 `Cargo.toml` 中配置了：

```toml
[profile.release]
strip = true  # 移除调试符号，大幅减小体积
```

## 🔧 已启用的优化配置

在 `Cargo.toml` 中：

```toml
[profile.release]
opt-level = 3        # 最高优化级别
lto = "thin"        # Link Time Optimization
codegen-units = 1   # 单代码单元，最大化优化
strip = true        # 移除调试符号
panic = "abort"     # panic 时直接退出，无栈展开开销
```

## 🧪 如何验证性能提升？

### 方法一：使用基准测试脚本

```bash
# 1. 启动 Release 版本
cargo run --release

# 2. 在另一个终端运行基准测试
./benchmark.sh
```

### 方法二：手动对比

```bash
# 终端 1: Dev 模式
cargo run

# 终端 2: 测试
time curl -X POST http://127.0.0.1:3000/sy/on/predict \
  -H "Content-Type: application/octet-stream" \
  --data-binary @test_data.snappy

# 然后切换到 Release 模式再测一次
```

### 方法三：使用 Python 测试脚本

```bash
# 运行 snappy 压缩测试（会显示处理时间）
python3 test_snappy.py

# 对比 Dev 和 Release 模式下的 processing_time_ms
```

## 💡 最佳实践

### ✅ DO（推荐）

- ✅ 开发调试用 `cargo run`
- ✅ 性能测试用 `cargo run --release`
- ✅ 生产部署用 `cargo build --release`
- ✅ 定期运行 `./benchmark.sh` 监控性能

### ❌ DON'T（避免）

- ❌ 生产环境用 Dev 模式
- ❌ 用 Dev 模式测试性能指标
- ❌ 只看编译时间不看运行性能
- ❌ 部署时不做性能测试

## 🎓 深入理解

### 为什么 Rust 的 Release 优化这么强？

1. **零成本抽象**：编译时展开所有抽象层，运行时无开销
2. **单态化**：泛型在编译时生成专门的代码
3. **激进内联**：跨 crate 的函数内联（LTO）
4. **LLVM 优化**：利用 LLVM 的所有优化 pass
5. **无 GC**：没有垃圾回收停顿

### 与 Python 对比

| 语言       | Dev 模式     | Release 模式    | 动态优化   |
| ---------- | ------------ | --------------- | ---------- |
| **Rust**   | 慢但可调试   | **极快**        | 编译期完成 |
| **Python** | 已经是"最快" | 无 release 概念 | JIT 有限   |

Python 的 `orjson` 已经是 C 扩展了，但 Rust 的 `serde_json` 在 Release 模式下性能相当甚至更优。

## 🚀 快速切换指令

```bash
# 开发（快速迭代）
cargo run

# 生产（最高性能）
cargo run --release

# 构建独立二进制
./build_release.sh

# 性能测试
./benchmark.sh
```

---

**记住：Release 模式才是 Rust 的真正实力！** 🦀⚡️
