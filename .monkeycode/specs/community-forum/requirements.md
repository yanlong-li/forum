# Requirements Document

## Introduction

项目名称：Community Forum（社区论坛系统）
项目类型：前后端分离的全栈 Web 应用
核心功能：现代化社区论坛平台，支持实时互动、第三方登录和完善的用户社交体系

## Glossary

- **用户 (User)**：在论坛注册并登录的成员
- **帖子 (Post)**：用户在论坛发布的主题内容
- **评论 (Comment)**：用户对帖子的回复内容
- **楼中楼 (Reply)**：用户对评论的再次回复
- **点赞/踩 (Like/Dislike)**：用户对帖子或评论的情感表达
- **收藏 (Bookmark)**：用户收藏的帖子
- **关注 (Follow)**：用户之间的关注关系
- **标签 (Tag)**：用于分类帖子的关键词
- **消息 (Notification)**：系统推送给用户的通知
- **WebSocket**：双向实时通信协议
- **OAuth2**：第三方授权开放标准

## Requirements

### RQ-001: 用户认证系统

**User Story:** 作为访客，我想要注册和登录论坛，以便我可以发帖、评论和互动

#### Acceptance Criteria

1. WHEN 用户提交邮箱和密码注册表单 THEN 系统 SHALL 验证邮箱格式和密码强度（至少8字符，包含大小写字母和数字），然后创建新账户并发送邮箱验证链接
2. WHEN 用户点击邮箱验证链接 THEN 系统 SHALL 激活账户并允许登录
3. WHEN 用户提交正确的邮箱和密码 THEN 系统 SHALL 允许用户登录并签发 JWT 访问令牌
4. WHEN 用户使用 GitHub 或 Google 账号登录时 THEN 系统 SHALL 通过 OAuth2 流程获取用户信息，为新用户自动创建账户，返回 JWT 令牌
5. IF 用户密码连续失败 5 次 THEN 系统 SHALL 锁定账户 30 分钟
6. IF JWT 令牌过期或无效 THEN 系统 SHALL 返回 401 状态码，要求用户重新登录

### RQ-002: 用户个人资料

**User Story:** 作为登录用户，我想要管理我的个人资料，以便其他用户了解我

#### Acceptance Criteria

1. WHEN 用户访问个人资料页面 THEN 系统 SHALL 显示用户名、头像、个人简介、注册时间、发帖数和关注者/关注数
2. WHEN 用户编辑个人资料 THEN 系统 SHALL 允许修改头像（上传或裁剪）、用户名、 个人简介
3. WHEN 其他用户访问某用户主页 THEN 系统 SHALL 显示该用户的公开资料和最近发布的帖子列表

### RQ-003: 帖子发布与浏览

**User Story:** 作为登录用户，我想要发布帖子和浏览其他用户的帖子，以便在社区中分享内容和获取信息

#### Acceptance Criteria

1. WHEN 用户点击"发布新帖"按钮 THEN 系统 SHALL 显示包含标题、正文编辑器（支持 Markdown）、标签选择的发布表单
2. WHEN 用户提交帖子表单 THEN 系统 SHALL 验证标题不为空、正文至少10字符、至少选择一个标签，然后保存帖子
3. WHEN 用户访问首页 THEN 系统 SHALL 显示帖子列表，按发布时间倒序排列，每页显示 20 条，包含标题、摘要、作者、发布时间、点赞数和评论数
4. WHEN 用户点击帖子标题 THEN 系统 SHALL 跳转到帖子详情页，显示完整内容和所有评论
5. WHEN 用户按标签筛选帖子 THEN 系统 SHALL 显示该标签下的所有帖子

### RQ-004: 评论与回复系统

**User Story:** 作为登录用户，我想要对帖子发表评论和回复，以便与其他用户互动

#### Acceptance Criteria

