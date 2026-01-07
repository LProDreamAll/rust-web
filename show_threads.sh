#!/bin/bash

echo "========================================="
echo "查看 Rust 服务的线程情况"
echo "========================================="
echo ""

# 查找进程 ID
PID=$(ps aux | grep "rust-web" | grep -v grep | awk '{print $2}' | head -1)

if [ -z "$PID" ]; then
    echo "❌ 服务未运行！"
    echo ""
    echo "请先启动服务："
    echo "  cargo run --release"
    exit 1
fi

echo "✓ 进程 ID: $PID"
echo ""

# macOS 系统
if [[ "$OSTYPE" == "darwin"* ]]; then
    echo "--- 线程列表 ---"
    ps -M $PID | head -20
    echo ""
    
    THREAD_COUNT=$(ps -M $PID | wc -l | xargs)
    echo "========================================="
    echo "📊 总线程数: $THREAD_COUNT"
    echo "========================================="
    
# Linux 系统
else
    echo "--- 线程列表 ---"
    ps -T -p $PID | head -20
    echo ""
    
    THREAD_COUNT=$(ps -T -p $PID | wc -l | xargs)
    echo "========================================="
    echo "📊 总线程数: $THREAD_COUNT"
    echo "========================================="
fi

echo ""
echo "💡 说明："
echo "  - tokio-runtime-w: Tokio 工作线程（处理异步任务）"
echo "  - tokio-blocking: 阻塞操作线程池"
echo "  - 主线程"
echo ""
echo "🚀 这些线程可以处理成千上万的并发连接！"
echo "========================================="

