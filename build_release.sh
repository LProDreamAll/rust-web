#!/bin/bash

echo "========================================="
echo "Building Release Version (Optimized)"
echo "========================================="
echo ""

# 清理旧的 build
echo "1. Cleaning old builds..."
cargo clean --release

echo ""
echo "2. Building with optimizations (this may take a while)..."
time cargo build --release

if [ $? -eq 0 ]; then
    echo ""
    echo "========================================="
    echo "✅ Build successful!"
    echo "========================================="
    echo ""
    echo "Binary location:"
    echo "  ./target/release/rust-web"
    echo ""
    
    # 显示文件大小
    if [ -f "./target/release/rust-web" ]; then
        SIZE=$(du -h ./target/release/rust-web | cut -f1)
        echo "Binary size: $SIZE"
    fi
    
    echo ""
    echo "To run the optimized version:"
    echo "  ./target/release/rust-web"
    echo "or"
    echo "  cargo run --release"
    echo ""
else
    echo ""
    echo "❌ Build failed!"
    exit 1
fi