1. WHEN 用户在帖子详情页提交评论 THEN 系统 SHALL 保存评论并通过 WebSocket 实时通知帖子作者
2. WHEN 用户对某条评论点击"回复" THEN 系统 SHALL 显示嵌套的回复表单，回复将显示在对应评论下方（楼中楼形式）
3. WHEN 评论被发布时 IF 该评论的父评论作者在线 THEN 系统 SHALL 通过 WebSocket 实时推送通知
4. WHEN 用户编辑自己的评论 THEN 系统 SHALL 保存修改后的内容，显示编辑标识和编辑时间
5. IF 用户删除自己的评论 THEN 系统 SHALL 将评论标记为"已删除"并显示"[已删除]"占位符

### RQ-005: 点赞与踩

**User Story:** 作为登录用户，我想要对帖子和评论点赞或踩，以便表达我的态度

#### Acceptance Criteria

1. WHEN 用户点击帖子或评论的"点赞"按钮 THEN 系统 SHALL 记录点赞并更新点赞数，然后通过 WebSocket 实时更新显示
2. WHEN 用户再次点击"点赞"按钮 THEN 系统 SHALL 取消点赞并更新点赞数
3. WHEN 用户点击"踩"按钮 THEN 系统 SHALL 记录踩并更新踩数，每个用户对每个帖子/评论只能选择点赞或踩之一
4. IF 用户已点赞后点击"踩" THEN 系统 SHALL 切换为踩（取消点赞，添加踩）

### RQ-006: 收藏功能

**User Story:** 作为登录用户，我想要收藏帖子，以便以后查看

#### Acceptance Criteria

1. WHEN 用户点击帖子旁边的"收藏"按钮 THEN 系统 SHALL 将帖子添加到用户的收藏列表
2. WHEN 用户再次点击"收藏"按钮 THEN 系统 SHALL 从收藏列表中移除该帖子
3. WHEN 用户访问"我的收藏"页面 THEN 系统 SHALL 显示用户收藏的所有帖子列表

### RQ-007: 关注系统

**User Story:** 作为登录用户，我想要关注其他用户，以便获取他们的最新动态

#### Acceptance Criteria

1. WHEN 用户点击某用户资料页的"关注"按钮 THEN 系统 SHALL 建立关注关系并更新关注数
2. WHEN 被关注用户发布新帖子 THEN 系统 SHALL 将该帖子添加到其粉丝的"关注动态"信息流中
3. WHEN 用户访问"我的关注"页面 THEN 系统 SHALL 显示用户关注的所有用户列表
4. WHEN 用户访问"我的粉丝"页面 THEN 系统 SHALL 显示关注该用户的所有用户列表

### RQ-008: 标签系统

**User Story:** 作为登录用户，我想要使用标签来组织和发现帖子

#### Acceptance Criteria

1. WHEN 用户发布帖子时选择标签 THEN 系统 SHALL 将标签与帖子关联
2. WHEN 用户点击某个标签 THEN 系统 SHALL 显示所有使用该标签的帖子列表
3. WHEN 用户访问"标签广场"页面 THEN 系统 SHALL 显示热门标签列表和帖子数量统计

### RQ-009: 消息通知系统

**User Story:** 作为登录用户，我想要收到新评论、新粉丝等通知，以便及时了解社区动态

#### Acceptance Criteria

1. WHEN 其他用户评论我的帖子时 THEN 系统 SHALL 创建评论通知并通过 WebSocket 实时推送给帖子作者
2. WHEN 其他用户关注我时 THEN 系统 SHALL 创建关注通知并通过 WebSocket 实时推送
3. WHEN 其他用户点赞或踩我的帖子/评论时 THEN 系统 SHALL 创建互动通知并通过 WebSocket 实时推送
4. WHEN 其他用户回复我的评论时 THEN 系统 SHALL 创建回复通知并通过 WebSocket 实时推送
5. WHEN 用户访问通知页面 THEN 系统 SHALL 显示所有未读和已读通知，支持标记已读
6. IF 用户有未读通知 THEN 系统 SHALL 在导航栏显示未读数量角标

### RQ-010: WebSocket 实时通信

**User Story:** 作为在线用户，我想要实时收到新动态推送，以便及时了解社区最新信息

#### Acceptance Criteria

