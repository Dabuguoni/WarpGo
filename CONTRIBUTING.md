# 🤝 贡献指南

感谢您对 WarpGo 项目的关注！我们欢迎各种形式的贡献。

## 📋 目录

- [行为准则](#行为准则)
- [如何贡献](#如何贡献)
- [开发环境设置](#开发环境设置)
- [提交规范](#提交规范)
- [Pull Request 流程](#pull-request-流程)
- [代码风格](#代码风格)

## 行为准则

参与此项目即表示您同意遵守我们的行为准则。请保持友善、尊重和包容的态度。

## 如何贡献

### 🐛 报告 Bug
- 使用 GitHub Issues 报告 bug
- 使用提供的 bug 报告模板
- 提供详细的重现步骤和环境信息

### ✨ 建议功能
- 使用 GitHub Issues 提出功能请求
- 使用提供的功能请求模板
- 详细描述功能的用途和价值

### 💻 代码贡献
- Fork 项目到您的 GitHub 账户
- 创建功能分支
- 提交您的更改
- 创建 Pull Request

## 开发环境设置

### 环境要求
- Node.js 18+
- Rust 1.70+
- Git

### 安装步骤
```bash
# 1. Fork 并克隆仓库
git clone https://github.com/your-username/WarpGo.git
cd WarpGo

# 2. 进入项目目录
cd WarpGo-manager

# 3. 安装依赖
npm install

# 4. 启动开发服务器
npm run tauri dev
```

### 项目结构
```
WarpGo-manager/
├── src/                 # Vue.js 前端代码
├── src-tauri/          # Rust 后端代码
│   ├── src/            # Rust 源码
│   └── Cargo.toml      # Rust 依赖配置
├── package.json        # Node.js 依赖配置
└── vite.config.ts      # Vite 构建配置
```

## 提交规范

我们使用 [Conventional Commits](https://www.conventionalcommits.org/) 规范：

```
<类型>[可选的作用域]: <描述>

[可选的正文]

[可选的脚注]
```

### 类型
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式化
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建过程或辅助工具的变动

### 示例
```
feat(ui): 添加账户切换动画效果

为账户列表添加平滑的切换动画，提升用户体验。

Closes #123
```

## Pull Request 流程

1. **创建分支**
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **开发和测试**
   - 编写代码
   - 添加测试（如果适用）
   - 确保所有测试通过

3. **提交更改**
   ```bash
   git add .
   git commit -m "feat: 添加新功能"
   ```

4. **推送分支**
   ```bash
   git push origin feature/your-feature-name
   ```

5. **创建 Pull Request**
   - 使用提供的 PR 模板
   - 详细描述更改内容
   - 链接相关的 Issues

6. **代码审查**
   - 响应审查意见
   - 根据反馈进行修改

## 代码风格

### Rust 代码
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 遵循 Rust 官方风格指南

### TypeScript/Vue 代码
- 使用 Prettier 格式化代码
- 遵循 Vue 3 组合式 API 最佳实践
- 使用 TypeScript 严格模式

### 提交前检查
```bash
# Rust 代码检查
cd WarpGo-manager/src-tauri
cargo fmt --all -- --check
cargo clippy -- -D warnings

# 前端代码检查
cd ..
npm run build
```

## 🙏 致谢

感谢所有贡献者的努力！您的贡献让 WarpGo 变得更好。

---

如有任何问题，请随时通过 GitHub Issues 联系我们。
