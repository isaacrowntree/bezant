# bezant-relogin

Pi-side IBKR Client Portal Gateway re-login service.

## What it does

The IBKR Client Portal Gateway minted by `docker compose up` requires
interactive 2FA login, and the session ages out roughly every 24 hours.
This script automates the boring parts of re-login:

- Polls `bezant-server`'s `/health` endpoint every 5 min via systemd timer
- When `authenticated=false`, drives a headless Playwright browser at
  `https://localhost:5000` to fill username + password
- Triggers an IB Key push to your phone — you tap "Approve"
- Polls `/health` for up to 2 min until it flips to `authenticated=true`
- **One push per session-expiry event.** If the push isn't tapped within
  2 minutes, writes a `disabled` sentinel and stops auto-firing. Manual
  reset (one SSH command) triggers a fresh attempt at a time of your
  choosing.

Why one push instead of three: empirically, when you're away (asleep,
gym, meeting) running multiple attempts inside the 5-min timer window
just spams the phone with IB Key pushes you can't respond to. One push
preserves the natural escape valve while you're around, and goes silent
when you're not.

It does **not** automate the IB Key approval — by design. That stays a
human-in-the-loop step so the Pi can't be silently logged in if the
credentials leak.

## Architecture

```
systemd timer (5 min) ──▶ bezant-relogin.service ──▶ npx tsx index.ts
                                                            │
                                                            ▼
                                            GET http://localhost:8080/health
                                                            │
                                              authenticated=false?
                                                            │
                                                            ▼
                              Playwright (headless Chromium) → https://localhost:5000
                                                            │
                                                            ▼
                                           IB Key push → your phone → tap Approve
                                                            │
                                                            ▼
                                           poll /health until authenticated=true
```

## Pi setup

These commands assume you're running as `isaac` on the Pi (`zackdesign-pi`).

### 1. Get the code onto the Pi

```bash
# from your laptop
scp -r relogin zackdesign-pi:~/bezant-relogin

# or pull from github on the Pi
ssh zackdesign-pi
cd ~ && git clone https://github.com/isaacrowntree/bezant.git
ln -s ~/bezant/relogin ~/bezant-relogin
```

### 2. Install runtime dependencies

```bash
ssh zackdesign-pi
cd ~/bezant-relogin
npm install
npm run install-browsers   # downloads Chromium (~150 MB)
```

If `playwright install` complains about missing system libs, run
`sudo npx playwright install-deps chromium` once.

### 3. Configure credentials

```bash
ssh zackdesign-pi
cd ~/bezant-relogin
cp .env.example .env
chmod 600 .env
$EDITOR .env   # fill IBKR_USERNAME and IBKR_PASSWORD
```

If you remapped the host port for CPGateway (macOS AirPlay collision),
also override `BEZANT_LOGIN_URL` here.

### 4. Install + enable the systemd user units

```bash
ssh zackdesign-pi
mkdir -p ~/.config/systemd/user
cp ~/bezant-relogin/systemd/bezant-relogin.service ~/.config/systemd/user/
cp ~/bezant-relogin/systemd/bezant-relogin.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now bezant-relogin.timer
```

### 5. Allow the user units to keep running when you're not logged in

User units stop when your SSH session ends unless lingering is enabled:

```bash
sudo loginctl enable-linger isaac
```

### 6. Verify

```bash
# show timer schedule
systemctl --user list-timers bezant-relogin.timer

# tail the live log
journalctl --user -u bezant-relogin -f

# fire one attempt manually
systemctl --user start bezant-relogin.service
```

## Manual re-trigger / reset after auto-disable

If the script self-disables (3 consecutive timeouts), reset it from your
laptop with:

```bash
ssh zackdesign-pi 'rm -f ~/.local/state/bezant-relogin/disabled && systemctl --user start bezant-relogin.service'
ssh zackdesign-pi 'sleep 6 && journalctl --user -u bezant-relogin -n 30 --no-pager'
```

## State

Persisted at `~/.local/state/bezant-relogin/`:

- `state.json` — `{ consecutiveFailures, lastAttemptAt, lastSuccessAt }`
- `disabled` — sentinel file. If present, the script exits silently.
  Created automatically after 3 consecutive failures, removed on manual
  reset or after a successful login.

## Selectors

The Playwright login uses these CSS selectors against the CPGateway
login page:

- `#user_name, input[name="username"]`
- `#password, input[name="password"]`
- `#submitForm, button[type="submit"], input[type="submit"]`

If IBKR redesigns the login form, update them in `index.ts`. Verify
against the live page source at `https://localhost:5000`.

## Why not full automation (TOTP)?

Putting the second factor on the same machine as the credentials defeats
the purpose of having a second factor at all. With IB Key push, the Pi
holds username + password but cannot complete a login without a
human-held device tapping "Approve". The credentials leaking from the Pi
isn't sufficient to log in.

If you want zero-touch logins later, the path is: switch IBKR to TOTP,
add `otplib` to this script, generate codes locally, fill the third
input. The cost is the security trade-off above.
