# 社区论坛功能开发路线图

> 创建时间: 2026-04-23
> 状态: 大部分完成

## 总览

本项目参考 Reddit、V2EX、Discourse、掘金等主流社区平台功能，制定分阶段实现计划。

## 任务状态说明

- [ ] 未开始
- [x] 已完成
- [ ] 进行中

---

## P0 - 核心体验（必须实现）

### [x] P0-1: Markdown 富文本编辑器集成 ✅
- [x] 后端: 支持 markdown 内容存储和渲染
- [x] 前端: MarkdownEditor.vue + MarkdownRenderer.vue 组件
- [x] 支持代码高亮
- [ ] 支持图片上传/嵌入 (TODO)
- [x] 支持 @提及 语法高亮
- [x] 用户搜索 API (/users/search)

### [x] P0-2: @用户提及功能开发 ✅
- [x] 后端: 解析内容中的 @username 模式 (utils.rs extract_mentions)
- [x] 后端: 创建 @通知 类型
- [x] 前端: @输入时显示用户下拉建议
- [x] 前端: @提及内容高亮显示
- [x] 通知中心显示 @提及 通知

### [x] P0-3: #话题标签自动链接 ✅
- [x] 后端: 解析内容中的 #话题 模式 (MarkdownRenderer)
- [x] 前端: #话题 高亮并可点击
- [x] 点击跳转至标签详情页

### [x] P0-4: 消息通知系统增强 ✅
- [x] 统一通知中心页面 (Notifications.vue)
- [x] 通知类型: 评论、回复、@提及、点赞、关注、系统
- [x] 通知未读计数角标
- [x] 通知一键已读

### [x] P0-5: 用户个人主页完善 ✅
- [x] 帖子列表标签
- [x] 评论列表
- [x] 收藏列表
- [x] 关注/粉丝列表
- [x] 用户统计 (发帖数、评论数、获赞数)

---

## P1 - 社区氛围（重要）

### [x] P1-1: 签到系统 ✅
- [x] 数据库: user_signups 表
- [x] 每日签到 API
- [x] 连续签到天数统计
- [x] 签到积分奖励
- [x] 前端签到按钮 (SigninCard.vue)

### [x] P1-2: 用户等级/积分系统 ✅
- [x] 数据库: users 表 (points, level)
- [x] 积分规则: 发帖+5, 评论+2, 被点赞+1, 签到+3
- [x] 等级规则: L1(0-100), L2(100-500), L3(500-1000)...
- [x] API: 获取用户等级积分
- [x] 前端显示用户等级徽章 (LevelBadge.vue)

### [x] P1-3: 用户勋章/成就系统 ✅
- [x] 数据库: badges 表, user_badges 表
- [x] 内置勋章: 首个帖子、注册满一年、获得100赞等
- [x] API: 获取用户勋章列表
- [x] 前端个人主页展示勋章

### [x] P1-4: 精华/置顶帖功能 ✅
- [x] 数据库: posts 表添加 is_pinned, is_featured 字段
- [x] 管理后台: 设置精华/置顶
- [x] 列表 API: 优先返回置顶帖
- [x] 前端: 置顶帖特殊样式

### [x] P1-5: 阅读量统计 ✅
- [x] 数据库: posts 表添加 view_count 字段
- [x] 后端: 访问详情页时 increment view_count
- [x] API: 返回阅读量
- [x] 前端: 帖子卡片显示阅读量

### [x] P1-6: 热门话题排行 ✅
- [x] API: /api/v1/posts/hot (综合排序: 点赞*3 + 评论*2 + 阅读*1)
- [x] 前端: 首页增加"热门" Tab

### [x] P1-7: 收藏夹分类功能 ✅
- [x] 数据库: folders 表 (用户收藏夹)
- [x] 数据库: bookmarks 表关联 folder_id
- [x] API: CRUD 收藏夹
- [x] API: 移动书签到收藏夹
- [x] 前端: 收藏管理页面

---

## P2 - 社交互动（增强粘性）

### [ ] P2-1: 私信功能
- [ ] 数据库: conversations, messages 表
- [ ] API: 发送私信、获取会话列表、获取消息历史
- [ ] WebSocket: 实时推送新消息
- [ ] 前端: 私信页面和弹窗

### [ ] P2-2: 评论楼中楼（嵌套回复）
- [ ] 数据库: 评论添加 nest_level, reply_to_comment_id
- [ ] 后端: 嵌套查询优化 (最多3层)
- [ ] 前端: 树形显示评论

### [x] P2-3: 评论点赞功能 ✅
- [x] 后端: comment_votes 表
- [x] API: 点赞/取消点赞评论
- [x] 前端: 评论点赞按钮和计数

### [x] P2-4: 最佳答案/采纳功能 ✅
- [x] 数据库: comments 表 is_accepted 字段
- [x] API: POST /comments/{id}/accept
- [x] 前端: 评论采纳按钮 (仅帖主可见)

### [x] P2-5: 分享功能 ✅
- [x] 前端: 分享按钮组件 (ShareButtons.vue)
- [x] 支持复制链接
- [x] 分享到 Twitter/Facebook 等

---

## P3 - 高级功能（差异化竞争）

### [x] P3-1: 用户黑名单 ✅
- [x] 数据库: user_blocks 表
- [x] API: /blocks CRUD
- [x] 前端: /settings/blocks 页面

### [x] P3-2: 敏感词过滤 ✅
- [x] 后端: sensitive_words.rs 服务
- [x] 后端: 内容发布时过滤 (post_service, comment_service)
- [x] 返回错误提示

### [x] P3-3: 内容举报 ✅
- [x] 数据库: reports 表
- [x] 举报原因选择 (ReportModal.vue)
- [x] 帖子举报功能
- [x] 评论举报功能
- [x] 管理后台处理举报
- [x] 举报成功反馈

### [x] P3-4: 草稿箱 ✅
- [x] 数据库: drafts 表
- [x] API: /drafts CRUD
- [x] 前端: /drafts 草稿列表页面

---

## P4 - 管理运营（可后期加）

### [ ] P4-1: 版主权限系统
- [ ] 数据库: moderators 表
- [ ] 版主可删帖、删评论
- [ ] 版主可禁言用户
- [ ] 版主可设置精华

### [x] P4-2: 用户禁言/封禁 ✅
- [x] 数据库: users 表 is_locked 字段
- [x] 管理后台: 禁言/解封 (AdminUsers.vue)

### [x] P4-3: 全站公告 ✅
- [x] 数据库: announcements 表
- [x] 管理后台: /admin/announcements
- [x] 前端: 顶部公告栏 (AnnouncementBanner.vue)

---

## 未完成项 (TODO)

### P0
- [ ] 图片上传/嵌入

### P2
- [ ] P2-1 私信功能
- [ ] P2-2 评论楼中楼

### P3
- [ ] 屏蔽被拉黑用户的内容

### P4
- [ ] P4-1 版主权限系统
- [ ] 被禁言用户操作限制
