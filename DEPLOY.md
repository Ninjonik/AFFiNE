# Deploy Custom AFFiNE to Your VPS

## Summary of Changes Made

✅ **Added TOS Links** to:
- Sidebar
- Sign-in dialog  
- About/settings page

✅ **Unlocked Team Features**:
- Analytics window options (7, 14, 28, 60, 90 days) now available for all workspaces

✅ **Built for linux/amd64** (your VPS architecture)

---

## Step 1: Build the Docker Image (On Your Mac)

**This will take 15-30 minutes** as it compiles Rust and builds everything inside a Linux container.

```bash
cd /Users/peterzatko/WebstormProjects/AFFiNE
./build-amd64.sh
```

The script will:
1. Install Rust inside a Linux amd64 container
2. Compile all native modules for x86_64
3. Build web, admin, and mobile frontends
4. Build the backend server
5. Create final runtime image

**Watch the output** - you'll see:
- Rust compilation progress
- Yarn build logs
- Docker layer creation

---

## Step 2: Save and Transfer (On Your Mac)

```bash
# Save the image (will be ~500MB-1GB compressed)
docker save affine-custom:amd64 | gzip > ~/affine-custom-amd64.tar.gz

# Check size
ls -lh ~/affine-custom-amd64.tar.gz

# Transfer to VPS (replace USER and VPS_IP)
scp ~/affine-custom-amd64.tar.gz USER@VPS_IP:~/
```

---

## Step 3: Load on VPS (SSH into your VPS)

```bash
# Load the image
docker load < ~/affine-custom-amd64.tar.gz

# Verify
docker images affine-custom

# Clean up tar file
rm ~/affine-custom-amd64.tar.gz
```

---

## Step 4: Update docker-compose.yml (On your VPS)

Edit `/docker/affine/docker-compose.yml` and replace **both** image lines:

**Before:**
```yaml
image: ghcr.io/toeverything/affine:${AFFINE_REVISION:-stable}
```

**After:**
```yaml
image: affine-custom:amd64
```

**Full updated docker-compose.yml:**

```yaml
name: affine
services:
  affine:
    image: affine-custom:amd64        # ← CHANGED
    container_name: affine_server
    ports:
      - '${PORT:-3010}:3010'
    depends_on:
      redis:
        condition: service_healthy
      postgres:
        condition: service_healthy
      affine_migration:
        condition: service_completed_successfully
    volumes:
      - ${UPLOAD_LOCATION}:/root/.affine/storage
      - ${CONFIG_LOCATION}:/root/.affine/config
    env_file:
      - .env
    environment:
      - REDIS_SERVER_HOST=redis
      - DATABASE_URL=postgresql://${DB_USERNAME}:${DB_PASSWORD}@postgres:5432/${DB_DATABASE:-affine}
      - AFFINE_INDEXER_ENABLED=false
    restart: unless-stopped

  affine_migration:
    image: affine-custom:amd64        # ← CHANGED
    container_name: affine_migration_job
    volumes:
      - ${UPLOAD_LOCATION}:/root/.affine/storage
      - ${CONFIG_LOCATION}:/root/.affine/config
    command: ['sh', '-c', 'node ./scripts/self-host-predeploy.js']
    env_file:
      - .env
    environment:
      - REDIS_SERVER_HOST=redis
      - DATABASE_URL=postgresql://${DB_USERNAME}:${DB_PASSWORD}@postgres:5432/${DB_DATABASE:-affine}
      - AFFINE_INDEXER_ENABLED=false
    depends_on:
      postgres:
        condition: service_healthy
      redis:
        condition: service_healthy

  redis:
    image: redis
    container_name: affine_redis
    healthcheck:
      test: ['CMD', 'redis-cli', '--raw', 'incr', 'ping']
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped

  postgres:
    image: pgvector/pgvector:pg16
    container_name: affine_postgres
    volumes:
      - ${DB_DATA_LOCATION}:/var/lib/postgresql/data
    environment:
      POSTGRES_USER: ${DB_USERNAME}
      POSTGRES_PASSWORD: ${DB_PASSWORD}
      POSTGRES_DB: ${DB_DATABASE:-affine}
      POSTGRES_INITDB_ARGS: '--data-checksums'
      POSTGRES_HOST_AUTH_METHOD: trust
    healthcheck:
      test: ['CMD', 'pg_isready', '-U', "${DB_USERNAME}", '-d', "${DB_DATABASE:-affine}"]
      interval: 10s
      timeout: 5s
      retries: 5
    restart: unless-stopped
```

---

## Step 5: Deploy (On your VPS)

```bash
cd /docker/affine

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

## Verify Your Changes

1. **TOS Links**: 
   - Check sidebar for TOS1 and TOS2 buttons
   - Check sign-in dialog for both links
   - Check Settings → About for both links

2. **Unlocked Features**:
   - Open any document analytics
   - All time windows (7, 14, 28, 60, 90 days) should be available

---

## Future Updates

When you make code changes:

```bash
# On Mac: Rebuild
cd /Users/peterzatko/WebstormProjects/AFFiNE
./build-amd64.sh

# Save and transfer
docker save affine-custom:amd64 | gzip > ~/affine-custom-amd64.tar.gz
scp ~/affine-custom-amd64.tar.gz USER@VPS_IP:~/

# On VPS: Reload and restart
docker load < ~/affine-custom-amd64.tar.gz
cd /docker/affine
docker compose down
docker compose up -d
```

---

## Troubleshooting

**Image won't load on VPS:**
- Ensure VPS is x86_64: `uname -m` (should show `x86_64`)
- Check Docker version: `docker --version`

**Build fails on Mac:**
- Ensure Docker Desktop is running
- Check available disk space (need ~20GB free)
- Try: `docker system prune -a` then rebuild

**Container won't start:**
- Check logs: `docker compose logs affine`
- Verify .env file has correct DB credentials
- Ensure postgres and redis are healthy: `docker compose ps`

