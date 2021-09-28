# Development container for Rust

## Requirements

- [Docker and docker-compose](https://hub.docker.com/search?offering=community&q=&type=edition)

For Visual Studio Code development (recommended):
- [Visual Studio Code](https://code.visualstudio.com/)
- [Remote Containers extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

Rust (rustup, cargo, etc.) is not required on the host, that is the whole point of the container based environment.

## Usage

1. Create a new directory for your next Rust project
2. Copy the `.devcontainer` folder and the files within from this repo to your new folder
3. Start the container and attach to it:
   - for terminal based development (vim): use `docker-compose run vscode` from the `.devcontainer` folder
      ```bash
      ~/new-project $ cd .devcontainer
      ~/new-project/.devcontainer $ ls
      Dockerfile  devcontainer.json  docker-compose.yml
      ~/new-project/.devcontainer $ docker-compose run vscode
      root@rust-env:/workspace#
      ```
   - for Visual Studio Code: open the project folder and use the **Reopen in Container** command
4. Run `cargo init` to initialize the project

## Features

- The common Rust tooling: `rustup`, `cargo`, `clippy`, `rustfmt`
- VSCode extensions for Rust development:
   - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer)
   - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) for Cargo.toml editing
   - [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) for helping with crate versions
   - [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for debugging
- Rustup bash completions
- SSH private key sharing for git access
- Git config sharing
- Persistent shell command history
- VIM for terminal based workflow

## Costumization

Additional generic (not Rust development specific) extensions can be specified in the [VSCode User Settings](https://code.visualstudio.com/docs/remote/containers#_always-installed-extensions). These are my recommended "always install" extensions:
[Gitlens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens),
[Git Graph](https://marketplace.visualstudio.com/items?itemName=mhutchie.git-graph),
[Trailing Spaces](https://marketplace.visualstudio.com/items?itemName=shardulm94.trailing-spaces)
```json
"remote.containers.defaultExtensions": [
   "eamodio.gitlens",
   "mhutchie.git-graph",
   "shardulm94.trailing-spaces",
],
```

## Useful links

[A more complex devcontainer by qdm12](https://github.com/qdm12/rustdevcontainer)

[The Microsoft way](https://github.com/microsoft/vscode-dev-containers/tree/main/containers/rust)

[VSCode devcontainer related docs](https://code.visualstudio.com/docs/remote/containers)

### Reference documentations

- [Dockerfile reference](https://docs.docker.com/engine/reference/builder/)

- [Compose file reference](https://docs.docker.com/compose/compose-file/compose-file-v3/)

- [devcontainer.json reference](https://code.visualstudio.com/docs/remote/devcontainerjson-reference)