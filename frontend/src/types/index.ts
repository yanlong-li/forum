export interface User {
  id: string
  username: string
  email?: string
  avatar_url: string | null
  bio: string | null
  level?: number
  points?: number
  is_admin?: boolean
  created_at: string
}

export interface UserProfile extends User {
  post_count: number
  follower_count: number
  following_count: number
  is_following: boolean
}

export interface Post {
  id: string
  author_id: string
  author: User
  title: string
  content: string
  tags: string[]
  like_count: number
  dislike_count: number
  comment_count: number
  is_bookmarked: boolean
  is_liked: boolean
  is_pinned: boolean
  is_featured: boolean
  view_count: number
  created_at: string
  updated_at: string
}

export interface PostSummary {
  id: string
  author_id: string
  author: User
  title: string
  content: string
  tags: string[]
  like_count: number
  comment_count: number
  is_bookmarked: boolean
  is_pinned: boolean
  is_featured: boolean
  view_count: number
  created_at: string
}

export interface Comment {
  id: string
  post_id: string
  author_id: string
  author: User
  parent_id: string | null
  content: string
  is_deleted: boolean
  like_count: number
  is_liked: boolean
  reply_count: number
  created_at: string
  updated_at: string
}

export interface Tag {
  id: string
  name: string
  post_count: number
  created_at: string
}

export interface Notification {
  id: string
  type: string
  actor: User
  post_id: string | null
  comment_id: string | null
  is_read: boolean
  created_at: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  limit: number
}

export interface ApiError {
  error: {
    code: string
    message: string
    details?: Array<{
      field: string
      message: string
    }>
  }
}
