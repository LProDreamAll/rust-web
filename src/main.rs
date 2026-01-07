use rust_web::{
    app_context::AppContext,
    config::ServerConfig,
    observability::logging::{self, LoggingConfig},
    server::Server,
};
use tracing::{info, Level};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    let log_config = LoggingConfig::new().level(Level::INFO).colorize(true);
    logging::init_logging(log_config);

    info!("Rust Web Server starting...");

    let config = ServerConfig::default();
    println!("Host: {}:{}", config.host, config.port);

    let context = AppContext::from_config(config);
    let server = Server::new(context);

    // 创建默认的 Tokio 运行时（自动使用所有 CPU 核心）
    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async move { server.run().await })?;

    Ok(())
}
