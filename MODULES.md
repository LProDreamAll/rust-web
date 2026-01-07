# 模块说明文档

## 📁 项目结构

```
src/
├── main.rs                  # 程序入口
├── lib.rs                   # 库入口，导出所有模块
├── server.rs                # HTTP 服务器和路由
├── app_context.rs           # 应用上下文（依赖注入容器）
├── config/                  # 配置模块
│   ├── mod.rs              # 模块导出
│   └── types.rs            # 配置类型定义
└── observability/          # 可观测性模块
    ├── mod.rs              # 模块导出
    └── logging.rs          # 日志配置
```

## 📦 核心模块

### 1. Config 模块 (`src/config/`)

管理应用配置。

#### `ServerConfig`

服务器配置结构体。

**字段：**

```rust
pub struct ServerConfig {
    pub host: String,               // 监听地址
    pub port: u16,                  // 监听端口
    pub log_dir: Option<String>,    // 日志目录
    pub log_level: Option<String>,  // 日志级别
    pub max_payload_size: usize,    // 最大负载大小（默认 512MB）
    pub request_timeout_secs: u64,  // 请求超时（默认 1800s）
}
```

**使用示例：**

```rust
use rust_web::config::ServerConfig;

// 方式 1: 使用默认值
let config = ServerConfig::default();

// 方式 2: 构建器模式
let config = ServerConfig::new()
    .host("127.0.0.1")
    .port(8080)
    .log_level("debug")
    .max_payload_size(1024 * 1024 * 1024); // 1GB

// 方式 3: 从 JSON 文件加载
let config = ServerConfig::from_json_file("config.json")?;

// 保存到 JSON 文件
config.to_json_file("config.json")?;

// 验证配置
config.validate()?;

// 获取地址
println!("Server will listen on: {}", config.addr());
```

**JSON 配置示例：**

```json
{
  "host": "0.0.0.0",
  "port": 3000,
  "log_level": "info",
  "max_payload_size": 536870912,
  "request_timeout_secs": 1800
}
```

### 2. Observability 模块 (`src/observability/`)

可观测性功能（日志、追踪、指标）。

#### `LoggingConfig`

日志配置结构体。

**字段：**

```rust
pub struct LoggingConfig {
    pub level: Level,           // 日志级别
    pub json_format: bool,      // JSON 格式输出
    pub log_dir: Option<String>, // 日志目录
    pub colorize: bool,         // 彩色输出
}
```

**使用示例：**

```rust
use rust_web::observability::logging::{self, LoggingConfig};
use tracing::Level;

// 方式 1: 使用默认配置
logging::init_default();

// 方式 2: 自定义配置
let config = LoggingConfig::new()
    .level(Level::DEBUG)
    .colorize(true);
logging::init_logging(config);

// 方式 3: 从环境变量读取级别
// RUST_LOG=debug cargo run

// 在代码中使用日志
use tracing::{info, warn, error, debug};

info!("服务启动");
debug!("调试信息: value = {}", 42);
warn!("警告: 连接数过多");
error!("错误: 无法连接数据库");
```

**日志级别：**

| 级别    | 用途     | 示例               |
| ------- | -------- | ------------------ |
| `TRACE` | 详细追踪 | 循环内部、底层细节 |
| `DEBUG` | 调试信息 | 变量值、函数调用   |
| `INFO`  | 重要信息 | 请求日志、启动信息 |
| `WARN`  | 警告     | 配置缺失、性能下降 |
| `ERROR` | 错误     | 数据库连接失败     |

### 3. AppContext 模块 (`src/app_context.rs`)

应用上下文，集中管理配置和依赖。

**使用示例：**

```rust
use rust_web::{app_context::AppContext, config::ServerConfig};

// 方式 1: 快速创建
let config = ServerConfig::default();
let context = AppContext::from_config(config);

// 方式 2: 使用构建器（未来扩展）
let context = AppContext::builder()
    .server_config(config)
    .build()?;

// 访问配置
println!("服务器地址: {}", context.server_config.addr());
```

