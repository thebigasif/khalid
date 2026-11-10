Added
- A media file can now be deleted, from the Delete button on its details page in the admin or with `DELETE /api/admin/v1/media/{id}/?updated_at=...`. The file goes for good. While a post, a page or the theme still uses it, Khalid keeps the file and names the files that use it, so no post is left with a broken picture.
- The folder field in Media now suggests the folders the library already has, from the new `GET /api/admin/v1/media/folders/`. A new folder can still be typed; it is made with the first file filed in it.
