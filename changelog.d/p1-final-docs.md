Fixed
- The getting-started guide now says what `khalid build` does with a blocked post: it leaves that post out and writes the rest. It used to say the build wrote nothing.
- The nginx rule from `khalid init --stack nginx` no longer adds `proxy_ssl_server_name on;` when the blog's address is plain `http://`, where the line does nothing.
