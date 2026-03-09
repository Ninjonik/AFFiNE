# Build AFFiNE on Debian 13 VPS

## Step 1: Install Dependencies

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Node.js 22 (via NodeSource)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs

# Install Yarn (Corepack method)
sudo corepack enable
sudo corepack prepare yarn@stable --activate

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default 1.93.1

# Install build tools
sudo apt install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    git \
    docker.io \
    docker-compose

# Add your user to docker group (logout/login after this)
sudo usermod -aG docker $USER

# Verify installations
node --version    # Should be v22.x
yarn --version    # Should be 4.x
cargo --version   # Should be 1.93.1
docker --version
```

---

## Step 2: Build Native Modules

```bash
# Navigate to your cloned repo
cd /path/to/AFFiNE

# Pull latest changes (with your TOS links & unlocked features)
git pull origin canary

# Install dependencies
yarn install

# Build Rust native modules for x86_64
cargo build --release --target x86_64-unknown-linux-gnu

# Copy native modules to the right place
cp target/x86_64-unknown-linux-gnu/release/libaffine_server_native.so \
   packages/backend/native/server-native.x64.node

cp packages/backend/native/server-native.x64.node \
   packages/backend/native/server-native.arm64.node

cp packages/backend/native/server-native.x64.node \
   packages/backend/native/server-native.armv7.node
```

---

## Step 3: Build Frontend & Backend

```bash
# Build web frontend (required)
yarn affine build -p web --deps --wait-deps

# Build admin frontend (optional - for admin panel)
yarn affine build -p admin --deps --wait-deps

# Skip mobile build (not needed for self-hosted) - create dummy dist
mkdir -p packages/frontend/apps/mobile/dist
touch packages/frontend/apps/mobile/dist/.gitkeep

# Build backend server (required)
yarn affine build -p server --deps --wait-deps
```

**Note:** Mobile frontend has build issues and isn't needed for self-hosted web access. We create an empty dist folder to satisfy Docker.

---

## Step 4: Build Docker Image

```bash
# Build the image using the official Dockerfile
docker build -f .github/deployment/node/Dockerfile -t affine-custom:latest .

# Verify image was created
docker images affine-custom
```

---

## Step 5: Update docker-compose.yml

```bash
cd /docker/affine

# Edit docker-compose.yml
nano docker-compose.yml
```

Change both image lines from:
```yaml
image: ghcr.io/toeverything/affine:${AFFINE_REVISION:-stable}
```

To:
```yaml
image: affine-custom:latest
```

---

## Step 6: Deploy

```bash
# Stop existing containers
docker compose down

# Start with custom image
docker compose up -d

# Watch logs
docker compose logs -f affine

# Check status
docker compose ps
```

---

## Estimated Build Times (Native x86_64)

- **Rust compilation**: 5-10 minutes
- **Web/Admin/Mobile**: 5-7 minutes
- **Backend server**: 2-3 minutes
- **Docker image**: 2-3 minutes
- **Total**: ~15-25 minutes

Much faster than the 30+ minutes on Mac with emulation! 🚀

---

## Troubleshooting

**If Rust build fails:**
```bash
# Ensure you sourced cargo
source $HOME/.cargo/env

# Check Rust version
rustc --version  # Should be 1.93.1
```

**If yarn install fails:**
```bash
# Clear cache
yarn cache clean

# Try again
yarn install
```

**If Docker build fails:**
```bash
# Ensure all dist folders exist
ls -la packages/frontend/apps/web/dist
ls -la packages/frontend/admin/dist
ls -la packages/frontend/apps/mobile/dist
ls -la packages/backend/server/dist

# If any are missing, rebuild them with yarn affine build
```

**Low disk space:**
```bash
# Check available space (need ~20GB)
df -h

# Clean Docker if needed
docker system prune -a
```

---

## One-Liner Quick Build (After Dependencies Installed)

```bash
cd /path/to/AFFiNE && \
git pull && \
yarn install && \
cargo build --release --target x86_64-unknown-linux-gnu && \
cp target/x86_64-unknown-linux-gnu/release/libaffine_server_native.so packages/backend/native/server-native.x64.node && \
cp packages/backend/native/server-native.x64.node packages/backend/native/server-native.arm64.node && \
cp packages/backend/native/server-native.x64.node packages/backend/native/server-native.armv7.node && \
yarn affine build -p web --deps --wait-deps && \
yarn affine build -p admin --deps --wait-deps && \
mkdir -p packages/frontend/apps/mobile/dist && touch packages/frontend/apps/mobile/dist/.gitkeep && \
yarn affine build -p server --deps --wait-deps && \
docker build -f .github/deployment/node/Dockerfile -t affine-custom:latest . && \
echo "✅ Build complete! Update docker-compose.yml and run: docker compose up -d"
```

**What this does:**
- Skips mobile build (has blocksuite dependency issues and not needed for web)
- Creates empty mobile/dist folder to satisfy Docker COPY command
- Builds only web, admin, and server (the essentials)

