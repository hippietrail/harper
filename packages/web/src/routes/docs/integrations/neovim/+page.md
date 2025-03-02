---
title: Neovim
---

Our Neovim integration is powered by [`harper-ls`](./language-server).

## Installation

`harper-ls` can be installed with [`mason.nvim`](https://mason-registry.dev/registry/list?search=harper-ls) or via any of our other [supported installation methods](./language-server#Installation). [Neovim supports language servers out-of-the-box](https://neovim.io/doc/user/lsp.html), but for ease of use, we suggest using `harper-ls` through [nvim-lspconfig](https://github.com/neovim/nvim-lspconfig/tree/master).

## Configuration

Below is an example config using nvim-lspconfig where everything is set to their default values:

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
        CorrectNumberSuffix = true
      },
      codeActions = {
        ForceStable = false
      },
      markdown = {
        IgnoreLinkTitle = false
      },
      diagnosticSeverity = "hint",
      isolateEnglish = false
    }
  }
}
```

:::note
This example only contains some of the available linters, check out our [rules page](../rules) to view the full list.
:::

For more information on what each of these configs do, you can head over to the [configuration section](./language-server#Configuration) of our `harper-ls` documentation.

## Additional Links

- [nvim-lspconfig's documentation on `harper-ls`](https://github.com/neovim/nvim-lspconfig/blob/master/doc/configs.md#harper_ls)
