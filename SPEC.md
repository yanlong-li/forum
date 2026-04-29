# Community Forum - 社区论坛系统

## 1. 项目概述

**项目名称**: Community Forum
**项目类型**: 前后端分离的全栈 Web 应用
**技术栈**:
- 后端: Rust + Axum
- 前端: Vue 3 + TailwindCSS + TypeScript
- 数据库: SQLite
- 实时通信: WebSocket
**核心功能**: 现代化社区论坛平台，支持实时互动、第三方登录（GitHub/Google）和完整的用户社交体系

## 2. 技术架构

### 2.1 系统架构

```
┌─────────────────┐     ┌─────────────────┐
│   Frontend      │     │   Backend       │
│   Vue 3         │────▶│   Rust/Axum     │
│   TailwindCSS  │     │   SQLite        │
└─────────────────┘     └─────────────────┘
         │                       │
         │     WebSocket         │
         └───────────────────────┘
```

### 2.2 项目结构

```
/workspace
├── backend/                    # Rust 后端项目
│   ├── src/
│   │   ├── main.rs            # 应用入口
│   │   ├── config.rs           # 配置管理
│   │   ├── error.rs            # 错误定义
│   │   ├── extractors.rs       # 请求提取器
│   │   ├── middleware/         # 中间件
│   │   ├── routes/             # API 路由
│   │   ├── services/           # 业务逻辑层
│   │   ├── repositories/        # 数据访问层
│   │   ├── models/             # 数据模型
│   │   ├── ws/                 # WebSocket 处理
│   │   └── oauth/              # OAuth 第三方登录
│   ├── Cargo.toml
│   └── migrations/            # 数据库迁移
│
├── frontend/                   # Vue 3 前端项目
│   ├── src/
│   │   ├── main.ts            # 应用入口
│   │   ├── App.vue            # 根组件
│   │   ├── router/            # 路由配置
│   │   ├── stores/             # Pinia 状态管理
│   │   ├── composables/       # 组合式函数
│   │   ├── components/        # 公共组件
│   │   ├── views/             # 页面视图
│   │   └── styles/            # 样式
│   ├── package.json
│   ├── vite.config.ts
│   └── tailwind.config.js
│
├── start.sh                   # 启动脚本
└── SPEC.md                    # 项目规格说明
```

## 3. 功能列表

### 3.1 用户认证

- [x] 邮箱 + 密码注册和登录
- [x] JWT Token 认证
- [x] GitHub OAuth 登录
- [x] Google OAuth 登录
- [x] 账户锁定机制（连续失败 5 次）
- [x] Token 刷新机制

### 3.2 用户资料

- [x] 查看用户公开资料
- [x] 编辑个人资料（头像、用户名、个人简介）
- [x] 查看用户帖子列表

### 3.3 帖子功能

- [x] 创建帖子（支持 Markdown）
- [x] 编辑和删除帖子
- [x] 帖子列表（分页、标签筛选）
- [x] 帖子详情

### 3.4 评论功能

- [x] 添加评论
- [x] 楼中楼回复
- [x] 编辑和删除评论
- [x] 评论列表

### 3.5 互动功能

- [x] 点赞/踩帖子和评论
- [x] 收藏帖子
- [x] 关注/取消关注用户
- [x] 标签系统

### 3.6 实时通知

- [x] WebSocket 连接
- [x] 新评论通知
- [x] 新粉丝通知
- [x] 点赞/踩通知
- [x] 回复通知

### 3.7 管理后台

- [x] 统计看板
- [x] 用户管理（禁用/启用账户）
- [x] 内容审核
- [x] 举报处理

### 3.8 前端 UI/UX

- [x] 响应式布局
- [x] 页面过渡动画（淡入淡出 + 滑动）
- [x] 悬停动效
- [x] 骨架屏加载
- [x] Toast 通知
- [x] 无限滚动

## 4. API 端点

### 4.1 认证
- `POST /api/v1/auth/register` - 用户注册
- `POST /api/v1/auth/login` - 用户登录
- `POST /api/v1/auth/refresh` - 刷新 Token
- `GET /api/v1/auth/github` - GitHub OAuth
- `GET /api/v1/auth/google` - Google OAuth

