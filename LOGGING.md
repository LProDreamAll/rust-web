# 日志配置说明

## ✅ 已配置好，可以直接使用

### 默认配置

- **日志级别**: INFO（显示 info、warn、error）
- **显示内容**: 时间、级别、线程 ID、行号、消息

## 🎯 使用方式

### 1. 基本用法

在代码中使用 `tracing` 宏：

```rust
use tracing::{info, warn, error, debug, trace};

// 不同级别的日志
info!("这是一条信息日志");
warn!("这是一条警告");
error!("这是一条错误");
debug!("这是调试信息（默认不显示）");
trace!("这是追踪信息（默认不显示）");

// 带变量的日志
let user_id = 123;
let status = "success";
info!("用户 {} 操作状态: {}", user_id, status);

// 结构化日志（推荐）
info!(user_id = %user_id, status = %status, "操作完成");
```

### 2. 控制日志级别

#### 方式一：环境变量（推荐）

```bash
# 显示所有级别（包括 debug、trace）
RUST_LOG=trace cargo run

# 只显示 info 及以上
RUST_LOG=info cargo run

# 只显示 warn 和 error
RUST_LOG=warn cargo run

# 针对特定模块
RUST_LOG=rust_web::server=debug cargo run
```

#### 方式二：修改代码

在 `src/main.rs` 中修改：

```rust
.with_env_filter(
    tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "debug".into())  // 改成 debug
)
```

### 3. 日志级别说明

| 级别      | 用途               | 示例                         |
| --------- | ------------------ | ---------------------------- |
| **ERROR** | 错误，需要立即关注 | 数据库连接失败、文件读取失败 |
| **WARN**  | 警告，可能有问题   | 配置缺失、性能下降           |
| **INFO**  | 重要信息（默认）   | 请求日志、启动信息           |
| **DEBUG** | 调试信息           | 变量值、函数调用             |
| **TRACE** | 详细追踪           | 循环内部、底层细节           |

## 📝 实际输出示例

启动服务后，你会看到：

```
2025-01-07T10:30:15.123Z  INFO ThreadId(01) [12] 请求成功 | 路径: /sy/on/predict | 方法: POST, work_id: 12345
2025-01-07T10:30:15.145Z  INFO ThreadId(02) [98] Processing completed in 2ms
```

格式说明：

- `2025-01-07T10:30:15.123Z` - 时间戳
- `INFO` - 日志级别
- `ThreadId(01)` - 线程 ID
- `[12]` - 代码行号
- 后面是日志消息

## 🔧 高级配置

### 1. JSON 格式输出（适合日志收集）

在 `src/main.rs` 中：

```rust
tracing_subscriber::fmt()
    .json()  // 改成 JSON 格式
    .init();
```

### 2. 输出到文件

```rust
use std::fs::File;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

let file = File::create("app.log")?;
tracing_subscriber::fmt()
    .with_writer(BoxMakeWriter::new(file))
    .init();
```

### 3. 同时输出到控制台和文件

```rust
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

tracing_subscriber::registry()
    .with(tracing_subscriber::fmt::layer())
    .with(tracing_subscriber::fmt::layer()
        .with_writer(std::fs::File::create("app.log")?))
    .init();
```

## 🧪 测试日志

### 1. 启动服务

```bash
cargo run --release
```

### 2. 发送请求

```bash
# 测试 /sy/on/predict 接口
python3 test_snappy.py

# 或者普通接口
curl http://127.0.0.1:3000/health
```

### 3. 查看日志

你应该能看到类似这样的输出：

```
 INFO ThreadId(03) [127] 请求成功 | 路径: /sy/on/predict | 方法: POST, work_id: 23456
 INFO ThreadId(03) [162] Processing completed in 3.2ms
```

## 🎨 不同级别的使用场景

### server.rs 中的示例

```rust
use tracing::{info, warn, error, debug};

async fn predict_on_sy(
    State(_state): State<Arc<AppState>>,
    body: Bytes,
) -> Response {
    let start = Instant::now();
    let worker_id = std::process::id();

    info!("请求成功 | 路径: /sy/on/predict | 方法: POST, work_id: {}", worker_id);

    // 解压失败
    let decompressed = match snap::raw::Decoder::new().decompress_vec(&body) {
        Ok(data) => {
            debug!("解压成功，大小: {} bytes", data.len());
            data
        },
        Err(e) => {
            error!("解压失败: {}", e);  // 使用 error!
            return error_response("Failed to decompress");
        }
    };

    let elapsed = start.elapsed();

    if elapsed.as_millis() > 100 {
        warn!("处理时间过长: {:?}", elapsed);  // 性能警告
    } else {
        info!("Processing completed in {:?}", elapsed);
    }

    // ...
}
```

## 📊 性能影响

- **INFO 级别**: 几乎无性能影响（< 1%）
- **DEBUG 级别**: 轻微影响（~ 2-5%）
- **TRACE 级别**: 明显影响（~ 10-20%）

**建议**：

- 开发环境: `RUST_LOG=debug`
- 生产环境: `RUST_LOG=info`
- 性能测试: `RUST_LOG=warn` 或关闭日志

## 🚀 快速命令

```bash
# 正常启动（INFO 级别）
cargo run --release

# 调试模式（DEBUG 级别）
RUST_LOG=debug cargo run --release

# 详细追踪（TRACE 级别）
RUST_LOG=trace cargo run --release

# 只看错误和警告
RUST_LOG=warn cargo run --release

# 只看特定模块
RUST_LOG=rust_web::server=trace cargo run --release
```

---

**现在你的日志已经配置好了！** 🎉
