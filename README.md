# OxFeed

Feed reader.

![Screenshot](screenshot.png)

## Compilation

```
make
```

## Installation

Create a new PostgreSQL database:

```
createdb oxfeed
psql -f api/sql/structure.sql oxfeed
```

Here an example of systemd service:

```
[Unit]
Description=oxfeed

[Service]
ExecStart=/home/git/public_repositories/oxfeed/current/target/release/oxfeed-api
WorkingDirectory=/home/git/public_repositories/oxfeed/current
Restart=on-failure
Environment="LISTEN_IP=127.0.0.1"
Environment="LISTEN_PORT=8003"
Environment="DATABASE_URL=postgresql://localhost/oxfeed"
Environment="SECRET=change_me"
Environment="RUST_LOG=warn"
Environment="CACHE_DIR=/var/cache/oxfeed"
```
