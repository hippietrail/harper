---
title: Helix
---

Our Helix integration is powered by [`harper-ls`](./language-server).

## Installation

[Helix supports language servers out-of-the-box](https://docs.helix-editor.com/languages.html#language-server-configuration), so all you have to do is [install `harper-ls` in your system](./language-server#Installation).

## Configuration

This the minimum you need to add to your `languages.toml` to get up and running:

```toml title=languages.toml
[language-server.harper-ls]
command = "harper-ls"
args = ["--stdio"]
```

Additionally, you can also configure things like which linters to use or how you want code actions to appear. Below is an example config where everything is set to their default values:

```toml title=languages.toml
[language-server.harper-ls.config.harper-ls]
userDictPath = ""
fileDictPath = ""
diagnosticSeverity = "hint"
isolateEnglish = false

[language-server.harper-ls.config.harper-ls.linters]
SpellCheck = true
SpelledNumbers = false
AnA = true
SentenceCapitalization = true
UnclosedQuotes = true
WrongQuotes = false
LongSentences = true
RepeatedWords = true
Spaces = true
Matcher = true
CorrectNumberSuffix = true

[language-server.harper-ls.config.harper-ls.codeActions]
ForceStable = false

[language-server.harper-ls.config.harper-ls.markdown]
IgnoreLinkTitle = false
```

:::note
This example only contains some of the available linters, check out our [rules page](../rules) to view the full list.
:::

For more information on what each of these configs do, you can head over to the [configuration section](./language-server#Configuration) of our `harper-ls` documentation.

## Additional Links

- [Helix's official documentation on `harper-ls`](https://github.com/helix-editor/helix/wiki/Language-Server-Configurations#harper-ls)
- [Community discussion on configuring `harper-ls` for Helix](https://github.com/Automattic/harper/discussions/135)