1. WHEN 用户登录成功 THEN 系统 SHALL 建立 WebSocket 连接并保持心跳
2. WHEN 页面有新内容发布时 IF 发布者的粉丝在线 THEN 系统 SHALL 通过 WebSocket 推送新帖子动态
3. WHEN 有人评论用户关注的帖子时 THEN 系统 SHALL 通过 WebSocket 推送评论通知
4. WHEN 用户的帖子被点赞/踩时 THEN 系统 SHALL 通过 WebSocket 推送互动通知
5. IF WebSocket 连接断开 THEN 系统 SHALL 自动尝试重连，最多重试 5 次，间隔 2 秒

### RQ-011: 管理员后台

**User Story:** 作为管理员，我想要管理用户和内容，以维护社区秩序

#### Acceptance Criteria

1. WHEN 管理员登录时 THEN 系统 SHALL 显示管理后台，包含用户管理、内容审核、举报处理、统计看板等功能入口
2. WHEN 管理员访问用户管理页面 THEN 系统 SHALL 显示用户列表，支持搜索、禁用/启用账户
3. WHEN 管理员访问内容审核页面 THEN 系统 SHALL 显示待审核的帖子和评论列表，支持通过或删除
4. WHEN 用户举报某个帖子或评论时 THEN 系统 SHALL 创建举报记录，管理员可查看和处理举报
5. WHEN 管理员访问统计看板 THEN 系统 SHALL 显示日活用户数、发帖数、评论数、注册用户数等统计图表

### RQ-012: 前端 UI/UX 设计

**User Story:** 作为访客/用户，我想要一个美观、流畅的界面，以便愉快地使用论坛

#### Acceptance Criteria

1. WHEN 用户访问网站时 THEN 系统 SHALL 加载现代简洁的主页，包含导航栏、热门帖子、侧边栏（标签、活跃用户）
2. WHEN 用户进行页面跳转时 THEN 系统 SHALL 使用淡入淡出过渡动画，时长 300ms
3. WHEN 用户悬停可点击元素时 THEN 系统 SHALL 显示悬停状态（颜色变化、微上浮效果）
4. WHEN 页面加载数据时 THEN 系统 SHALL 显示优雅的加载动画（骨架屏或旋转加载器）
5. IF 用户提交表单时出现错误 THEN 系统 SHALL 显示友好的错误提示，不生硬打断用户操作
6. IF 用户操作成功 THEN 系统 SHALL 显示成功提示，2 秒后自动消失
7. WHEN 页面滚动到底部 THEN 系统 SHALL 自动加载更多内容（无限滚动）
8. WHEN 用户在移动端访问时 THEN 系统 SHALL 适配移动端布局，显示汉堡菜单

### RQ-013: 安全性要求

**User Story:** 作为系统，我需要保障用户数据和通信安全

#### Acceptance Criteria

1. WHEN 用户登录或注册时 THEN 系统 SHALL 使用 HTTPS 加密所有通信
2. WHEN 存储用户密码时 THEN 系统 SHALL 使用 bcrypt 算法哈希存储，不明文保存
3. WHEN 生成 JWT 令牌时 THEN 系统 SHALL 设置合理的过期时间（访问令牌 24 小时，刷新令牌 7 天）
4. WHEN 处理用户输入时 THEN 系统 SHALL 防止 XSS 攻击，对特殊字符进行转义
5. WHEN 操作数据库时 THEN 系统 SHALL 使用参数化查询，防止 SQL 注入
6. IF 用户尝试访问未授权资源 THEN 系统 SHALL 返回 403 状态码

### RQ-014: 性能要求

**User Story:** 作为系统，我需要在合理的时间内响应用户请求

#### Acceptance Criteria

1. WHEN 首页加载时 THEN 系统 SHALL 在 2 秒内完成首屏内容展示
2. WHEN 用户提交帖子表单时 THEN 系统 SHALL 在 1 秒内保存并返回结果
3. WHEN WebSocket 推送消息时 THEN 系统 SHALL 在 500ms 内将消息送达在线用户
4. IF 系统负载较高时 THEN 系统 SHALL 通过缓存和数据库索引保持响应速度

