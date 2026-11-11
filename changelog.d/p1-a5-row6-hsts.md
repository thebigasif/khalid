Added
- `khalid serve --https` and `--domain` send `Strict-Transport-Security: max-age=31536000` on every page, so browsers keep to HTTPS for a year and Khalid's own audit no longer notes "No HSTS support". Plain HTTP sends no HSTS, because a proxy in front ends TLS and sets that header.
