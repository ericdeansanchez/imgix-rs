Summarize changes in around 50 characters or less

More detailed explanatory text, if necessary. Wrap it to about 72
characters or so. In some contexts, the first line is treated as the
subject of the commit and the rest of the text as the body. The
blank line separating the summary from the body is critical (unless
you omit the body entirely).

Explain the problem that this commit is solving. Focus on _why_ you
are making this change as opposed to how. 

This *__guide__* is meant to help not hinder. If you're unsure about
any of this, just write a subject line and we'll do the rest together.

- Bulleted lists are cool!
  - Sub-items are also cool!
- Use an imperative voice like git does:
  - e.g. _Merge_ branch 'feature', _Revert_ "The things"
  - e.g. _Fix_ minor typo in README.md
- Capitalize the subject line.
- Do not end the subject line with a period.
- Read Contributing.md for more information.

Don't be afraid to use markdown if you'd like. If you think your PR
needs a cool table, do it.

| test        | expected | actual |
|-------------|----------|--------|
| `add(1, 2)` | 3        | 4      |

- [ ] code is properly formatted (i.e. `cargo fmt`)
- [ ] code is adequately tested (i.e. `cargo test`)
- [ ] read Contributing.md for `git hooks`.

This repository uses an issue tracker, you can put references to them
at the bottom, like this:

Fixes: #123
Closes: #456
See also: #457, #789