### 4.2 用户
- `GET /api/v1/users/me` - 获取当前用户
- `PATCH /api/v1/users/me` - 更新当前用户
- `GET /api/v1/users/:username` - 获取用户资料
- `GET /api/v1/users/:username/posts` - 获取用户帖子
- `GET /api/v1/users/:username/followers` - 获取粉丝列表
- `GET /api/v1/users/:username/following` - 获取关注列表

### 4.3 帖子
- `GET /api/v1/posts` - 获取帖子列表
- `GET /api/v1/posts/:id` - 获取帖子详情
- `POST /api/v1/posts` - 创建帖子
- `PATCH /api/v1/posts/:id` - 更新帖子
- `DELETE /api/v1/posts/:id` - 删除帖子

### 4.4 评论
- `GET /api/v1/comments/posts/:post_id/comments` - 获取评论列表
- `POST /api/v1/comments/posts/:post_id/comments` - 添加评论
- `PATCH /api/v1/comments/:id` - 编辑评论
- `DELETE /api/v1/comments/:id` - 删除评论

### 4.5 投票和收藏
- `POST /api/v1/votes` - 投票
- `DELETE /api/v1/votes/:post_id` - 取消投票
- `GET /api/v1/bookmarks` - 获取收藏列表
- `POST /api/v1/bookmarks` - 添加收藏
- `DELETE /api/v1/bookmarks/:post_id` - 取消收藏

### 4.6 关注
- `POST /api/v1/follow/:user_id` - 关注用户
- `DELETE /api/v1/follow/:user_id` - 取消关注

### 4.7 标签
- `GET /api/v1/tags` - 获取标签列表
- `GET /api/v1/tags/:name` - 获取标签详情
- `GET /api/v1/tags/:name/posts` - 获取标签下的帖子

### 4.8 通知
- `GET /api/v1/notifications` - 获取通知列表
- `PATCH /api/v1/notifications/:id/read` - 标记已读
- `PATCH /api/v1/notifications/read-all` - 全部已读

### 4.9 管理后台
- `GET /api/v1/admin/stats` - 获取统计数据
- `GET /api/v1/admin/users` - 用户列表
- `PATCH /api/v1/admin/users/:id/status` - 更新用户状态
- `GET /api/v1/admin/reports` - 举报列表
- `PATCH /api/v1/admin/reports/:id` - 处理举报

## 5. 数据库模型

### 5.1 主要表

- **users** - 用户表
- **oauth_accounts** - OAuth 账户表
- **posts** - 帖子表
- **post_tags** - 帖子-标签关联表
- **tags** - 标签表
- **comments** - 评论表
- **votes** - 投票表
- **bookmarks** - 收藏表
- **follows** - 关注表
- **notifications** - 通知表
- **reports** - 举报表

## 6. 启动说明

### 6.1 环境要求

- Rust 1.70+
- Node.js 18+
- npm 9+

### 6.2 启动步骤

1. 安装依赖：
   ```bash
   # 安装 Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # 安装前端依赖
   cd frontend
   npm install
   ```

2. 配置环境变量（可选）：
   ```bash
   # 后端环境变量
   export DATABASE_URL="sqlite:forum.db?mode=rwc"
   export JWT_SECRET="your-secret-key"
   export GITHUB_CLIENT_ID="your-github-client-id"
   export GITHUB_CLIENT_SECRET="your-github-client-secret"
   export GOOGLE_CLIENT_ID="your-google-client-id"
   export GOOGLE_CLIENT_SECRET="your-google-client-secret"
   ```

3. 启动服务：
   ```bash
   # 启动后端（端口 3000）
   cd backend
   cargo run --release

   # 启动前端（端口 5173）
   cd frontend
   npm run dev
   ```

   或者使用启动脚本：
   ```bash
   chmod +x start.sh
   ./start.sh
   ```

## 7. 注意事项

- 生产环境请修改 `JWT_SECRET` 和数据库连接字符串
- OAuth 登录需要配置相应的 OAuth App 凭据
- 前端默认代理 API 请求到 `http://localhost:3000`
