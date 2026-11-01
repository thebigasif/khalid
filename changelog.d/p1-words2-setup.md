Fixed
- `khalid staff owner` names the port `khalid serve` last ran on in its link for this computer, so a blog started with `--listen 127.0.0.1:3000` gets a link to port 3000 without being told. `--listen` still sets it by hand.
- `cargo build` warns, in three plain lines, when `admin/dist/` is empty, so a build from source without the npm steps says it has no admin and how to add it.
- The README's quick start starts `khalid serve` with `MIMALLOC_PURGE_DELAY=0`, as the Docker image and the systemd service do, so a blog started by hand gives unused memory back at once.