## Data Models

### User
- id: UUID (主键)
- username: VARCHAR(50) 唯一用户名
- email: VARCHAR(255) 唯一邮箱
- password_hash: VARCHAR(255) 密码哈希（可为 NULL，仅第三方登录时）
- avatar_url: VARCHAR(500) 头像 URL
- bio: TEXT 个人简介
- email_verified: BOOLEAN 邮箱是否已验证
- is_admin: BOOLEAN 是否为管理员
- is_locked: BOOLEAN 账户是否被锁定
- failed_login_attempts: INTEGER 连续失败登录次数
- locked_until: TIMESTAMP 锁定到期时间
- created_at: TIMESTAMP 注册时间
- updated_at: TIMESTAMP 更新时间

### OAuthAccount
- id: UUID (主键)
- user_id: UUID (外键 -> User)
- provider: VARCHAR(20) OAuth 提供商 (github/google)
- provider_user_id: VARCHAR(255) 第三方用户 ID
- created_at: TIMESTAMP 关联时间

### Post
- id: UUID (主键)
- author_id: UUID (外键 -> User)
- title: VARCHAR(200) 帖子标题
- content: TEXT 帖子正文 (Markdown)
- is_deleted: BOOLEAN 是否已删除
- created_at: TIMESTAMP 发布时间
- updated_at: TIMESTAMP 更新时间

### PostTag (多对多关联)
- post_id: UUID (外键 -> Post)
- tag_id: UUID (外键 -> Tag)

### Tag
- id: UUID (主键)
- name: VARCHAR(50) 唯一标签名
- post_count: INTEGER 帖子数量统计
- created_at: TIMESTAMP 创建时间

### Comment
- id: UUID (主键)
- post_id: UUID (外键 -> Post)
- author_id: UUID (外键 -> User)
- parent_id: UUID (外键 -> Comment，可为 NULL，表示一级评论)
- content: TEXT 评论内容
- is_deleted: BOOLEAN 是否已删除
- created_at: TIMESTAMP 评论时间
- updated_at: TIMESTAMP 更新时间

### Vote
- id: UUID (主键)
- user_id: UUID (外键 -> User)
- post_id: UUID (外键 -> Post，可为 NULL)
- comment_id: UUID (外键 -> Comment，可为 NULL)
- value: SMALLINT (1=点赞, -1=踩)
- created_at: TIMESTAMP 投票时间
- UNIQUE constraint on (user_id, post_id) and (user_id, comment_id)

### Bookmark
- id: UUID (主键)
- user_id: UUID (外键 -> User)
- post_id: UUID (外键 -> Post)
- created_at: TIMESTAMP 收藏时间
- UNIQUE constraint on (user_id, post_id)

### Follow
- id: UUID (主键)
- follower_id: UUID (外键 -> User, 关注者)
- following_id: UUID (外键 -> User, 被关注者)
- created_at: TIMESTAMP 关注时间
- UNIQUE constraint on (follower_id, following_id)

### Notification
- id: UUID (主键)
- user_id: UUID (外键 -> User, 接收通知的用户)
- type: VARCHAR(30) 通知类型 (comment/like/follow/reply)
- actor_id: UUID (外键 -> User, 触发通知的用户)
- post_id: UUID (外键 -> Post，可为 NULL)
- comment_id: UUID (外键 -> Comment，可为 NULL)
- is_read: BOOLEAN 是否已读
- created_at: TIMESTAMP 通知时间

### Report
- id: UUID (主键)
- reporter_id: UUID (外键 -> User)
- post_id: UUID (外键 -> Post，可为 NULL)
- comment_id: UUID (外键 -> Comment，可为 NULL)
- reason: TEXT 举报原因
- status: VARCHAR(20) 处理状态 (pending/processed/dismissed)
- created_at: TIMESTAMP 举报时间
- processed_at: TIMESTAMP 处理时间
- processed_by: UUID (外键 -> User, 管理员)
