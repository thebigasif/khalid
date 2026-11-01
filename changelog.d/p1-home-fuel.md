Fixed
- A blog with thousands of posts builds and serves again. Its home page lists every post, and it stopped with "template home.html failed while rendering" because a page had a fixed limit on its work. A page now gets more room for each post it lists, and a template that loops without end is still stopped.
