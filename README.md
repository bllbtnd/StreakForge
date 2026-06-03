# StreakForge

A GitHub streak card generator written in Rust.  
Paste a URL into your README and get a 200×200 animated SVG card showing any user's current contribution streak.

---

## Usage

No setup required. Just use the live URL:

```md
![GitHub Streak](https://streakforge.ballabotond.com/card/yourname)
```

Or in HTML:

```html
<img src="https://streakforge.ballabotond.com/card/yourname" alt="GitHub Streak" width="200" height="200" />
```

Swap `yourname` for any GitHub username. That's it.

---

## Endpoints

### `GET /card/:username`

Returns a 200×200 SVG streak card for the given user.

| Parameter | Type   | Default | Description                                      |
|-----------|--------|---------|--------------------------------------------------|
| `tz`      | string | `UTC`   | IANA timezone for streak calculation (e.g. `Europe/Budapest`) |

```
https://streakforge.ballabotond.com/card/yourname?tz=Europe/Budapest
```

### `GET /leaderboard`

Returns a 400×420 SVG leaderboard of the top 10 streaks seen by the service.

```
https://streakforge.ballabotond.com/leaderboard
```

---

## Flame colors

The flame color shifts as your streak grows — from a calm yellow to an intense blue:

| Streak     | Color              |
|------------|--------------------|
| 1–6 days   | `#ffd700` yellow   |
| 7–13 days  | `#ffaa00` golden   |
| 14–29 days | `#ff6600` orange   |
| 30–59 days | `#ff3300` red-orange |
| 60–99 days | `#ee1111` red      |
| 100–149    | `#cc0066` magenta  |
| 150–199    | `#9900cc` purple   |
| 200–364    | `#5500ff` violet   |
| 365+       | `#0055ff` blue     |

The flicker animation also speeds up the longer your streak.

---

## Caching

Cards are cached in Cloudflare D1 for **1 hour** — fast for readers, fresh enough for daily streaks.  
Users inactive for more than 30 days are evicted and re-fetched on next request.

---

## Self-hosting

```bash
# 1. Clone
git clone <repo-url> && cd streakforge

# 2. Create a D1 database
wrangler d1 create streakforge
# → paste the returned database_id into wrangler.toml

# 3. Apply the schema
wrangler d1 execute streakforge --remote --file=schema.sql

# 4. Set your GitHub token (classic token, read:user scope)
wrangler secret put GITHUB_TOKEN

# 5. Deploy
wrangler deploy
```

### Environment

| Variable       | Required | Description                                |
|----------------|----------|--------------------------------------------|
| `GITHUB_TOKEN` | Yes      | GitHub personal access token (`read:user`) |
| `DB`           | Yes      | Cloudflare D1 binding (set in wrangler.toml) |

### Development

```bash
# Type-check for the WASM target
cargo check --target wasm32-unknown-unknown

# Build the worker
worker-build --release

# Local preview (uses local D1 simulation)
wrangler dev

# Deploy
wrangler deploy
```

---

## Tech stack

- **Rust** compiled to WebAssembly via [`worker`](https://crates.io/crates/worker) crate
- **Cloudflare Workers** — runs at the edge, zero cold starts
- **Cloudflare D1** — SQLite for streak persistence and leaderboard
- **GitHub GraphQL API** — contribution calendar data
