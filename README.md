# Community Forum

A modern community forum system built with Rust + Axum backend and Vue 3 + TypeScript frontend.

## Tech Stack

### Backend
- **Language**: Rust
- **Framework**: Axum 0.8
- **Database**: SQLite with SQLx
- **Authentication**: JWT
- **WebSocket**: for real-time notifications

### Frontend
- **Framework**: Vue 3
- **Language**: TypeScript
- **Build Tool**: Vite
- **Styling**: TailwindCSS 3
- **State Management**: Pinia
- **HTTP Client**: Axios

## Features

- User authentication (register/login with JWT)
- Post CRUD with markdown support
- Tag system for post categorization
- Comment and reply system
- Like and dislike posts/comments
- Bookmark favorite posts
- Follow other users
- Real-time notifications via WebSocket
- User profile management
- Admin dashboard API

## Project Structure

```
.
├── backend/              # Rust Axum backend
│   ├── src/
│   │   ├── models/      # Data models
│   │   ├── repositories/# Database operations
│   │   ├── routes/      # API endpoints
│   │   ├── services/    # Business logic
│   │   └── ws/          # WebSocket handlers
│   ├── migrations/      # Database migrations
│   └── forum.db         # SQLite database
├── frontend/             # Vue 3 frontend
│   ├── src/
│   │   ├── components/  # Vue components
│   │   ├── composables/  # Vue composables
│   │   ├── views/       # Page views
│   │   ├── stores/      # Pinia stores
│   │   └── router/      # Vue Router
│   └── dist/            # Built files
└── start.sh             # Startup script
```

## Getting Started

### Prerequisites

- Rust 1.70+
- Node.js 18+
- npm or yarn

### Backend Setup

```bash
cd backend
cargo run
```

The backend server will start on `http://localhost:3000`.

### Frontend Setup

```bash
cd frontend
npm install
npm run dev
```

The frontend dev server will start on `http://localhost:5173`.

### Using the Startup Script

```bash
./start.sh
```

This will start both backend and frontend servers.

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login
- `POST /api/v1/auth/refresh` - Refresh token

### Users
- `GET /api/v1/users/:username` - Get user profile
- `PATCH /api/v1/users/profile` - Update profile
- `GET /api/v1/users/:username/followers` - Get followers
- `GET /api/v1/users/:username/following` - Get following

### Posts
- `GET /api/v1/posts` - List posts
- `POST /api/v1/posts` - Create post
- `GET /api/v1/posts/:id` - Get post
- `PATCH /api/v1/posts/:id` - Update post
- `DELETE /api/v1/posts/:id` - Delete post

### Comments
- `GET /api/v1/comments/posts/:post_id/comments` - List comments
- `POST /api/v1/comments/posts/:post_id/comments` - Create comment
- `PATCH /api/v1/comments/:id` - Update comment
- `DELETE /api/v1/comments/:id` - Delete comment

### Tags
- `GET /api/v1/tags` - List all tags
- `GET /api/v1/tags/:name` - Get tag with posts

### Votes
- `POST /api/v1/votes` - Vote on post/comment
- `DELETE /api/v1/votes/:post_id` - Remove vote

### Bookmarks
- `GET /api/v1/bookmarks` - List bookmarked posts
- `POST /api/v1/bookmarks` - Bookmark a post
- `DELETE /api/v1/bookmarks/:post_id` - Remove bookmark

### Follow
- `POST /api/v1/follow/:user_id` - Follow user
- `DELETE /api/v1/follow/:user_id` - Unfollow user

### Notifications
- `GET /api/v1/notifications` - List notifications
- `POST /api/v1/notifications/read` - Mark as read
- `POST /api/v1/notifications/read-all` - Mark all as read

### WebSocket
- `WS /api/v1/ws` - WebSocket connection for real-time notifications

### Admin
- `GET /api/v1/admin/stats` - Get forum statistics
- `GET /api/v1/admin/reports` - List reports
- `POST /api/v1/admin/reports/:id/process` - Process report

## Environment Variables

### Backend (.env)
```
DATABASE_URL=sqlite://forum.db
JWT_SECRET=your-secret-key
JWT_ACCESS_EXPIRES=3600
JWT_REFRESH_EXPIRES=604800
HOST=0.0.0.0
PORT=3000
```

## License

MIT
