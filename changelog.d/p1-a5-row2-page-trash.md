Changed
- Deleting a page now moves it to the trash, like a post, instead of removing its file for good. The Trash screen lists it as a live page, and "Put it back" makes it live again at the same address. It is emptied after `trash_days`, like a post.
- The Admin API's trash list says what each copy was, in a new `kind` field (`post` or `page`). Restoring a page answers 204 with no body; restoring a post still answers the post.
