# 🚀 Dynamic Portfolio Server

**Hexagonal Architecture (Ports & Adapters) | Rust | Axum | PostgreSQL | SQLx**

---

## 📌 Overview

Dynamic Portfolio Server is a production-ready backend built with **Rust** using **Hexagonal Architecture (Ports & Adapters)**.

It powers a dynamic portfolio system that supports:

* 🔐 Authentication (JWT + Refresh Tokens)
* 👤 User Management
* 🧠 Tech Stack Management
* 📝 Blog Management
* 💼 Project Management
* 🖼 Image Upload (Cloudinary – FormData & Base64)
* 🔄 Secure Token Refresh & Logout
* 🧩 Clean Separation of Concerns (Domain-Driven Structure)

This project follows clean architectural boundaries, ensuring scalability, testability, and maintainability.

---

# 🏗 Architecture

This project implements **Hexagonal Architecture (Ports & Adapters)**.

```
                ┌─────────────────────┐
                │      API Layer      │
                │   (Axum Routers)    │
                └─────────┬───────────┘
                          │
                ┌─────────▼───────────┐
                │   Application Layer │
                │   (Use Cases)       │
                └─────────┬───────────┘
                          │
                ┌─────────▼───────────┐
                │      Domain Core    │
                │   (Business Logic)  │
                └─────────┬───────────┘
                          │
                ┌─────────▼───────────┐
                │     Ports (Traits)  │
                └─────────┬───────────┘
                          │
                ┌─────────▼───────────┐
                │     Adapters        │
                │ PostgreSQL, JWT,    │
                │ Cloudinary, etc.    │
                └─────────────────────┘
```

---

## 📂 Project Structure

```
adapter/            → Infrastructure implementations (DB, JWT, Cloudinary)
api/                → HTTP routing layer
application/        → Application services (use cases)
core/               → Core shared logic
domain/             → Business entities & rules
port/               → Traits (Interfaces)
router_handler/     → Request handlers
state/              → AppState container
config/             → Environment configuration
error/              → Custom API errors
utils/              → Utilities
```

---

# 🛠 Tech Stack

| Technology    | Purpose               |
| ------------- | --------------------- |
| 🦀 Rust       | Core backend language |
| ⚡ Axum        | Web framework         |
| 🐘 PostgreSQL | Database              |
| 🔄 SQLx       | Async database driver |
| 🔐 JWT        | Authentication        |
| 🍪 Cookies    | Refresh token storage |
| ☁️ Cloudinary | Image hosting         |
| 🧵 Tokio      | Async runtime         |

---

# 🌐 Base URL

```
http://localhost:{PORT}/api/v1
```

---

# 🔐 Authentication APIs

### `/api/v1/auth`

| Method | Endpoint                    | Description              |
| ------ | --------------------------- | ------------------------ |
| POST   | `/register`                 | Register new user        |
| POST   | `/login`                    | Login user               |
| GET    | `/get_all_users`            | Get all users            |
| GET    | `/get_single_users/{email}` | Get single user by email |
| PATCH  | `/single_users/{id}`        | Update user              |
| DELETE | `/single_users/{id}`        | Delete user              |

---

# 🧠 Stack APIs

### `/api/v1/stack`

| Method | Endpoint             | Description       |
| ------ | -------------------- | ----------------- |
| POST   | `/create`            | Create tech stack |
| GET    | `/get_all_stack`     | Get all stacks    |
| GET    | `/single_stack/{id}` | Get single stack  |
| PATCH  | `/single_stack/{id}` | Update stack      |
| DELETE | `/single_stack/{id}` | Delete stack      |

---

# 📝 Blog APIs

### `/api/v1/blog`

| Method | Endpoint       | Description     |
| ------ | -------------- | --------------- |
| POST   | `/create`      | Create blog     |
| GET    | `/all`         | Get all blogs   |
| GET    | `/detail/{id}` | Get single blog |
| PATCH  | `/detail/{id}` | Update blog     |
| DELETE | `/detail/{id}` | Delete blog     |

---

# 💼 Project APIs

### `/api/v1/project`

| Method | Endpoint       | Description        |
| ------ | -------------- | ------------------ |
| POST   | `/create`      | Create project     |
| GET    | `/all`         | Get all projects   |
| GET    | `/detail/{id}` | Get single project |
| PATCH  | `/detail/{id}` | Update project     |
| DELETE | `/detail/{id}` | Delete project     |

---

# 🖼 Image Upload APIs

### `/api/v1/upload`

| Method | Endpoint     | Description                 |
| ------ | ------------ | --------------------------- |
| POST   | `/form_data` | Upload image using FormData |
| POST   | `/base64`    | Upload image using Base64   |

---

# 🔄 Refresh Token APIs

### `/api/v1/refresh`

| Method | Endpoint   | Description                            |
| ------ | ---------- | -------------------------------------- |
| POST   | `/refresh` | Refresh access token                   |
| POST   | `/logout`  | Logout user & invalidate refresh token |

Uses secure HTTP-only cookies for refresh tokens.

---

# ⚙️ Environment Variables

Create a `.env` file:

```env
DATABASE_URL=postgres://user:password@localhost:5432/db_name
PORT=8000

JWT_SECRET=your_super_secret_key
JWT_EXPIRY_SECONDS=3600

CLOUD_NAME=your_cloud_name
CLOUD_API_KEY=your_cloud_api_key
CLOUD_API_SECRET=your_cloud_api_secret

DB_POOL_MAX_CONNECTIONS=12
```

---

# 🚀 Running the Project

### 1️⃣ Clone the Repository

```bash
git clone https://github.com/your-username/dynamic-portfolio-server.git
cd dynamic-portfolio-server
```

### 2️⃣ Run Database Migrations

```bash
sqlx database create
sqlx migrate run
```

### 3️⃣ Start Server

```bash
cargo run
```

Server will start at:

```
🚀 http://0.0.0.0:{PORT}
```

---

# 🧪 Error Handling

Centralized error handling via:

* Custom `ApiErrors`
* 404 fallback handler
* Method Not Allowed interceptor
* Consistent JSON error responses

---

# 🔥 Design Decisions (Senior-Level Notes)

### ✅ 1. Dependency Injection via `Arc<dyn Trait>`

All infrastructure services are injected as trait objects:

```rust
Arc<dyn UserDBServices + Send + Sync>
```

This allows:

* Easy mocking
* Clear boundaries
* Testable use cases
* Swappable database implementations

---

### ✅ 2. Clean Separation of Concerns

| Layer          | Responsibility         |
| -------------- | ---------------------- |
| Router         | HTTP wiring            |
| Router Handler | Request parsing        |
| Application    | Business orchestration |
| Domain         | Core rules             |
| Port           | Interfaces             |
| Adapter        | External systems       |

---

### ✅ 3. Stateless & Scalable

* No in-memory session storage
* JWT-based auth
* DB-backed refresh tokens
* Horizontally scalable

---

### ✅ 4. Production Ready Patterns

* Structured error handling
* Layered middleware
* Graceful async server
* Connection pooling
* Secure cookie handling

---

# 📈 Future Improvements

* Role-based access control (RBAC)
* API rate limiting
* OpenAPI / Swagger docs
* Integration tests
* CI/CD pipeline
* Docker containerization
* Redis caching
* Observability (Tracing + Metrics)

---

# 👨‍💻 Author

**Joshua**

Backend Engineer
Rust | System Design | Clean Architecture Advocate

---

# 📜 License

MIT License

---
