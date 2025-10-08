#!/bin/bash

# WarpGo - 深度清理脚本
# 清除所有缓存、依赖、编译产物和数据库

echo "🔥 开始深度清理项目..."
echo "⚠️  警告: 这将删除所有缓存和数据库!"
echo ""
read -p "是否继续? (y/N): " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ 取消清理"
    exit 1
fi

echo ""
echo "🧹 执行深度清理..."

# 清理 Node.js
echo "📦 清理 Node.js..."
rm -rf node_modules
rm -f package-lock.json

# 清理 Vite
echo "⚡ 清理 Vite..."
rm -rf .vite
rm -rf dist

# 清理 Rust/Tauri (最大的占用)
echo "🦀 清理 Rust 编译缓存..."
rm -rf src-tauri/target
rm -f src-tauri/Cargo.lock

# 清理全局 Cargo 缓存 (可选)
# echo "🗑️  清理全局 Cargo 缓存..."
# cargo clean 2>/dev/null

# 清理系统文件
echo "🍎 清理系统文件..."
find . -name ".DS_Store" -delete 2>/dev/null
find . -name "*.swp" -delete 2>/dev/null
find . -name "*.swo" -delete 2>/dev/null
find . -name "*~" -delete 2>/dev/null

# 清理临时文件
echo "🗑️  清理临时文件..."
find . -name "*.tmp" -delete 2>/dev/null
find . -name "*.log" -delete 2>/dev/null
find . -name "*.pid" -delete 2>/dev/null

# 清理数据库文件
echo "🗄️  清理数据库..."
rm -f accounts.db
rm -f accounts.db-shm
rm -f accounts.db-wal

# 清理构建产物
echo "📦 清理构建产物..."
rm -rf src-tauri/target/release/bundle
rm -rf src-tauri/target/debug

# 清理 IDE 缓存
echo "💻 清理 IDE 缓存..."
rm -rf .vscode/.history 2>/dev/null
rm -rf .idea 2>/dev/null

echo ""
echo "✅ 深度清理完成!"
echo ""
echo "📊 当前项目大小:"
du -sh . 2>/dev/null

echo ""
echo "📁 各目录大小:"
du -sh src src-tauri 2>/dev/null

echo ""
echo "💡 接下来:"
echo "   1. npm install           # 安装依赖"
echo "   2. npm run tauri dev     # 开发模式"
echo "   3. npm run tauri build   # 构建生产版本"
echo ""
