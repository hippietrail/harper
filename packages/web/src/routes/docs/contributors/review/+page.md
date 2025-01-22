---
title: Reviewing Pull Requests
---

There are a lot of individual components and artifacts that make up Harper.
How a patch gets reviewed depends significantly on which component or artifact it affects.

If a patch only affects a grammar rule or otherwise something part of `harper-core` (often it is labeled as such in GitHub), you can build and test the change with **any** of Harper's frontends.

## Using GitHub Actions Artifacts

We run builds for a variety of platforms whenever a Pull Request is pushed to.
You can use these to review changes to various aspects of Harper including `harper-ls`, `harper-cli` and the Visual Studio Code plugin.

![How to download the Windows Visual Studio Code plugin from the GitHub Actions run.](/images/download_artifact.gif)

## Using Cargo

If you only have [Cargo](https://doc.rust-lang.org/cargo/) installed, you can compile either `harper-ls` (the language server) or `harper-cli` (our command-line debug tool) directly from the branch in GitHub:

```bash
cargo install --git https://github.com/automattic/harper --branch <branch-name> <binary-artifact>
```

For example, for [PR #445](https://github.com/Automattic/harper/pull/455), we can install the patched version of the `harper-cli` debug tool with the following command:

```bash
cargo install --git https://github.com/automattic/harper --branch somewhat-something harper-cli
```

From there you can run the tool on any file with `harper-cli lint`.

## Using Docker

We build our web documentation in a Docker image.
This documentation includes a [demo](/), so you can also use this image to review changes to linting rules and other aspects of the core algorithm.

```bash
git clone https://github.com/automattic/harper
cd harper
git switch <branch from PR>
IMAGE_HASH=$(docker build . -q)
docker run -p 3000:3000 -it $IMAGE_HASH
```
