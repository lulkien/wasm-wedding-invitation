# SurrealDB — Native Deployment Guide

This document covers how to install and run SurrealDB **natively** on Linux
(no Docker required) and how to define the schema used by this app.

---

## 1. Install the Binary

The official install script detects your architecture and drops a single
self-contained binary into `~/.cargo/bin/` (or `/usr/local/bin/`).

```sh
curl --proto '=https' --tlsv1.2 -sSf https://install.surrealdb.com | sh
```

Verify the installation:

```sh
surreal version
```

---

## 2. Storage Backends

SurrealDB supports several storage engines. Choose one based on your needs:

| Backend | Flag | Use case |
|---|---|---|
| In-memory | `memory` | Development / testing — data lost on restart |
| SurrealKV | `surrealkv:///path/to/data` | **Recommended for production** — native Rust, single file |
| RocksDB | `rocksdb:///path/to/data` | Production alternative — mature C++ engine |

---

## 3. Running the Server

### Development (in-memory)

```sh
surreal start \
  --user root \
  --password secret \
  --bind 127.0.0.1:8000 \
  memory
```

### Production (SurrealKV on disk)

```sh
surreal start \
  --user root \
  --password secret \
  --bind 127.0.0.1:8000 \
  surrealkv:///srv/wedding/data.db
```

> **Tip:** Bind to `127.0.0.1` (loopback) so the port is never exposed to the
> internet. Your Dioxus server runs on the same machine and connects locally.

---

## 4. Run as a systemd Service

Create the service file:

```sh
sudo vim /etc/systemd/system/surrealdb.service
```

Paste:

```ini
[Unit]
Description=SurrealDB
After=network.target

[Service]
Type=simple
User=www-data
Group=www-data

ExecStart=/usr/local/bin/surreal start \
    --user root \
    --password secret \
    --bind 127.0.0.1:8000 \
    surrealkv:///srv/wedding/data.db

Restart=on-failure
RestartSec=5

# Harden the process
NoNewPrivileges=true
ProtectSystem=strict
ReadWritePaths=/srv/wedding

[Install]
WantedBy=multi-user.target
```

Enable and start:

```sh
sudo systemctl daemon-reload
sudo systemctl enable surrealdb
sudo systemctl start surrealdb
sudo systemctl status surrealdb
```

---

## 5. Schema

Connect to the running instance:

```sh
surreal sql \
  --conn ws://127.0.0.1:8000 \
  --user root \
  --password secret \
  --default-namespace wedding \
  --default-database wedding
```

Run the schema DDL:

```sql
-- Namespace and database are created automatically on first USE.
-- Run these once after starting a fresh instance.

-- Departure location lookup table
DEFINE TABLE location_map SCHEMAFULL;
DEFINE FIELD location_id   ON location_map TYPE int;
DEFINE FIELD location_name ON location_map TYPE string;

-- Seed the lookup rows
CREATE location_map:1 SET location_id = 1, location_name = 'FPT Tower';
CREATE location_map:2 SET location_id = 2, location_name = 'Handico Tower';
CREATE location_map:3 SET location_id = 3, location_name = 'Lotte Mall West Lake';
CREATE location_map:4 SET location_id = 4, location_name = 'I use my own vehicle';
CREATE location_map:5 SET location_id = 5, location_name = 'I\'ll pass';

-- Guest table
DEFINE TABLE people SCHEMAFULL;
DEFINE FIELD uid          ON people TYPE string ASSERT $value != NONE;
DEFINE FIELD name         ON people TYPE string;
DEFINE FIELD greeting     ON people TYPE string;
DEFINE FIELD line1        ON people TYPE string;
DEFINE FIELD line2        ON people TYPE option<string>;
DEFINE FIELD line3        ON people TYPE option<string>;
DEFINE FIELD desc         ON people TYPE option<string>;
DEFINE FIELD depart_from  ON people TYPE option<record<location_map>>;
```

### `depart_from` record links

`depart_from` is a SurrealDB record link to `location_map`, not a plain integer.
`NONE` means the guest has not yet chosen a departure point.

| Record | `location_id` | Meaning |
|---|---|---|
| `NONE` | — | Not decided yet (default) |
| `location_map:1` | `1` | FPT Tower shuttle |
| `location_map:2` | `2` | Handico Tower shuttle |
| `location_map:3` | `3` | Lotte Mall West Lake shuttle |
| `location_map:4` | `4` | Own vehicle |
| `location_map:5` | `5` | Can't attend |

---

## 6. Adding Guests

Each guest needs a unique 8-character hex UID. Generate one:

```sh
openssl rand -hex 4
# e.g. a1b2c3d4
```

Insert the guest (record key = uid):

```sql
CREATE people:a1b2c3d4 CONTENT {
    uid:         "a1b2c3d4",
    name:        "Nguyen Van A",
    greeting:    "Dear",
    line1:       "We joyfully invite you to share in the celebration of our wedding.",
    line2:       "Your presence would mean the world to us.",
    line3:       NONE,
    desc:        "optional internal note",
    depart_from: NONE
};
```

The guest's unique invitation URL:

```
https://your-domain.com/invitation/a1b2c3d4
```

---

## 7. App Configuration

The Rust database module (`src/database/mod.rs`) reads these constants:

| Constant | Default | Description |
|---|---|---|
| `DB_URL` | `127.0.0.1:8000` | WebSocket URL of the SurrealDB instance |
| `DB_NS` | `wedding` | Namespace |
| `DB_NAME` | `wedding` | Database name |
| `DB_USER` | `root` | Root username |
| `DB_PASS` | `secret` | Root password |

Change them directly in the source, or wire them up to environment variables
before building for production.

> **Security note:** The app connects to SurrealDB using root credentials over
> a local loopback socket. Never expose port `8000` to the public internet.
> Use a reverse proxy (nginx / Caddy) for the Dioxus server only.

---

## 8. Useful CLI Commands

```sh
# Check server version
surreal version

# Interactive SQL shell
surreal sql --conn ws://127.0.0.1:8000 --user root --password secret --ns wedding --db wedding

```sh
# List all guests with their resolved departure location name
SELECT uid, name, depart_from.location_name AS departure FROM people;

# Check a specific guest (by record key)
SELECT * FROM people:a1b2c3d4;

# Reset a guest's RSVP
UPDATE people:a1b2c3d4 SET depart_from = NONE;

# Set a guest's departure point (e.g. FPT Tower = location_map:1)
UPDATE people:a1b2c3d4 SET depart_from = location_map:1;

# Delete a guest
DELETE people:a1b2c3d4;
```

---

## 9. Upgrading SurrealDB

Re-run the install script — it replaces the binary in place:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://install.surrealdb.com | sh
sudo systemctl restart surrealdb
```

Check the [SurrealDB upgrade guide](https://surrealdb.com/docs/surrealdb/installation/upgrading)
before upgrading between major versions.
