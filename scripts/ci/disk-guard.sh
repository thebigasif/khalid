#!/usr/bin/env bash
# Free disk space on a maintainer runner before a Rust job builds. A full
# disk made the linker die with a bus error, so a job that cannot get
# enough space fails here, clearly, before it builds anything.
#
# Every run removes this checkout's incremental build folders older than a
# day. When the disk has less than DISK_GUARD_MIN_GB free (default 40), it
# also removes the target folders of the other runners on this machine
# that nothing has written to for two hours, so a running job is never
# touched. Only then, when the space is still short, the job fails.
#
# Usage: scripts/ci/disk-guard.sh   (in a job on a maintainer runner)
set -euo pipefail

min="${DISK_GUARD_MIN_GB:-40}"
workspace="${GITHUB_WORKSPACE:?Run this inside a GitHub Actions job.}"
runners="$(realpath "${RUNNER_WORKSPACE:?Run this inside a GitHub Actions job.}/../../..")"

free_gb() { df --output=avail -BG "$workspace" | tail -n 1 | tr -dc '0-9'; }

if [ -d "$workspace/target" ]; then
  find "$workspace/target" -maxdepth 4 -type d -name incremental -mtime +1 -prune -exec rm -rf {} + || true
fi

free="$(free_gb)"
echo "Free disk space: $free GB (at least $min GB needed)."
if [ "$free" -lt "$min" ]; then
  for target in "$runners"/runner-*/_work/*/*/target; do
    [ -d "$target" ] || continue
    [ "$(realpath "$target")" = "$(realpath -m "$workspace/target")" ] && continue
    if [ -z "$(find "$target" -newermt '2 hours ago' -print -quit 2>/dev/null)" ]; then
      echo "Removing $target, idle for two hours or more ($(du -sh "$target" 2>/dev/null | cut -f1))."
      rm -rf "$target"
    fi
  done
  free="$(free_gb)"
  echo "Free disk space after pruning: $free GB."
fi
if [ "$free" -lt "$min" ]; then
  echo "Only $free GB is free on this runner's disk, under the $min GB a Rust build needs. A full disk breaks the linker, so this job stops here. Free space on the machine, then run the job again."
  exit 1
fi
