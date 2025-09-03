# Rust + React (Vite) Single Binary

This project bundles a **React (Vite)** frontend with a **Rust (Axum)** backend into a single binary using [`rust-embed`](https://crates.io/crates/rust-embed).

## ✨ Features

- React + Vite frontend
- 🦀 Rust backend (Axum + Tokio)
- 📦 Frontend assets embedded in the binary
- 🔀 SPA routing fallback (`/dashboard` → `index.html`)
- 🌐 API routes served under `/api/*`
- 🗜 Optional gzip/br compression with `tower-http`

---

## 📂 Project Layout

your-app/
├─ frontend/ # Vite + React app
│ ├─ index.html
│ ├─ src/…
│ └─ package.json
├─ src/
│ └─ main.rs # Axum backend + static handler
├─ build.rs # (optional) auto-build frontend in release mode
└─ Cargo.toml

---

## 🚀 Getting Started

### 1. Install prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (>= 18 recommended)
- [npm](https://www.npmjs.com/)

### 2. Build the frontend

```bash
cd frontend
npm install
npm run build

### 3. Rund the main app
cd ..
cargo run


### 4. Run from a container

cp env_example .env

#### Dockerfile
# Build the image
docker build --build-arg BIN_NAME=rust_embed_web -t my-service .

# Run the container
docker run --rm -p 8080:8080 my-service
```

#### docker compose

# Build and start

docker compose up --build

# Start (after first build)

docker compose up -d

# Stop

docker compose down

### Check it works:

In a browser enter:
http://localhost:8080/api/health
http://localhost:8080/api/hello
