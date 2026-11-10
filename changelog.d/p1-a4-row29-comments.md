Added
- Comments can be marked as spam and moved back. On the Comments screen, Spam hides a reader's comment from its post and keeps it under Spam, and Not spam puts it back among the waiting ones. Programs do the same through the Admin API: `POST /api/admin/v1/comments/{id}/spam/` marks a comment, and `DELETE` on the same address moves it back. The MCP server lists both as tools.
