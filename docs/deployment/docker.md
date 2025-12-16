# Docker Deployment

This guide covers running Rustmail in Docker containers.

---

## Quick Start

### Pull the Image

```bash
docker pull ghcr.io/rustmail/rustmail:latest
```

### Run with Docker

```bash
docker run -d \
  --name rustmail \
  -p 3002:3002 \
  -v /path/to/config.toml:/app/config.toml:ro \
  -v rustmail-data:/app/db \
  ghcr.io/rustmail/rustmail:latest
```

---

## Docker Compose

Create a `docker-compose.yml`:

```yaml
version: '3.8'

services:
  rustmail:
    image: ghcr.io/rustmail/rustmail:latest
    container_name: rustmail
    restart: unless-stopped
    ports:
      - "3002:3002"
    volumes:
      - ./config.toml:/app/config.toml:ro
      - rustmail-data:/app/db
    environment:
      - TZ=Europe/Paris

volumes:
  rustmail-data:
```

Start with:

```bash
docker-compose up -d
```

---

## Configuration

### Volume Mounts

| Path               | Description                             |
|--------------------|-----------------------------------------|
| `/app/config.toml` | Configuration file (required)           |
| `/app/db`          | Database directory (persistent storage) |

### Ports

| Port   | Description       |
|--------|-------------------|
| `3002` | Web panel and API |

### Environment Variables

| Variable | Description        |
|----------|--------------------|
| `TZ`     | Container timezone |

---

## Building the Image

To build from source:

```bash
# Clone the repository
git clone https://github.com/Rustmail/rustmail.git
cd rustmail

# Build the image
docker build -t rustmail:local .
```

The Dockerfile uses a multi-stage build with Debian Bookworm Slim as the runtime base.

---

## Docker Compose with Reverse Proxy

### With Traefik

```yaml
version: '3.8'

services:
  rustmail:
    image: ghcr.io/rustmail/rustmail:latest
    container_name: rustmail
    restart: unless-stopped
    volumes:
      - ./config.toml:/app/config.toml:ro
      - rustmail-data:/app/db
    labels:
      - "traefik.enable=true"
      - "traefik.http.routers.rustmail.rule=Host(`panel.example.com`)"
      - "traefik.http.routers.rustmail.tls=true"
      - "traefik.http.routers.rustmail.tls.certresolver=letsencrypt"
      - "traefik.http.services.rustmail.loadbalancer.server.port=3002"
    networks:
      - traefik

networks:
  traefik:
    external: true

volumes:
  rustmail-data:
```

### With Nginx Proxy Manager

```yaml
version: '3.8'

services:
  rustmail:
    image: ghcr.io/rustmail/rustmail:latest
    container_name: rustmail
    restart: unless-stopped
    volumes:
      - ./config.toml:/app/config.toml:ro
      - rustmail-data:/app/db
    networks:
      - npm_network

networks:
  npm_network:
    external: true

volumes:
  rustmail-data:
```

Then in NPM, create a proxy host pointing to `rustmail:3002`.

### With Caddy

```yaml
version: '3.8'

services:
  rustmail:
    image: ghcr.io/rustmail/rustmail:latest
    container_name: rustmail
    restart: unless-stopped
    volumes:
      - ./config.toml:/app/config.toml:ro
      - rustmail-data:/app/db
    networks:
      - caddy

  caddy:
    image: caddy:alpine
    restart: unless-stopped
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./Caddyfile:/etc/caddy/Caddyfile:ro
      - caddy-data:/data
    networks:
      - caddy

networks:
  caddy:

volumes:
  rustmail-data:
  caddy-data:
```

Caddyfile:
```
panel.example.com {
    reverse_proxy rustmail:3002
}
```

---

## Health Checks

Add health monitoring:

```yaml
services:
  rustmail:
    image: ghcr.io/rustmail/rustmail:latest
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3002/api/panel/check"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 10s
```

---

## Logging

View logs:

```bash
# Follow logs
docker logs -f rustmail

# Last 100 lines
docker logs --tail 100 rustmail
```

Configure log rotation in Docker daemon or use a logging driver:

```yaml
services:
  rustmail:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

---

## Backup

### Database Backup

```bash
# Stop container for consistent backup
docker stop rustmail

# Copy database
docker cp rustmail:/app/db/db.sqlite ./backup-$(date +%Y%m%d).sqlite

# Restart
docker start rustmail
```

### Volume Backup

```bash
docker run --rm \
  -v rustmail-data:/data:ro \
  -v $(pwd):/backup \
  alpine tar czf /backup/rustmail-backup.tar.gz /data
```

---

## Updates

### Pull New Image

```bash
docker-compose pull
docker-compose up -d
```

### Manual Update

```bash
docker pull ghcr.io/rustmail/rustmail:latest
docker stop rustmail
docker rm rustmail
# Run new container with same volumes
```

---

## Troubleshooting

### Container Won't Start

Check logs:
```bash
docker logs rustmail
```

Common issues:
- Invalid `config.toml` syntax
- Missing required configuration fields
- Permission issues on mounted volumes

### Cannot Connect to Panel

- Verify port 3002 is exposed: `docker port rustmail`
- Check container is running: `docker ps`
- Verify network connectivity to container

### Database Errors

- Ensure `/app/db` volume is writable
- Check disk space on host
- Verify volume mount is correct

### Permission Denied

The container runs as user `rustmail` (UID 1000). Ensure mounted volumes are accessible:

```bash
# Fix permissions on host
chown -R 1000:1000 /path/to/data
```
