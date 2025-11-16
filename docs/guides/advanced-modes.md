# Guide: Advanced Modes

Spinel CLI includes several specialized modes for diagnostics and server management, similar to `redis-cli`.

**Note**: Some of these modes are still under development.

To use a mode, start `spinel-cli` with the appropriate flag.

---

### Scan Mode (`--scan`)

Iterates through the keyspace without blocking the server, making it a safe alternative to `KEYS` in production environments.

**Status**: Not yet fully implemented.

---

### Latency Monitoring Mode (`--latency`)

Helps diagnose latency issues by measuring the time it takes for a `PING` command to be acknowledged.

**Status**: Not yet fully implemented.

---

### Stat Mode (`--stat`)

Provides a real-time stream of server statistics, similar to the `INFO` command but updated continuously.

**Status**: Not yet fully implemented.

---

### Big Keys Mode (`--bigkeys`)

Scans the keyspace to find the largest keys, helping you identify potential memory issues.

**Status**: Not yet fully implemented.

---

### Memory Keys Mode (`--memkeys`)

Reports the memory usage of keys.

**Status**: Not yet fully implemented.

---

### Hot Keys Mode (`--hotkeys`)

Identifies keys that are being accessed frequently.

**Status**: Not yet fully implemented.

---

### Cluster Mode (`--cluster`)

Provides commands for managing a SpinelDB cluster.

**Status**: Not yet fully implemented.
