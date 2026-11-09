Fixed
- The Docker guide in `docs/deploy.md` now shows how to make the admin's owner inside the image, and says to leave out `--site`, which the image adds itself. A new check, `scripts/packages-check.sh`, runs the Docker image and the systemd service exactly as the guide says and fetches the home page from each.
