#!/bin/sh
# Installs sync_frontmost_app.sh as a LaunchAgent on the Mac that runs Screen
# Sharing, and starts it. Run it again after moving this repository.
#
# Usage: install.sh [HOST]
#   HOST defaults to h-ms (REMOTE_HOST in src/rule_sets/apps/screen_sharing.rs).
set -eu

host=${1:-h-ms}
dir=$(cd "$(dirname "$0")" && pwd)
label=com.github.hioki.karaconf.remote-frontmost-app
plist=$HOME/Library/LaunchAgents/$label.plist
log=$HOME/Library/Logs/karaconf-remote-frontmost-app.log

mkdir -p "$(dirname "$plist")" "$(dirname "$log")"
cat >"$plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>$label</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/sh</string>
        <string>$dir/sync_frontmost_app.sh</string>
        <string>$host</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <!-- Retry interval while the host is unreachable -->
    <key>ThrottleInterval</key>
    <integer>10</integer>
    <key>StandardOutPath</key>
    <string>$log</string>
    <key>StandardErrorPath</key>
    <string>$log</string>
</dict>
</plist>
PLIST
plutil -lint "$plist" >/dev/null

# bootout returns before the job is fully gone, and an immediate bootstrap of
# the same label can fail.
launchctl bootout "gui/$(id -u)/$label" 2>/dev/null && sleep 1 || true
launchctl bootstrap "gui/$(id -u)" "$plist"
echo "Installed $plist (log: $log)"
