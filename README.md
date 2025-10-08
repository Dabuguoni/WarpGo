# WarpGo

<div align="center">

[![构建状态](https://github.com/Dabuguoni/WarpGo/workflows/构建和发布/badge.svg)](https://github.com/Dabuguoni/WarpGo/actions)
[![持续集成](https://github.com/Dabuguoni/WarpGo/workflows/持续集成/badge.svg)](https://github.com/Dabuguoni/WarpGo/actions)
[![GitHub release](https://img.shields.io/github/v/release/Dabuguoni/WarpGo)](https://github.com/Dabuguoni/WarpGo/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

[English](#english) | [中文](#中文)

A modern Warp account management tool built with Tauri, Rust and Vue 3.

基于 Tauri、Rust 和 Vue 3 构建的现代化 Warp 账户管理工具。

</div>
<img width="400" height="300" alt="8e06084d04e60a9366a03b47f4374dad" src="https://github.com/user-attachments/assets/3d1a6856-b460-4a59-97ac-ee79e160b676" />
<img width="400" height="300" alt="6e5a66a1dc0a6d4cfa77f55feca9d0c6" src="https://github.com/user-attachments/assets/25e825c4-9325-44e9-99ce-60c16e96fb9f" />

---

## English

### 📖 Table of Contents

- [Features](#features)
- [Tech Stack](#tech-stack)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Building](#building)
- [Cleaning](#cleaning)
- [FAQ](#faq)

### ✨ Features

- 🔐 **Account Management** - Add, delete, and switch between multiple Warp accounts
- 🔄 **Token Auto-Refresh** - Automatically refresh Firebase tokens
- 📊 **Quota Tracking** - Monitor account usage limits in real-time
- 🌐 **System Proxy** - Built-in proxy management with mitmproxy integration
- 🌍 **Multi-Language** - Supports English and Chinese
- 🎨 **Modern UI** - Clean, minimal design with smooth animations
- 🚀 **Native Performance** - Built with Rust for optimal speed
- 💾 **Local Storage** - SQLite database for secure local data storage
- 🔔 **Notifications** - Desktop notifications for important events
- ⚡ **Lightweight** - Small footprint, runs efficiently in the background

### 🛠️ Tech Stack

**Frontend:**
- Vue 3 (Composition API)
- TypeScript
- Vite
- Vue I18n (Internationalization)

**Backend:**
- Rust
- Tauri 2.x
- Rusqlite (SQLite)
- Reqwest (HTTP client)
- Tokio (Async runtime)

**Proxy:**
- mitmproxy
- Python 3.x (proxy script)

### 📋 Prerequisites

Before you begin, ensure you have the following installed:

1. **Node.js** (v18 or higher)
   ```bash
   node --version
   ```

2. **Rust** (v1.70 or higher)
   ```bash
   rustc --version
   ```

3. **pnpm/npm/yarn**
   ```bash
   npm --version
   ```

4. **mitmproxy** (Optional, for proxy features)
   ```bash
   # macOS
   brew install mitmproxy
   
   # Linux
   pip install mitmproxy
   
   # Windows
   # Download from: https://mitmproxy.org/
   ```

### 📥 Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/yourusername/warpgo.git
   cd warpgo
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Set up mitmproxy certificate** (if using proxy features)
   ```bash
   # Start mitmproxy once to generate certificates
   mitmproxy
   # Follow the instructions to trust the certificate
   ```

### 🚀 Usage

#### Development Mode

Run the application in development mode with hot-reload:

```bash
npm run tauri dev
```

#### Adding an Account

1. Click the **"Add Account"** button
2. Paste your Warp account JSON data
3. Click **"Add"** to save

**JSON Format Example:**
```json
{
  "email": "your@email.com",
  "token": "your-firebase-token",
  "account_id": "your-account-id"
}
```

#### Managing Accounts

- **Set Active**: Click the checkmark icon (✓) to set an account as active
- **Get Limits**: Click the download icon (⬇) to fetch current usage quotas
- **Delete**: Click the trash icon (🗑) to remove an account

#### Using Proxy

1. Set an account as active
2. Click **"Start Proxy"** button
3. System proxy will be configured automatically
4. Click **"Stop Proxy"** to disable

### 📦 Building

#### 本地构建

Build the application for production:

```bash
npm run tauri build
```

The built application will be available in:
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

#### 🚀 GitHub Actions 自动构建

项目已配置 GitHub Actions 自动化构建和发布：

**持续集成 (CI)**
- 每次推送到 `main`、`master`、`develop` 分支时自动运行
- 执行代码检查、类型检查和测试
- 快速构建验证

**自动发布**
- 创建新的 Git 标签时自动触发
- 同时构建 Windows、macOS、Linux 版本
- 自动创建 GitHub Release 并上传安装包

**发布新版本的步骤：**
```bash
# 1. 更新版本号
git tag v1.0.1

# 2. 推送标签
git push origin v1.0.1

# 3. GitHub Actions 将自动构建并发布
```

### 🧹 Cleaning

The project generates large cache files (~1.5GB) during compilation.

#### Deep Clean

Removes everything including cache, dependencies and database (requires confirmation):

```bash
./clean-deep.sh
```

This removes:
- `node_modules/`
- `dist/`
- `src-tauri/target/` (~1.4GB)
- `Cargo.lock`
- Database files
- All cache and temporary files

### ❓ FAQ

**Q: How do I get my Warp account JSON data?**  
A: You need to extract it from the Warp application's configuration file or API response. The JSON should include your email, token, and account ID.

**Q: Why do I need mitmproxy?**  
A: The proxy feature allows you to intercept and modify Warp API requests for advanced account management. It's optional if you only need basic features.

**Q: The app won't start. What should I do?**  
A: Make sure all prerequisites are installed and ports 1420 is not in use. Try running `./clean.sh` and reinstalling dependencies.

**Q: Can I use this on Windows/Linux?**  
A: Yes! Tauri is cross-platform. Follow the same installation steps for your OS.

**Q: Is my data secure?**  
A: Yes, all data is stored locally in an SQLite database on your machine. No data is sent to external servers.

### 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - For the amazing framework
- [Vue.js](https://vuejs.org/) - For the reactive UI framework
- [Rust](https://www.rust-lang.org/) - For the powerful backend language
- [mitmproxy](https://mitmproxy.org/) - For proxy functionality

---

## 中文

### 📖 目录

- [功能特性](#功能特性)
- [技术栈](#技术栈-1)
- [环境要求](#环境要求)
- [安装步骤](#安装步骤)
- [使用说明](#使用说明)
- [构建打包](#构建打包)
- [清理缓存](#清理缓存-1)
- [常见问题](#常见问题)

### ✨ 功能特性

- 🔐 **账户管理** - 添加、删除和切换多个 Warp 账户
- 🔄 **Token 自动刷新** - 自动刷新 Firebase 令牌
- 📊 **流量监控** - 实时监控账户使用限额
- 🌐 **系统代理** - 内置代理管理，集成 mitmproxy
- 🌍 **多语言支持** - 支持中文和英文
- 🎨 **现代化界面** - 简洁、美观的设计，流畅的动画效果
- 🚀 **原生性能** - 使用 Rust 构建，性能优异
- 💾 **本地存储** - SQLite 数据库安全存储本地数据
- 🔔 **桌面通知** - 重要事件的桌面通知提醒
- ⚡ **轻量级** - 占用空间小，后台高效运行

### 🛠️ 技术栈

**前端：**
- Vue 3 (组合式 API)
- TypeScript
- Vite
- Vue I18n (国际化)

**后端：**
- Rust
- Tauri 2.x
- Rusqlite (SQLite)
- Reqwest (HTTP 客户端)
- Tokio (异步运行时)

**代理：**
- mitmproxy
- Python 3.x (代理脚本)

### 📋 环境要求

开始之前，请确保已安装以下环境：

1. **Node.js** (v18 或更高版本)
   ```bash
   node --version
   ```

2. **Rust** (v1.70 或更高版本)
   ```bash
   rustc --version
   ```

3. **pnpm/npm/yarn**
   ```bash
   npm --version
   ```

4. **mitmproxy** (可选，用于代理功能)
   ```bash
   # macOS
   brew install mitmproxy
   
   # Linux
   pip install mitmproxy
   
   # Windows
   # 从此处下载: https://mitmproxy.org/
   ```

### 📥 安装步骤

1. **克隆仓库**
   ```bash
   git clone https://github.com/yourusername/warpgo.git
   cd warpgo
   ```

2. **安装依赖**
   ```bash
   npm install
   ```

3. **设置 mitmproxy 证书**（如果使用代理功能）
   ```bash
   # 首次运行 mitmproxy 生成证书
   mitmproxy
   # 按照提示信任证书
   ```

### 🚀 使用说明

#### 开发模式

以开发模式运行应用（支持热重载）：

```bash
npm run tauri dev
```

#### 添加账户

1. 点击**"添加账户"**按钮
2. 粘贴你的 Warp 账户 JSON 数据
3. 点击**"添加"**保存

**JSON 格式示例：**
```json
{
  "email": "your@email.com",
  "token": "your-firebase-token",
  "account_id": "your-account-id"
}
```

#### 管理账户

- **设为活动**：点击对勾图标 (✓) 将账户设为活动状态
- **获取限额**：点击下载图标 (⬇) 获取当前使用配额
- **删除账户**：点击垃圾桶图标 (🗑) 删除账户

#### 使用代理

1. 将一个账户设为活动状态
2. 点击**"启动代理"**按钮
3. 系统代理将自动配置
4. 点击**"停止代理"**停用

### 📦 构建打包

#### 本地构建

构建生产版本：

```bash
npm run tauri build
```

构建完成后，应用程序位于：
- **macOS**: `src-tauri/target/release/bundle/dmg/`
- **Windows**: `src-tauri/target/release/bundle/msi/`
- **Linux**: `src-tauri/target/release/bundle/appimage/`

#### 🚀 GitHub Actions 自动化构建

项目已配置完整的 GitHub Actions 工作流：

**持续集成 (CI)**
- 代码推送到主分支时自动运行
- 执行代码格式检查、类型检查和单元测试
- 快速构建验证确保代码质量

**自动发布流程**
- 创建新的 Git 标签时自动触发发布流程
- 并行构建 Windows、macOS、Linux 三个平台
- 自动创建 GitHub Release 并上传所有安装包

**发布新版本：**
```bash
# 1. 创建版本标签
git tag v1.0.1

# 2. 推送到远程仓库
git push origin v1.0.1

# 3. GitHub Actions 自动构建和发布
# 几分钟后即可在 Releases 页面下载安装包
```

### 🧹 清理缓存

项目编译过程中会产生大量缓存文件（约 1.5GB）。

#### 深度清理

删除所有缓存、依赖和数据库（需要确认）：

```bash
./clean-deep.sh
```

清理内容：
- `node_modules/`
- `dist/`
- `src-tauri/target/` (~1.4GB)
- `Cargo.lock`
- 数据库文件
- 所有缓存和临时文件

### ❓ 常见问题

**问：如何获取 Warp 账户的 JSON 数据？**  
答：你需要从 Warp 应用的配置文件或 API 响应中提取。JSON 应该包含你的邮箱、令牌和账户 ID。

**问：为什么需要 mitmproxy？**  
答：代理功能允许你拦截和修改 Warp API 请求，用于高级账户管理。如果只需要基本功能，可以不安装。

**问：应用无法启动怎么办？**  
答：确保所有环境要求已满足，且端口 1420 未被占用。尝试运行 `./clean.sh` 并重新安装依赖。

**问：可以在 Windows/Linux 上使用吗？**  
答：可以！Tauri 是跨平台的。按照相同的安装步骤即可。

**问：我的数据安全吗？**  
答：是的，所有数据都本地存储在 SQLite 数据库中。不会向外部服务器发送任何数据。

### 🙏 致谢

- [Tauri](https://tauri.app/) - 出色的框架
- [Vue.js](https://vuejs.org/) - 响应式 UI 框架
- [Rust](https://www.rust-lang.org/) - 强大的后端语言
- [mitmproxy](https://mitmproxy.org/) - 代理功能支持

---

<div align="center">

Made with ❤️ by WarpGo Team

[⬆ Back to Top](#warpgo)

</div>
