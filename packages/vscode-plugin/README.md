# Harper for VS Code

Harper is the next-generation grammar checker for your code. It catches common stylistic errors, as well as complex grammatical or layout-related problems. It works for almost [all common programming languages](./language-server#Supported-Languages) and a number of markup formats.

If you use Rust, Java, JavaScript, or any number of other programming languages, you comments may be ending up as part of your API's documentation. If that's the case, grammatical mistakes in your code could be down-ranking your site on search results and tarnishing your reputation for quality.

Most importantly, Harper runs on-device and uses barely any memory at all. That means you can get feedback on your work in milliseconds, dramatically increasing your iteration speed.

## Installation

Installation should be relatively straightforward.
It just depends on which editor and marketplace you're using.

If you use the official Microsoft Visual Studio Code release, go ahead and go to the marketplace and search for "Harper" and click "Install".
You can also visit our [official page](https://marketplace.visualstudio.com/items?itemName=elijah-potter.harper&ssr=false#overview).

If you use OpenVSX, for instance if you use VSCodium, you'll want to install from [here](https://open-vsx.org/extension/elijah-potter/harper).

### Commands

| Command                         | Id                              | Description         |
| ------------------------------- | ------------------------------- | ------------------- |
| Harper: Restart Language Server | `harper.languageserver.restart` | Restart `harper-ls` |

### Settings

| Setting                        | Type                                              | Default Value   | Description                                                                                                                                                 |
| ------------------------------ | ------------------------------------------------- | --------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `harper-ls.path`               | `string`                                          | `""`            | Optional path to a `harper-ls` executable to use. Primarily useful if the bundled binary doesn't work in your system like in immutable Linux distributions. |
| `harper-ls.linters.*`          | `boolean`                                         | Varies          | Detect and provide suggestions in a variety of common situations.                                                                                           |
| `harper-ls.diagnosticSeverity` | `"error"`, `"hint"`, `"information"`, `"warning"` | `"information"` | How severe do you want diagnostics to appear in the editor?                                                                                                 |

## Developing and Contributing

See the [Development Guide](/packages/vscode-plugin/development-guide.md).
