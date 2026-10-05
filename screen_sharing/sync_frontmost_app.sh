#!/bin/sh
# Keeps the Karabiner variable remote_frontmost_app (REMOTE_FRONTMOST_APP_VARIABLE
# in src/karabiner_data.rs) set to the frontmost app of the Mac controlled
# through Screen Sharing, so that the copies of the app rules fire for it.
# Runs as a LaunchAgent on the Mac that runs Screen Sharing (see install.sh),
# which starts it again whenever it exits.
#
# Usage: sync_frontmost_app.sh HOST
set -u

host=${1:?usage: $0 HOST}
dir=$(cd "$(dirname "$0")" && pwd)
karabiner_cli='/Library/Application Support/org.pqrs/Karabiner-Elements/bin/karabiner_cli'

# The same options as SSH_OPTIONS in src/rule_sets/apps/screen_sharing.rs, so
# this long-lived connection also keeps up the master that the VK2 commands
# reuse.
ssh -o BatchMode=yes -o ConnectTimeout=5 \
    -o ControlMaster=auto -o 'ControlPath=~/.ssh/karaconf-%C' -o ControlPersist=10m \
    -o ServerAliveInterval=5 -o ServerAliveCountMax=2 \
    "$host" 'swift -' <"$dir/frontmost_app.swift" |
    while IFS= read -r bundle_id; do
        # Anything outside the bundle identifier characters would break the JSON.
        case $bundle_id in
        *[!A-Za-z0-9.-]*) continue ;;
        esac
        printf '{"remote_frontmost_app":"%s"}\n' "$bundle_id"
    done |
    "$karabiner_cli" --set-variables-from-stdin

# The connection is gone: a stale value would keep firing the rules of an app
# that may no longer be frontmost.
"$karabiner_cli" --set-variables '{"remote_frontmost_app":""}'
