Added
- An owner can delete one form answer for good, for example when the visitor who sent it asks to be erased under the GDPR. The Forms screen has a Delete button on each answer that asks once more, and the Admin API adds `DELETE /api/admin/v1/forms/{id}/answers/{answer}/`, needing `subscribers:write`. The audit log keeps that an answer was deleted, never what it said.
