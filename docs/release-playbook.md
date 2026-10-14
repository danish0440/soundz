# Release playbook

The release playbook shared by every crafting app lives in craftrules:
[`../craftrules/release/playbook.md`](../../craftrules/release/playbook.md)
(`storytold/craftrules`). It is the canonical copy; don't duplicate it here.

SoundZ's implementation (`.github/workflows/release.yml`, `packaging/`,
`cargo xtask version`) was copied from DesignZ/PhotoZ and renamed. SoundZ-specific
details are in [releasing.md](releasing.md).
