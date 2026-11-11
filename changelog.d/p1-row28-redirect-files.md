Added
- A static build now writes its redirects for nginx (`nginx-redirects.conf`) and Apache (`.htaccess`) as well as `_redirects`. A blog on either server keeps every moved address with one `include` line, or none. Before, only Netlify and Cloudflare Pages read the redirects (KH-SEO-8, gap audit 4 row 28).
