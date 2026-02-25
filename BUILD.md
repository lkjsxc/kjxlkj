# Build Requirements

**Status:** ✅ **BUILD PASSES** - Both backend and frontend compile successfully.

## Prerequisites

### System Dependencies

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y build-essential pkg-config libssl-dev

# Fedora/RHEL
sudo dnf install -y gcc pkg-config openssl-devel

# macOS
xcode-select --install
```

### Rust Toolchain

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version  # Should be 1.70+
cargo --version
```

### Node.js (for Frontend)

```bash
# Install Node.js 18+
# Using nvm (recommended)
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
nvm install 18
nvm use 18

# Or download from https://nodejs.org/
```

### PostgreSQL (Optional, for database-backed features)

```bash
# Ubuntu/Debian
sudo apt-get install -y postgresql postgresql-contrib

# Install pgvector extension
cd /tmp
git clone --branch v0.7.0 https://github.com/pgvector/pgvector.git
cd pgvector
make
sudo make install
```

### Docker (Optional, for containerized deployment)

```bash
# Install Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER

# Install Docker Compose
sudo apt-get install -y docker-compose-plugin
```

---

## Build Instructions

### Backend (Rust)

```bash
cd /home/lkjsxc/kjxlkj

# Check (faster, verifies syntax and types)
cargo check --workspace

# Build (debug mode)
cargo build --workspace

# Build (release mode, optimized)
cargo build --workspace --release

# Run tests
cargo test --workspace

# Run server
cargo run -p kjxlkj-server
```

### Frontend (TypeScript/React)

```bash
cd /home/lkjsxc/kjxlkj/src/frontend/app

# Install dependencies
npm install

# Development server (with hot reload)
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Lint
npm run lint
```

### Full Stack (Docker Compose)

```bash
cd /home/lkjsxc/kjxlkj

# Build and start all services
docker compose up --build

# Start in detached mode
docker compose up -d

# View logs
docker compose logs -f

# Stop all services
docker compose down
```

---

## Environment Configuration

### 1. Copy Environment Template

```bash
cp .env.example .env
```

### 2. Edit `.env`

```bash
# Database connection (required for persistence)
DATABASE_URL=postgresql://user:password@localhost:5432/kjxlkj

# Optional: Embedding service
EMBEDDING_BASE_URL=http://127.0.0.1:1234/v1
EMBEDDING_MODEL=text-embedding-nomic-embed-text-v1.5
```

### 3. Configure Runtime (`data/config.json`)

```json
{
  "server": {
    "bind_addr": "0.0.0.0:8080",
    "static_dir": "./static"
  },
  "database": {
    "max_connections": 20
  },
  "search": {
    "embedding_provider": "lmstudio",
    "semantic_enabled": true
  },
  "agent": {
    "mode": "yolo",
    "prompt_path": "./data/agent-prompt.json"
  }
}
```

---

## Verification

### Check Backend

```bash
# Verify compilation
cargo check --workspace

# Run unit tests
cargo test --workspace --lib

# Check formatting
cargo fmt -- --check
```

### Check Frontend

```bash
cd src/frontend/app

# Type check
npx tsc --noEmit

# Lint
npm run lint

# Build
npm run build
```

### Health Check (after starting server)

```bash
# Server health
curl http://localhost:8080/api/healthz

# Should return: ok
```

---

## Troubleshooting

### Build Errors

**`linker cc not found`**
```bash
# Install C compiler
sudo apt-get install -y build-essential
```

**`openssl/ssl.h not found`**
```bash
# Install OpenSSL development headers
sudo apt-get install -y libssl-dev
```

**`pkg-config not found`**
```bash
sudo apt-get install -y pkg-config
```

### Runtime Errors

**`database connection refused`**
```bash
# Start PostgreSQL
sudo systemctl start postgresql

# Or use Docker
docker run -d --name postgres \
  -e POSTGRES_PASSWORD=postgres \
  -e POSTGRES_DB=kjxlkj \
  -p 5432:5432 \
  pgvector/pgvector:pg16
```

**`frontend not loading`**
```bash
# Rebuild frontend
cd src/frontend/app
npm run build

# Check static files exist
ls -la ../../static/
```

---

## Expected Build Output

### Successful Backend Build
```
   Compiling kjxlkj-domain v0.1.0
   Compiling kjxlkj-db v0.1.0
   Compiling kjxlkj-auth v0.1.0
   Compiling kjxlkj-rbac v0.1.0
   Compiling kjxlkj-workspace v0.1.0
   Compiling kjxlkj-search v0.1.0
   Compiling kjxlkj-automation v0.1.0
   Compiling kjxlkj-ws v0.1.0
   Compiling kjxlkj-http v0.1.0
   Compiling kjxlkj-server v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 45.2s
```

### Successful Frontend Build
```
vite v5.0.0 building for production...
✓ 94 modules transformed.
../../../static/index.html                   0.54 kB │ gzip:   0.33 kB
../../../static/assets/index-z3IykJSz.css    6.68 kB │ gzip:   1.74 kB
../../../static/assets/index-DcW8pMLg.js   809.82 kB │ gzip: 275.01 kB
✓ built in 2.01s
```

### Server Startup
```
2026-02-25T10:19:13.943324Z  INFO kjxlkj_server: Starting kjxlkj-server...
2026-02-25T10:19:13.943487Z  INFO kjxlkj_server: Configuration loaded
2026-02-25T10:19:13.943518Z  INFO kjxlkj_server: Database pool initialized
2026-02-25T10:19:13.943530Z  INFO kjxlkj_server: Repositories initialized
2026-02-25T10:19:13.943537Z  INFO kjxlkj_server: Session store initialized
2026-02-25T10:19:13.943562Z  INFO kjxlkj_server: WebSocket state initialized
2026-02-25T10:19:13.944019Z  INFO kjxlkj_server: Listening on 0.0.0.0:8080
```

---

## Related Documentation

- [QUICKSTART.md](/docs/guides/QUICKSTART.md) - First-run workflow
- [DOCKER.md](/docs/guides/DOCKER.md) - Docker orchestration
- [API.md](/docs/guides/API.md) - API usage guide
- [testing.md](/docs/spec/technical/testing.md) - Verification tiers
