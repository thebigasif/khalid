Added
- Write `[[fact:pro-plan-price]]` in a post to show a value from `facts.toml`. Change it once there and every post that names it shows the new value at the next build. A key `facts.toml` does not have stops `khalid build` and `khalid lint` and names the line.
- A fact in `facts.toml` may name its `source`, the page where you check the value. The warning to check a fact again links it.
- `khalid lint` warns when a fact's `checked` date has not come yet (`fact-checked-in-future`), and when a post's brief plans a link to a post your site does not have (`brief-link-unknown`), suggesting the post's current address when it moved.
