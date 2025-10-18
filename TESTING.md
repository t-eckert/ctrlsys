# Testing Guide - Sprint 2: Timers

## Prerequisites

1. **PostgreSQL Database**
   - Install PostgreSQL locally or use Docker:
   ```bash
   docker run --name ctrlsys-postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:15
   ```

2. **Create Database**
   ```bash
   createdb ctrlsys
   # Or with Docker:
   docker exec -it ctrlsys-postgres createdb -U postgres ctrlsys
   ```

3. **Set Environment Variables**
   ```bash
   export DATABASE_URL="postgresql://postgres:postgres@localhost/ctrlsys"
   export CTRLSYS_API_TOKENS="test-token-123"
   export CTRLSYS_PORT="3000"
   ```

## Running the Server

```bash
cargo run --bin server --features server
```

Expected output:
```
Database connection established
Database migrations completed
Background tasks started
Server listening on 0.0.0.0:3000
```

## Configuring the CLI

```bash
cargo run --bin cli --features cli -- config set-server http://localhost:3000
cargo run --bin cli --features cli -- config set-token test-token-123
cargo run --bin cli --features cli -- config show
```

## Testing Timer Features

### 1. Create a Timer

```bash
cargo run --bin cli --features cli -- timer create "Test Timer" 30
```

Expected output:
```
Timer created and started!
  Name: Test Timer
  ID: <uuid>
  Duration: 30 seconds

Watch it with: cs timer watch <uuid>
```

### 2. List Timers

```bash
cargo run --bin cli --features cli -- timer list
```

Expected output:
```
Timers:

  <uuid> - Test Timer (running) - 25 seconds remaining
  <uuid> - Old Timer (completed) - finished 5 minutes ago
```

**Notes:**
- Running timers show remaining time
- Completed timers show how long ago they finished
- Timers completed more than 24 hours ago are automatically hidden
- Timers are sorted by status (running, pending, completed, cancelled)

### 3. Watch Timer (TUI Mode)

```bash
cargo run --bin cli --features cli -- timer watch <uuid>
```

This opens a full-screen TUI showing:
- Timer name and status
- Progress bar
- Remaining time in MM:SS format
- Live updates every second via WebSocket

Press 'q' to quit.

### 4. Watch All Active Timers (TUI Mode)

```bash
cargo run --bin cli --features cli -- timer watch-all
```

This opens a full-screen TUI showing a table of all running timers:
- Timer name
- Total duration
- Remaining time
- Status

Features:
- Updates every second via REST API polling
- Shows only running timers
- Press 'q' to quit

This is useful when you have multiple timers running and want to monitor them all at once.

## API Endpoints

You can also test the REST API directly:

```bash
# List timers
curl http://localhost:3000/api/v1/timers \
  -H "Authorization: Bearer test-token-123"

# Create timer
curl -X POST http://localhost:3000/api/v1/timers \
  -H "Authorization: Bearer test-token-123" \
  -H "Content-Type: application/json" \
  -d '{"name":"API Timer","duration_seconds":60}'

# Get specific timer
curl http://localhost:3000/api/v1/timers/<uuid> \
  -H "Authorization: Bearer test-token-123"

# Cancel timer
curl -X DELETE http://localhost:3000/api/v1/timers/<uuid> \
  -H "Authorization: Bearer test-token-123"
```

## WebSocket Testing

You can connect to the WebSocket endpoint using `wscat` or similar tools:

```bash
npm install -g wscat

wscat -c ws://localhost:3000/api/v1/timers/<uuid>/ws \
  -H "Authorization: Bearer test-token-123"
```

You'll receive JSON timer updates every second.

## Troubleshooting

### "Connection refused"
- Make sure the server is running
- Check that the port 3000 is available
- Verify DATABASE_URL is set correctly

### "Unauthorized" errors
- Make sure you've set the API token in CLI config
- Verify the token matches what's in CTRLSYS_API_TOKENS

### Database errors
- Run migrations manually: `sqlx migrate run`
- Check database permissions
- Verify database exists

## Success Criteria

- [x] Server starts and runs migrations
- [x] CLI can create timers
- [x] CLI can list timers
- [x] TUI watch mode shows live countdown
- [x] Timers auto-complete after duration expires
- [x] WebSocket provides real-time updates
- [x] Background task marks expired timers as completed
