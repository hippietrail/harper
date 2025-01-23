---
title: Author a Rule
---

[Harper's grammatical rules are many](../rules), but most are relatively easy to understand.
Before we get into how to write a rule, it is important that we get some of the language cleared up.

When we refer to a Harper rule, we are talking about [an implementation of the Linter trait](https://docs.rs/harper-core/latest/harper_core/linting/trait.Linter.html).
As you can see, there is an enormous amount of flexibility in this trait and a wide variety of strategies for querying the provided document.

## Patterns

For new contributors, defining a pattern and [implementing the PatternLinter trait](https://docs.rs/harper-core/latest/harper_core/linting/trait.PatternLinter.html) is the easiest way to get started.
If you like to learn by example, you can make a copy of the `ThatWhich` rule and modify it to look for something else.
Here's the general playbook:

- Start by forking the [Harper monorepo](https://github.com/Automattic/harper/fork) and create a new file under `harper-core/src/linting` called `my_rule.rs`.
- Follow our [guide to get your environment set up.](./environment)
- Copy in a template rule (like from `that_which.rs`).
- Modify the constructor to create a pattern to look for the problematic text you have in mind.
- Export your rule from the `linting module` ([which you can find here](https://github.com/Automattic/harper/blob/master/harper-core/src/linting/mod.rs))
- Add your rule to the `LintGroup` [macro call](https://github.com/Automattic/harper/blob/master/harper-core/src/linting/lint_group.rs), which will aggregate its results with the other linters in Harper.
- Open a PR.

## Querying the Document Directly

If you don't wish to use a Harper [Pattern](https://docs.rs/harper-core/latest/harper_core/patterns/trait.Pattern.html), you may query the `Document` directly.
Make sure you read the [available methods](https://docs.rs/harper-core/latest/harper_core/struct.Document.html) available for `Document`s and for [TokenStrings](https://docs.rs/harper-core/latest/harper_core/struct.Document.html#impl-TokenStringExt-for-Document).
