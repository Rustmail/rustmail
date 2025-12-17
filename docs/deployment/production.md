# Production Deployment

Best practices and recommendations for running Rustmail in production.

---

## Checklist

Before deploying to production:

- [ ] Tested configuration locally
- [ ] Verified bot permissions in Discord
- [ ] Set up HTTPS for the panel
- [ ] Configured backups
- [ ] Set up monitoring
- [ ] Documented your configuration

---

## System Requirements

### Minimum

- 1 CPU core
- 512 MB RAM
- 1 GB disk space

### Recommended

- 2 CPU cores
- 1 GB RAM
- 5 GB disk space (for database growth)

Rustmail is lightweight. Resource usage grows with ticket volume and message history.

---

## Security

### HTTPS

Always use HTTPS for the panel in production. Options:

1. **Reverse proxy with TLS termination** (recommended)
   - Nginx, Caddy, Traefik
   - Automatic certificate renewal with Let's Encrypt

2. **Cloud load balancer**
   - AWS ALB, GCP Load Balancer, Cloudflare

See [Configuration](../getting-started/configuration.md#reverse-proxy-setup) for proxy setup.

### Firewall

Restrict access to necessary ports only:

```bash
# Allow SSH
ufw allow 22/tcp

# Allow HTTPS (reverse proxy)
ufw allow 443/tcp

# Block direct access to bot port (if using reverse proxy)
# ufw deny 3002/tcp

ufw enable
```

### Secrets Management

Protect sensitive configuration:

- Bot token
- OAuth2 client secret
- Database file

Options:
- File permissions: `chmod 600 config.toml`
- Docker secrets
- Environment-based secret injection at deployment

### Updates

Keep Rustmail updated for security fixes:

```bash
# Check current version
./rustmail --version

# Update (Docker)
docker pull ghcr.io/rustmail/rustmail:latest
```

---

## High Availability

Rustmail is designed as a single-instance application. For high availability:

### Database

- Regular backups
- Store backups off-server
- Test restore procedures

### Process Management

Use a process manager to ensure the bot restarts on failure:

**Systemd (Linux):**
```ini
[Unit]
Description=Rustmail Discord Bot
After=network.target

[Service]
Type=simple
User=rustmail
WorkingDirectory=/opt/rustmail
ExecStart=/opt/rustmail/rustmail
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

**Docker:**
```yaml
services:
  rustmail:
    restart: unless-stopped
```

### Monitoring

Monitor bot status:

```bash
# Simple health check
curl -f http://localhost:3002/api/bot/status
```

Integration with monitoring systems:
- Prometheus metrics endpoint (future feature)
- External uptime monitoring
- Discord webhook alerts

---

## Backup Strategy

### What to Backup

| Item          | Location      | Frequency |
|---------------|---------------|-----------|
| Database      | `db.sqlite`   | Daily     |
| Configuration | `config.toml` | On change |

### Automated Backups

Example backup script:

```bash
#!/bin/bash
BACKUP_DIR="/backups/rustmail"
DATE=$(date +%Y%m%d_%H%M%S)

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup database
cp /opt/rustmail/db.sqlite "$BACKUP_DIR/db_$DATE.sqlite"

# Backup config
cp /opt/rustmail/config.toml "$BACKUP_DIR/config_$DATE.toml"

# Keep last 30 days
find "$BACKUP_DIR" -type f -mtime +30 -delete
```

Add to crontab:
```
0 3 * * * /opt/rustmail/backup.sh
```

### Off-site Backup

Sync backups to remote storage:

```bash
# rsync to remote server
rsync -avz /backups/rustmail/ backup-server:/backups/rustmail/

# AWS S3
aws s3 sync /backups/rustmail/ s3://your-bucket/rustmail-backups/
```

---

## Logging

### Log Levels

Rustmail outputs logs to stdout. In production:

```bash
# Redirect to file
./rustmail >> /var/log/rustmail/rustmail.log 2>&1
```

### Log Rotation

With logrotate (`/etc/logrotate.d/rustmail`):

```
/var/log/rustmail/*.log {
    daily
    rotate 14
    compress
    delaycompress
    missingok
    notifempty
    create 0640 rustmail rustmail
}
```

### Centralized Logging

For Docker deployments, use logging drivers:

```yaml
services:
  rustmail:
    logging:
      driver: "syslog"
      options:
        syslog-address: "udp://logserver:514"
        tag: "rustmail"
```

---

## Performance Tuning

### SQLite Optimization

The database uses SQLite with default settings. For high-volume deployments:

```sql
-- Run these optimizations
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA cache_size = -64000;  -- 64MB cache
```

### Connection Pooling

Rustmail uses SQLx with connection pooling. Default settings are suitable for most deployments.

---

## Maintenance

### Planned Downtime

For updates requiring restart:

1. Close active tickets or notify staff
2. Stop the bot
3. Backup database
4. Apply update
5. Verify configuration
6. Start the bot
7. Verify functionality

### Database Maintenance

Periodically optimize the database:

```bash
# Stop bot first
sqlite3 db.sqlite "VACUUM;"
sqlite3 db.sqlite "ANALYZE;"
```

### Cleanup Old Data

If needed for storage or compliance:

```sql
-- Delete closed tickets older than 2 years
DELETE FROM thread_messages
WHERE thread_id IN (
  SELECT id FROM threads
  WHERE status = 0
  AND closed_at < datetime('now', '-2 years')
);

DELETE FROM threads
WHERE status = 0
AND closed_at < datetime('now', '-2 years');

VACUUM;
```

---

## Troubleshooting

### Bot Goes Offline

1. Check process status: `systemctl status rustmail`
2. Check logs for errors
3. Verify Discord API status
4. Check network connectivity
5. Verify token validity

### Panel Inaccessible

1. Check bot process is running
2. Verify port 3002 is listening: `netstat -tlnp | grep 3002`
3. Check reverse proxy configuration
4. Verify SSL certificate validity
5. Check firewall rules

### Performance Issues

1. Monitor CPU/memory usage
2. Check database size
3. Review ticket volume
4. Consider database maintenance

### Data Recovery

From backup:

```bash
# Stop bot
systemctl stop rustmail

# Restore database
cp /backups/rustmail/db_20240115.sqlite /opt/rustmail/db.sqlite

# Verify
sqlite3 /opt/rustmail/db.sqlite "SELECT COUNT(*) FROM threads;"

# Start bot
systemctl start rustmail
```
