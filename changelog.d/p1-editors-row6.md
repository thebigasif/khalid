Fixed
- On `khalid serve`, the admin now knows who last changed each post. Earlier versions show who saved them, and an edit made on an old copy names the person who changed the post first, not "Someone". Khalid keeps the names in `data/post-editors.txt`, never in your post files.
- The dashboard's recent drafts show the name of whoever changed each one, not the 16-digit code of their key. The Admin API's post rows carry it as `updated_by_name`.
