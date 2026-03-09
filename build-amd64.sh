#!/bin/bash
set -e

echo "=== Building AFFiNE for linux/amd64 ==="
echo "This will take 15-30 minutes as it compiles Rust and builds everything inside a Linux container..."
echo ""

docker buildx build \
  --platform linux/amd64 \
  --progress=plain \
  -f Dockerfile.amd64 \
  -t affine-custom:amd64 \
  --load \
  .

echo ""
echo "=== Build Complete! ==="
echo "Image: affine-custom:amd64"
echo ""
echo "To save and transfer to VPS:"
echo "  docker save affine-custom:amd64 | gzip > ~/affine-custom-amd64.tar.gz"
echo "  scp ~/affine-custom-amd64.tar.gz USER@VPS_IP:~/"

