# 实施计划

## 1. 项目初始化与基础设施

- [ ] 1.1 创建 Rust 后端项目结构
  - 创建 `backend/` 目录
  - 初始化 Cargo 项目
  - 添加依赖：axum, tokio, sqlx, jsonwebtoken, bcrypt, serde 等
  - 配置日志和错误处理

- [ ] 1.2 创建 Vue 3 前端项目
  - 使用 Vite 创建 Vue 3 + TypeScript 项目
  - 配置 TailwindCSS
  - 安装依赖：vue-router, pinia, axios, markdown-it 等
  - 配置 Vite 代理解决跨域

- [ ] 1.3 数据库迁移
  - 创建 SQLite 数据库
  - 执行 migrations/001_initial.sql 创建所有表
  - 创建 sqlx 离线查询文件

## 2. 后端核心实现

### 2.1 配置与错误处理

- [ ] 2.1.1 实现配置管理 (config.rs)
  - 从环境变量加载配置
  - 数据库连接配置
  - JWT 密钥配置
  - OAuth 配置

- [ ] 2.1.2 实现错误类型 (error.rs)
  - 定义 AppError 枚举
  - 实现 IntoResponse trait
  - 统一错误响应格式

### 2.2 数据模型层

- [ ] 2.2.1 实现 User 模型和仓库
  - 定义 User 结构体
  - 实现 UserRepository trait
  - 实现用户 CRUD 操作

- [ ] 2.2.2 实现 Post 模型和仓库
  - 定义 Post 结构体
  - 实现 PostRepository trait
  - 实现分页查询

- [ ] 2.2.3 实现 Comment 模型和仓库
  - 定义 Comment 结构体
  - 实现 CommentRepository trait
  - 支持楼中楼查询

- [ ] 2.2.4 实现 Vote/Bookmark/Follow 模型和仓库
  - 定义各结构体
  - 实现对应的 Repository

- [ ] 2.2.5 实现 Notification 模型和仓库
  - 定义 Notification 结构体
  - 实现未读计数功能

- [ ] 2.2.6 实现 Tag 模型和仓库
  - 定义 Tag 结构体
  - 实现标签搜索功能

### 2.3 服务层

- [ ] 2.3.1 实现认证服务 (auth_service.rs)
  - 用户注册（邮箱验证流程）
  - 用户登录（JWT 签发）
  - Token 刷新
  - 账户锁定逻辑

- [ ] 2.3.2 实现 OAuth 服务 (oauth/)
  - GitHub OAuth 流程
  - Google OAuth 流程
  - 用户信息获取和创建

- [ ] 2.3.3 实现用户服务 (user_service.rs)
  - 获取用户资料
  - 更新个人资料
  - 关注/取消关注

- [ ] 2.3.4 实现帖子服务 (post_service.rs)
  - 创建帖子
  - 更新/删除帖子
  - 点赞/踩逻辑
  - 收藏逻辑

- [ ] 2.3.5 实现评论服务 (comment_service.rs)
  - 添加评论
  - 编辑/删除评论
  - 楼中楼回复

- [ ] 2.3.6 实现通知服务 (notification_service.rs)
  - 创建通知
  - 标记已读
  - WebSocket 推送集成

- [ ] 2.3.7 实现管理服务 (admin_service.rs)
  - 用户管理
  - 内容审核
  - 举报处理
  - 统计功能

### 2.4 API 路由层

- [ ] 2.4.1 实现认证路由 (routes/auth.rs)
  - POST /auth/register
  - POST /auth/login
  - POST /auth/verify-email
  - GET /auth/github
  - GET /auth/github/callback
  - GET /auth/google
  - GET /auth/google/callback
  - POST /auth/refresh

- [ ] 2.4.2 实现用户路由 (routes/user.rs)
  - GET /users/:username
  - PATCH /users/me
  - GET /users/:username/posts
  - GET /users/:username/followers
  - GET /users/:username/following

- [ ] 2.4.3 实现帖子路由 (routes/post.rs)
  - GET /posts
  - GET /posts/:id
  - POST /posts
  - PATCH /posts/:id
  - DELETE /posts/:id

- [ ] 2.4.4 实现评论路由 (routes/comment.rs)
  - GET /posts/:post_id/comments
  - POST /posts/:post_id/comments
  - PATCH /comments/:id
  - DELETE /comments/:id

- [ ] 2.4.5 实现投票/收藏路由
  - POST /votes
  - DELETE /votes/:post_id
  - GET /users/me/bookmarks
  - POST /bookmarks
  - DELETE /bookmarks/:post_id
  - POST /follow/:user_id
  - DELETE /follow/:user_id

- [ ] 2.4.6 实现标签和通知路由
  - GET /tags
  - GET /tags/:name
  - GET /notifications
  - PATCH /notifications/:id/read
  - PATCH /notifications/read-all