### 4. Server 模块 (`src/server.rs`)

HTTP 服务器和路由处理。

**主要组件：**

- `Server`: 服务器主结构
- `AppState`: 应用状态（包含 AppContext）
- Handler 函数：处理具体的 HTTP 请求

**使用示例：**

```rust
use rust_web::{server::Server, app_context::AppContext, config::ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig::default();
    let context = AppContext::from_config(config);
    let server = Server::new(context);

    server.run().await?;
    Ok(())
}
```

**路由：**

| 路由             | 方法 | 描述            |
| ---------------- | ---- | --------------- |
| `/health`        | GET  | 健康检查        |
| `/rerank`        | POST | 文档重排序      |
| `/sy/on/predict` | POST | Snappy 压缩预测 |

## 🔧 扩展指南

### 添加新的配置项

在 `src/config/types.rs` 中：

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    // 现有字段...

    // 添加新字段
    #[serde(default = "default_worker_threads")]
    pub worker_threads: usize,
}

fn default_worker_threads() -> usize {
    num_cpus::get()
}

impl ServerConfig {
    pub fn worker_threads(mut self, threads: usize) -> Self {
        self.worker_threads = threads;
        self
    }
}
```

### 添加新的路由

在 `src/server.rs` 中：

```rust
impl Server {
    pub fn build_app(&self) -> Router {
        // ...
        let protected_routes = Router::new()
            .route("/rerank", post(rerank))
            .route("/your_new_route", post(your_handler)); // 新路由
        // ...
    }
}

// 新的 handler
async fn your_handler(
    State(state): State<Arc<AppState>>,
    Json(body): Json<YourRequest>,
) -> Response {
    // 处理逻辑
}
```

### 添加依赖到 AppContext

在 `src/app_context.rs` 中：

```rust
pub struct AppContext {
    pub server_config: ServerConfig,
    pub database: Arc<Database>,  // 新依赖
}

impl AppContext {
    pub fn from_config(config: ServerConfig) -> Arc<Self> {
        let database = Arc::new(Database::connect(&config.db_url));
        Arc::new(Self {
            server_config: config,
            database,
        })
    }
}
```

## 📝 最佳实践

### 1. 配置管理

```rust
// ✅ 好的做法：使用 builder 模式
let config = ServerConfig::new()
    .host("0.0.0.0")
    .port(3000)
    .log_level("info");

// ✅ 好的做法：验证配置
config.validate()?;

// ❌ 避免：直接修改字段
config.port = 0;  // 可能导致无效配置
```

### 2. 日志使用

```rust
// ✅ 好的做法：结构化日志
info!(user_id = %user_id, action = "login", "User logged in");

// ✅ 好的做法：适当的日志级别
debug!("Processing item {}", item_id);  // 调试信息
info!("Request completed");             // 重要事件
warn!("Cache miss");                    // 潜在问题
error!("Database connection failed");   // 错误

// ❌ 避免：过度使用高级别日志
info!("Loop iteration {}", i);  // 应该用 debug!
```

### 3. 错误处理

```rust
// ✅ 好的做法：返回 Result
pub fn process() -> Result<(), Box<dyn std::error::Error>> {
    config.validate()?;
    // ...
    Ok(())
}

// ✅ 好的做法：记录错误
if let Err(e) = operation() {
    error!("Operation failed: {}", e);
    return Err(e);
}
```

## 🧪 测试

每个模块都包含单元测试：

```bash
# 运行所有测试
cargo test

# 运行特定模块测试
cargo test --lib config
cargo test --lib logging

# 查看测试覆盖率
cargo tarpaulin
```

## 📚 更多文档

- [README.md](README.md) - 快速开始
- [PERFORMANCE.md](PERFORMANCE.md) - 性能优化
- [LOGGING.md](LOGGING.md) - 日志详解
- [VSCODE_SETUP.md](VSCODE_SETUP.md) - 开发环境配置
