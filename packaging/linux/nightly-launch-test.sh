#!/usr/bin/env bash
# The launcher inside the plain Nightly tarball (packaging/linux/nightly-launch.sh): a launch adds
# the Launcher's private profile, a version query or an already-formed protocol passes through, and
# the profile has the shape the app's fence accepts (absolute, no parent traversal, outside this
# unpacked copy).
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
mkdir -p "$work/tree"

# An argv recorder stands in for the GUI. It never opens a window.
cat >"$work/tree/soundz-bin" <<'RECORDER'
#!/bin/sh
printf '<%s>\n' "$@"
RECORDER
chmod 755 "$work/tree/soundz-bin"
cp "$HERE/nightly-launch.sh" "$work/tree/soundz"
chmod 755 "$work/tree/soundz"

profile="${XDG_CONFIG_HOME:-${HOME:-}/.config}/soundz-nightly"
case "$profile" in
  /*) ;;
  *) echo "FAIL: the launcher profile is not absolute: $profile" >&2; exit 1 ;;
esac
case "/$profile/" in
  */../*) echo "FAIL: the launcher profile contains parent traversal" >&2; exit 1 ;;
esac
case "$profile/" in
  "$work/tree/"*) echo "FAIL: the launcher profile is inside the unpacked copy" >&2; exit 1 ;;
esac

expect_launch() {
  expected="$(printf '<%s>\n' --launch-profile nightly-v1 --profile-root "$profile" -- "$@")"
  actual=$("$work/tree/soundz" "$@")
  [[ "$actual" == "$expected" ]] || {
    printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual" >&2
    exit 1
  }
}
expect_forwarded() {
  expected="$(printf '<%s>\n' "$@")"
  actual=$("$work/tree/soundz" "$@")
  [[ "$actual" == "$expected" ]] || {
    printf 'expected:\n%s\nactual:\n%s\n' "$expected" "$actual" >&2
    exit 1
  }
}

expect_launch
expect_launch 'session with spaces.sc' '--other-name.sc'
expect_forwarded --version
expect_forwarded -V
expect_forwarded --launch-profile nightly-v1 --profile-root /launcher/profile --version

# The launcher must be a script, not the GUI under its own name: an unpacked tar has to start the
# private profile even when nobody passes a flag.
head -n 1 "$work/tree/soundz" | grep -qx '#!/bin/sh' || { echo "FAIL: soundz is not the launcher" >&2; exit 1; }

echo "PASS: tarball Nightly launcher starts the GUI with the Launcher profile and forwards files"
