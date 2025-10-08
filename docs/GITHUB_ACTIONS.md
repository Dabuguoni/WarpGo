# 🚀 GitHub Actions 使用指南

本文档详细介绍了 WarpGo 项目中配置的 GitHub Actions 工作流程。

## 📋 工作流概览

### 1. 持续集成 (CI) - `ci.yml`
**触发条件:**
- 推送到 `main`、`master`、`develop` 分支
- 创建 Pull Request

**功能:**
- 代码格式检查 (`cargo fmt`, `cargo clippy`)
- TypeScript 类型检查
- 运行单元测试
- 快速构建验证

### 2. 构建和发布 - `build.yml`
**触发条件:**
- 推送到主分支
- Pull Request
- 发布 Release

**功能:**
- 跨平台构建 (Windows, macOS, Linux)
- 生成安装包
- 上传构建产物
- 自动发布到 GitHub Releases

### 3. 版本发布 - `release.yml`
**触发条件:**
- 推送 Git 标签 (格式: `v*`)

**功能:**
- 创建 GitHub Release
- 跨平台构建
- 自动上传所有平台的安装包

### 4. 依赖更新 - `dependabot.yml`
**触发条件:**
- 每周一自动运行
- 手动触发

**功能:**
- 检查 npm 依赖更新
- 检查 Cargo 依赖更新
- 创建更新报告 Issue

## 🔧 配置说明

### 环境变量
项目使用以下环境变量：
- `GITHUB_TOKEN`: GitHub 自动提供，用于访问仓库
- `CARGO_TERM_COLOR`: 设置为 `always`，启用彩色输出

### 缓存策略
为了加速构建，配置了以下缓存：

**Node.js 缓存:**
```yaml
- name: 设置 Node.js
  uses: actions/setup-node@v4
  with:
    node-version: '18'
    cache: 'npm'
    cache-dependency-path: WarpGo-manager/package-lock.json
```

**Rust 缓存:**
```yaml
- name: 缓存 Rust 依赖
  uses: actions/cache@v3
  with:
    path: |
      ~/.cargo/bin/
      ~/.cargo/registry/index/
      ~/.cargo/registry/cache/
      ~/.cargo/git/db/
      WarpGo-manager/src-tauri/target/
    key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
```

## 📦 发布流程

### 自动发布新版本

1. **更新版本号**
   ```bash
   # 在 WarpGo-manager/package.json 中更新版本
   # 在 WarpGo-manager/src-tauri/Cargo.toml 中更新版本
   # 在 WarpGo-manager/src-tauri/tauri.conf.json 中更新版本
   ```

2. **创建并推送标签**
   ```bash
   git add .
   git commit -m "chore: bump version to v1.0.1"
   git tag v1.0.1
   git push origin main
   git push origin v1.0.1
   ```

3. **自动构建**
   - GitHub Actions 自动检测到新标签
   - 并行构建三个平台的安装包
   - 创建 GitHub Release
   - 上传所有安装包

### 手动触发构建

可以通过 GitHub 网页界面手动触发工作流：

1. 进入仓库的 "Actions" 页面
2. 选择要运行的工作流
3. 点击 "Run workflow" 按钮

## 🛠️ 自定义配置

### 修改构建目标

在 `build.yml` 中修改构建矩阵：

```yaml
strategy:
  matrix:
    platform: [macos-latest, ubuntu-20.04, windows-latest]
```

### 添加新的检查

在 `ci.yml` 中添加新的检查步骤：

```yaml
- name: 自定义检查
  run: |
    # 添加你的检查命令
```

### 修改发布内容

在 `release.yml` 中自定义 Release 描述：

```yaml
body: |
  ## 🎉 WarpGo 新版本发布
  
  ### 📦 下载链接
  - **Windows**: 下载 `.msi` 或 `.exe` 文件
  - **macOS**: 下载 `.dmg` 文件
  - **Linux**: 下载 `.AppImage` 或 `.deb` 文件
```

## 🔍 故障排除

### 常见问题

1. **构建失败**
   - 检查依赖版本兼容性
   - 查看构建日志中的错误信息
   - 确保本地构建成功

2. **发布失败**
   - 检查标签格式是否正确 (`v*`)
   - 确保有足够的仓库权限
   - 检查文件路径是否正确

3. **缓存问题**
   - 可以在 Actions 页面清除缓存
   - 或者修改缓存键值强制重新构建

### 调试技巧

1. **启用调试日志**
   ```yaml
   env:
     ACTIONS_STEP_DEBUG: true
   ```

2. **使用 tmate 进行远程调试**
   ```yaml
   - name: Setup tmate session
     uses: mxschmitt/action-tmate@v3
   ```

## 📊 监控和通知

### 构建状态徽章

在 README 中添加的徽章会显示构建状态：

```markdown
[![构建状态](https://github.com/your-username/WarpGo/workflows/构建和发布/badge.svg)](https://github.com/your-username/WarpGo/actions)
```

### 通知设置

可以在 GitHub 设置中配置通知：
- 构建失败时发送邮件
- 成功发布时发送通知

## 🔒 安全考虑

1. **Secrets 管理**
   - 不要在工作流中硬编码敏感信息
   - 使用 GitHub Secrets 存储敏感数据

2. **权限控制**
   - 使用最小权限原则
   - 定期审查工作流权限

3. **依赖安全**
   - 定期更新 Actions 版本
   - 使用 Dependabot 监控依赖安全

---

如有任何问题，请查看 GitHub Actions 文档或创建 Issue 寻求帮助。
