Fixed
- A site on a server built under a mount, such as `/blog/`, now posts its form blocks and sign-up forms under that mount, as `khalid serve` already did. Before, `khalid build` wrote `/forms/contact`, an address the proxy in front of the blog does not pass on.
