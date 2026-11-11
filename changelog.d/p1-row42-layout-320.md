Changed
- The layout check now opens every template's showcase at 320 px too, the smallest phone a page must fit and what a 1280 px window shows at 400% zoom. Before, it started at 390 px, so a page that scrolled sideways on a small phone passed. `just layout-selftest` proves a fault seen only at 320 px is caught (BL-WEB-3, gap audit 4 row 42).

Fixed
- On a 320 px phone, an author page no longer scrolls sideways when a profile link shows a long web address (Dialogue, Glossy, Letterpress, Vital), and a review score's bar moves under its number when a column is too narrow for both (Respawn).
