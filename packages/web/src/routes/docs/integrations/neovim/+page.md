---
title: Neovim
---

Our Neovim integration is powered by [`harper-ls`](./language-server).

## Installation

How you choose to install `harper-ls` depends on your use-case. For Neovim, we only directly support usage through [`nvim-lspconfig`](https://github.com/neovim/nvim-lspconfig/blob/master/doc/configs.md#harper_ls) and installation via [`mason.nvim`](https://mason-registry.dev/registry/list?search=harper-ls). However, you can also [install it separately](./language-server#Installation) and set it up yourself if you wish.

## Configuration

Below is an example config where everything is set to their default values:

```lua
lspconfig.harper_ls.setup {
  settings = {
    ["harper-ls"] = {
      userDictPath = "",
      fileDictPath = "",
      linters = {
        SpellCheck = true,
        SpelledNumbers = false,
        AnA = true,
        SentenceCapitalization = true,
        UnclosedQuotes = true,
        WrongQuotes = false,
        LongSentences = true,
        RepeatedWords = true,
        Spaces = true,
        Matcher = true,
        CorrectNumberSuffix = true,
      },
      codeActions = {
        ForceStable = false
      },
      markdown = {
        IgnoreLinkTitle = false,
      },
      diagnosticSeverity = "hint",
      isolateEnglish = false
    }
  }
}
```

:::note
This example only containes some of the available linters, head over to our [rules page](../rules) to view the full list.
:::

For more information on what each of these configs do, you can head over to the [configuration section](./language-server#Configuration) of our `harper-ls` documentation.
