Added
- A site can add its own hedge words and vague sentence openers in a `[words]` table in `lint.toml`, such as words in its own language. The linter's `plain-answer` rule and the audit's `hedged-answer` check now read one shared list, so `khalid lint` and `khalid audit` always flag the same words.