- [ ] 2.4.7 实现管理后台路由 (routes/admin.rs)
  - GET /admin/stats
  - GET /admin/users
  - PATCH /admin/users/:id/status
  - GET /admin/posts/pending
  - PATCH /admin/posts/:id/approve
  - GET /admin/reports
  - PATCH /admin/reports/:id

### 2.5 WebSocket 实现

- [ ] 2.5.1 实现 WebSocket 处理器 (ws/)
  - 连接管理 (client.rs)
  - 消息广播 (broadcast.rs)
  - 心跳机制

- [ ] 2.5.2 实现 WebSocket 路由 (routes/ws.rs)
  - 连接升级
  - 认证中间件
  - 消息订阅

### 2.6 中间件

- [ ] 2.6.1 实现 JWT 认证中间件 (middleware/auth.rs)
  - Token 验证
  - 用户上下文注入

- [ ] 2.6.2 实现 CORS 中间件 (middleware/cors.rs)
  - 跨域配置

## 3. 前端核心实现

### 3.1 项目配置

- [ ] 3.1.1 配置 Vite 和代理
  - 配置 API 代理到后端
  - 配置 WebSocket 代理

- [ ] 3.1.2 配置 TailwindCSS
  - 配置主题颜色
  - 配置字体
  - 添加动画类

- [ ] 3.1.3 配置 Vue Router
  - 路由配置
  - 导航守卫

- [ ] 3.1.4 配置 Pinia Store
  - auth store
  - user store
  - post store
  - notification store
  - ws store

### 3.2 公共组件

- [ ] 3.2.1 布局组件 (components/layout/)
  - Navbar 导航栏
  - Sidebar 侧边栏
  - Footer 页脚

- [ ] 3.2.2 通用组件 (components/common/)
  - Button 按钮
  - Input 输入框
  - Modal 模态框
  - Toast 提示
  - Loading 加载
  - Empty 空状态

- [ ] 3.2.3 用户组件 (components/user/)
  - UserAvatar 用户头像
  - UserCard 用户卡片

- [ ] 3.2.4 帖子组件 (components/post/)
  - PostCard 帖子卡片
  - PostList 帖子列表
  - PostEditor 帖子编辑器

- [ ] 3.2.5 评论组件 (components/comment/)
  - CommentItem 评论项
  - CommentList 评论列表

### 3.3 页面视图

- [ ] 3.3.1 认证页面
  - Login 登录页
  - Register 注册页

- [ ] 3.3.2 首页和帖子页面
  - Home 首页
  - PostDetail 帖子详情
  - CreatePost 创建帖子

- [ ] 3.3.3 用户页面
  - Profile 用户主页
  - Settings 设置页
  - Bookmarks 收藏页
  - Followers 粉丝页
  - Following 关注页

- [ ] 3.3.4 标签页面
  - Tags 标签广场
  - TagPosts 标签帖子列表

- [ ] 3.3.5 通知页面
  - Notifications 通知中心

- [ ] 3.3.6 管理后台页面
  - Dashboard 统计看板
  - UserManagement 用户管理
  - ContentModeration 内容审核
  - ReportHandling 举报处理

### 3.4 组合式函数

- [ ] 3.4.1 实现 API 客户端 (composables/useApi.ts)
  - axios 实例
  - 请求拦截器
  - 响应拦截器
  - Token 刷新逻辑

- [ ] 3.4.2 实现 WebSocket 客户端 (composables/useWebSocket.ts)
  - 连接管理
  - 自动重连
  - 消息处理

- [ ] 3.4.3 实现 Toast 提示 (composables/useToast.ts)
  - 成功/错误/警告提示

### 3.5 样式和动画

- [ ] 3.5.1 实现页面过渡动画
  - fade + slide 过渡效果
  - 路由过渡动画

- [ ] 3.5.2 实现加载动画
  - 骨架屏组件
  - 旋转加载器

- [ ] 3.5.3 实现悬停动效
  - 按钮悬停效果
  - 卡片悬停效果

## 4. 集成测试

- [ ] 4.1 后端 API 测试
  - 认证 API 测试
  - 帖子 CRUD 测试
  - 评论功能测试
  - WebSocket 连接测试

- [ ] 4.2 前端集成测试
  - 页面渲染测试
  - 路由跳转测试
  - WebSocket 消息测试

## 5. 检查点

确保所有核心功能实现完成：
- 用户注册和登录
- OAuth 第三方登录
- 帖子发布和浏览
- 评论和回复
- 点赞/踩/收藏/关注
- WebSocket 实时通知
- 管理后台

## 6. 部署配置

- [ ] 6.1 配置生产环境构建
  - 前端生产构建
  - 后端 Release 构建

- [ ] 6.2 创建启动脚本
  - 一键启动脚本
