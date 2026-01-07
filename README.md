# Rust Web Server

一个基于 Rust + Tokio + Axum 的高性能 Web 服务，采用模块化架构设计。

## ⚠️ 性能提示

> **你现在可能在用 Dev 模式（很慢）！**  
> 生产环境请用：`cargo run --release`（**快 2-10 倍**）  
> 详细说明：[PERFORMANCE.md](PERFORMANCE.md)

## 架构设计

项目采用三层架构：

- **AppContext** (`src/app_context.rs`): 应用上下文，集中管理配置和依赖
- **Server** (`src/server.rs`): 服务层，负责路由组装和请求处理
- **main** (`src/main.rs`): 入口函数，负责初始化和启动

## 快速开始

### 1. 启动服务

#### 开发模式（Dev）

```bash
cargo run
```

- ✅ 编译快（几秒）
- ❌ 运行慢（未优化）
- 💡 适合：开发调试

#### 生产模式（Release）⚡️ **推荐**

```bash
cargo run --release
```

- ❌ 首次编译慢（1-2 分钟）
- ✅ 运行快（**2-10 倍性能提升**）
- 💡 适合：生产部署、性能测试

或者先构建再运行：

```bash
./build_release.sh                    # 构建优化版本
./target/release/rust-web      # 运行二进制文件
```

服务默认监听在 `http://0.0.0.0:3000`

### 2. 查看日志

服务启动后会自动显示日志：

```bash
# 默认 INFO 级别
cargo run --release

# 更详细的调试日志
RUST_LOG=debug cargo run --release

# 只看警告和错误
RUST_LOG=warn cargo run --release
```

详细说明：[LOGGING.md](LOGGING.md)

### 3. 测试 API

#### 方式一：使用测试脚本（推荐）

```bash
./test_api.sh
```

#### 方式二：手动 curl 测试

**健康检查（GET）：**

```bash
curl http://127.0.0.1:3000/health
```

响应：

```
ok
```

**Rerank 接口（POST）：**

```bash
curl -X POST http://127.0.0.1:3000/rerank \
  -H "Content-Type: application/json" \
  -d '{
    "model": "rerank-model-v1",
    "query": "What is machine learning?",
    "documents": [
      "Machine learning is a subset of artificial intelligence.",
      "The weather today is sunny.",
      "Deep learning uses neural networks.",
      "Pizza is a popular Italian food."
    ]
  }'
```

响应示例：

```json
{
  "results": [
    {
      "index": 0,
      "document": "Machine learning is a subset of artificial intelligence.",
      "relevance_score": 1.0
    },
    {
      "index": 1,
      "document": "The weather today is sunny.",
      "relevance_score": 0.5
    },
    {
      "index": 2,
      "document": "Deep learning uses neural networks.",
      "relevance_score": 0.33333334
    },
    {
      "index": 3,
      "document": "Pizza is a popular Italian food.",
      "relevance_score": 0.25
    }
  ]
}
```

## API 接口说明

### GET /health

健康检查接口

**响应：** `ok` (纯文本)

### POST /sy/on/predict

**处理 Snappy 压缩的 JSON 数据**（等价于 Python 的 `orjson.loads(snappy.decompress(request.body))`）

**请求：**

- Content-Type: `application/octet-stream`
- Body: Snappy 压缩的 JSON 数据

**测试：**

```bash
python3 test_snappy.py
```

**响应示例：**

```json
{
  "status": "success",
  "worker_id": 12345,
  "processing_time_ms": 2,
  "received_data": {
    "model": "test-model-v1",
    "inputs": [...]
  }
}
```

### POST /rerank

文档重排序接口

**请求体：**

```json
{
  "model": "string", // 模型名称
  "query": "string", // 查询文本
  "documents": ["string"] // 待排序的文档列表
}
```

**响应：**

```json
{
  "results": [
    {
      "index": 0, // 原始文档索引
      "document": "string", // 文档内容
      "relevance_score": 1.0 // 相关性分数
    }
  ]
}
```

## 扩展路由

在 `src/server.rs` 的 `build_app()` 方法中添加新路由：

```rust
let protected_routes = Router::new()
    .route("/rerank", post(rerank))
    .route("/your-new-route", post(your_handler)); // 添加新路由
```

## 配置镜像源

项目已配置国内镜像源（rsproxy），如需切换：

编辑 `.cargo/config.toml`：

```toml
[source.crates-io]
replace-with = "rsproxy"  # 可改为 "tuna" 或注释掉使用官方源
```

## 依赖

- `tokio`: 异步运行时
- `axum`: Web 框架
- `serde` / `serde_json`: JSON 序列化/反序列化（**Rust 原生，性能媲美 orjson**）
- `snap`: Snappy 压缩/解压（Rust 原生，对标 Python 的 `python-snappy`）
- `tracing` / `tracing-subscriber`: 结构化日志系统（已配置好）

### Python vs Rust 对比

| Python 库 | Rust 库      | 说明                                |
| --------- | ------------ | ----------------------------------- |
| `orjson`  | `serde_json` | Rust 原生 JSON 性能极高，无需第三方 |
| `snappy`  | `snap`       | Pure Rust 实现，零依赖 C 库         |
| `logging` | `tracing`    | 异步友好的结构化日志                |

## 性能对比：Dev vs Release

| 模式        | 编译时间        | 运行速度           | 二进制大小      | 适用场景 |
| ----------- | --------------- | ------------------ | --------------- | -------- |
| **Dev**     | ⚡️ 快（秒级）  | 🐌 慢              | 📦 大（含调试） | 开发调试 |
| **Release** | 🐌 慢（分钟级） | ⚡️ **快 2-10 倍** | 📦 小（已优化） | 生产部署 |

### 性能测试

```bash
# 启动服务（Release 模式）
cargo run --release

# 在另一个终端运行基准测试
./benchmark.sh
```

## 生产部署

### 方式一：直接运行二进制（推荐）

```bash
# 1. 构建 release 版本
cargo build --release

# 2. 二进制文件位于
./target/release/rust-web

# 3. 可以复制到任何地方运行（静态链接）
cp ./target/release/rust-web /usr/local/bin/
rust-web
```

### 方式二：使用 cargo install

```bash
cargo install --path .
~/.cargo/bin/rust-web
```

### 方式三：Docker 部署

```dockerfile
# Dockerfile 示例
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/rust-web /usr/local/bin/
EXPOSE 3000
CMD ["rust-web"]
```

### 性能调优建议

1. **始终使用 Release 模式**：`--release`
2. **启用 LTO**：已在 `Cargo.toml` 配置
3. **多核并行**：Tokio 自动利用所有 CPU 核心
4. **内存预分配**：Rust 零开销抽象，无 GC 停顿

## 开发

```bash
# 检查代码
cargo check

# 运行测试
cargo test

# 格式化代码
cargo fmt

# 代码检查
cargo clippy

# 构建 release 版本
./build_release.sh
```
