#!/bin/bash

echo "========================================="
echo "Performance Benchmark: Dev vs Release"
echo "========================================="
echo ""

BASE_URL="http://127.0.0.1:3000"
REQUESTS=1000

# 检查 wrk 或 ab 是否安装
if command -v wrk &> /dev/null; then
    TOOL="wrk"
    echo "Using wrk for benchmarking..."
elif command -v ab &> /dev/null; then
    TOOL="ab"
    echo "Using Apache Bench (ab) for benchmarking..."
else
    echo "⚠️  Benchmark tools not found. Installing wrk is recommended:"
    echo "   macOS: brew install wrk"
    echo "   Linux: apt-get install wrk"
    echo ""
    echo "Falling back to simple curl timing test..."
    TOOL="curl"
fi

echo ""
echo "Please ensure the server is running on port 3000"
echo "Press Enter to start benchmark, or Ctrl+C to cancel..."
read

if [ "$TOOL" = "wrk" ]; then
    echo ""
    echo "Running wrk benchmark (10 seconds, 4 threads, 100 connections)..."
    echo ""
    wrk -t4 -c100 -d10s --latency "${BASE_URL}/health"
    
elif [ "$TOOL" = "ab" ]; then
    echo ""
    echo "Running Apache Bench ($REQUESTS requests, 10 concurrent)..."
    echo ""
    ab -n $REQUESTS -c 10 "${BASE_URL}/health"
    
else
    echo ""
    echo "Running simple timing test (100 requests)..."
    echo ""
    
    start=$(date +%s.%N)
    for i in {1..100}; do
        curl -s "${BASE_URL}/health" > /dev/null
    done
    end=$(date +%s.%N)
    
    runtime=$(echo "$end - $start" | bc)
    rps=$(echo "100 / $runtime" | bc)
    
    echo "Total time: ${runtime}s"
    echo "Requests per second: ~${rps}"
fi

echo ""
echo "========================================="
echo "💡 Tips:"
echo "========================================="
echo "Dev mode:     cargo run"
echo "Release mode: cargo run --release"
echo ""
echo "Expected improvement: 2-10x faster in release mode"
echo "========================================="

