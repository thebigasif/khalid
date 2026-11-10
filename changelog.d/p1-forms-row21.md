Added
- A Forms screen in the admin lists what visitors sent through each form block, newest first, a page at a time. Owners and admins see it; editors and writers do not, because answers hold visitors' email addresses. The Admin API adds `GET /api/admin/v1/forms/` and `GET /api/admin/v1/forms/{id}/answers/`, both needing `subscribers:write`.
