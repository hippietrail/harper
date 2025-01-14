---
title: Set Up Your Environment
---

To use the tooling required to build and debug Harper, you'll need to the following programs available in your `$PATH`.

- [`just`](https://github.com/casey/just)
- `bash`
- [`cargo`](https://www.rust-lang.org/) (we develop against the latest version of Rust)
- `yarn`
- `node`
- `grep`
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)
- `zip`
- `pandoc`

We develop a set of tools, accessible via `just`, to build and debug Harper's algorithm (otherwise known as `harper-core`) and its various integrations.
To get see all the tools in your toolbox run:

```bash
just --list
```

Before getting started, we highly recommend that you run `just setup` to populate your build caches and download all dependencies.
