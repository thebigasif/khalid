# shellcheck shell=bash
# The demos' pictures in AVIF, made once. Sourced, not run, by the scripts
# that build the demos: scripts/pages.sh, which builds what serpdad.com
# shows, and scripts/demo-check/showcase.sh, which builds what the layout
# and page-weight checks measure, so both offer the same files.
#
# AVIF is about half the size of WebP, so a phone at 3x downloads half the
# bytes, but it takes about 1.5 s of one core to make a 1,152 px file
# (docs/standards/performance.md, "Pictures on high-density phones"). A
# site turns it on with `avif = true` under [media]; every demo does. The
# files each build makes are linked into one cache shared by every build,
# and the next build links them back in, so each photo is made once for
# all the demos and all the runs. Each file is named by a hash of its
# photo, so a file found in the cache is the one the build would make.
# CI keeps the cache between runs; when the encoder changes, its key does.
# Files are linked, not copied, where the cache and the site share a disk.
#
#   DEMO_MEDIA_CACHE  the shared cache, outside the repository, by default
#                     ~/.cache/khalid/demo-media

demo_media_cache="${DEMO_MEDIA_CACHE:-${XDG_CACHE_HOME:-$HOME/.cache}/khalid/demo-media}"

# demo_media_in <site>: turns AVIF on in the site's site.toml and links
# every file the cache holds into the site's data/media/.
demo_media_in() {
  local settings="$1/site.toml"
  if ! grep -q '^avif *=' "$settings"; then
    if grep -qx '\[media\]' "$settings"; then
      sed -i '/^\[media\]$/a avif = true' "$settings"
    else
      printf '\n[media]\navif = true\n' >>"$settings"
    fi
  fi
  mkdir -p "$demo_media_cache" "$1/data/media"
  demo_media_link "$demo_media_cache" "$1/data/media"
}

# demo_media_out <site>: links the files the build made into the cache,
# keeping the ones already there.
demo_media_out() {
  [ -d "$1/data/media" ] || return 0
  demo_media_link "$1/data/media" "$demo_media_cache"
}

# demo_media_link <from> <to>: every file in <from> that <to> lacks, as a
# hard link, or as a copy when the two folders are on different disks.
demo_media_link() {
  cp -al --update=none "$1/." "$2/" 2>/dev/null || cp -a --update=none "$1/." "$2/"
}
