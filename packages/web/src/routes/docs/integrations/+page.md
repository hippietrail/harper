---
title: Integrations
---

# {title}

## Obsidian

![A screenshot of Obsidian with Harper installed](/images/obsidian_screenshot.webp)

To add Harper to [Obsidian](/obsidian), just install the community plugin.

More specifically, go to Obsidian's settings and click "Community Plugins."
You may be asked to enable them.

Next, click "Browse" and search for "Harper".
Finally, go back to the Community Plugins page and enable the plugin.


![A screenshot of the Harper Obsidian installation.](/images/obsidian_install_screenshot.webp)

## Emacs

Our Emacs support is limited and primarily provided by the community.
You can see the longer discussion [here](https://github.com/Automattic/harper/discussions/150).

## Helix

To use Harper in [Helix](https://helix-editor.com/), you'll need to have `harper-ls` installed.

Once you do, add this to you configuration:

```toml
[language-server.harper-ls]
command = "harper-ls"
args = ["--stdio"]
```
