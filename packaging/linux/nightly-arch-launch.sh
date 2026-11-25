#!/bin/sh
# The SoundZ Nightly launcher for the Arch handoff (packaging/linux/nightly-arch-tar.sh): the
# real GUI stays in .soundz-nightly/, and this starts it with the Launcher's private profile,
# as every other Nightly package does.
set -eu

bin="$(dirname "$0")/.soundz-nightly/soundz"
profile="${XDG_CONFIG_HOME:-${HOME:-}/.config}/soundz-nightly"

# The Launcher's protocol, and a bare version query, pass through untouched.
case "${1-}" in
  --launch-profile | --profile-root | --version | -V) exec "$bin" "$@" ;;
esac
exec "$bin" --launch-profile nightly-v1 --profile-root "$profile" -- "$@"
