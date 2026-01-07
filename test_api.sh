#!/bin/bash

# API Testing Script for Rust  Server
# Usage: ./test_api.sh

BASE_URL="http://127.0.0.1:3000"

echo "========================================="
echo "Testing Rust  Server APIs"
echo "========================================="
echo ""

# Test 1: Health Check (GET)
echo "1. Testing GET /health"
echo "   Request: curl -X GET ${BASE_URL}/health"
curl -s -X GET "${BASE_URL}/health"
echo -e "\n"

# Test 2: Rerank (POST)
echo "2. Testing POST /rerank"
echo "   Request: curl -X POST ${BASE_URL}/rerank"
curl -s -X POST "${BASE_URL}/rerank" \
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
  }' | jq '.'
echo ""

# Test 3: Rerank with different data (POST)
echo "3. Testing POST /rerank with different query"
echo "   Request: curl -X POST ${BASE_URL}/rerank"
curl -s -X POST "${BASE_URL}/rerank" \
  -H "Content-Type: application/json" \
  -d '{
    "model": "rerank-model-v2",
    "query": "Best programming languages",
    "documents": [
      "Rust is a systems programming language.",
      "Python is great for data science.",
      "JavaScript runs in the browser.",
      "Go is designed for concurrency."
    ]
  }' | jq '.'
echo ""

echo "========================================="
echo "Testing completed!"
echo "========================================="
echo ""
echo "Note: To test snappy compression endpoint, run:"
echo "  python3 test_snappy.py"
echo ""

