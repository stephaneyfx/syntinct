# Overview
This tool generates a dark theme for neovim.

# Install
## Nix & home-manager

This repository is a flake that contains the following:
- Packages:
  - `syntark`: Dark theme for neovim.
  - `syntinct`: Tool to generate the theme.
- Default overlay (`overlays.default`) adding the packages defined in this flake.

## Other platforms

The tool needs to be built and run, which is detailed in the [build](#build) section.

# Build
## Prerequisites

- Rust

## Build & install

In the command below, `$OUT` refers to a directory in the neovim `runtimepath`. On linux, this can
be `$XDG_CONFIG_HOME/nvim`.

```sh
mkdir -p $OUT/colors
cargo run > $OUT/colors/syntark.lua
```

The theme can be selected in neovim using the following:

```
:colorscheme syntark
```

# Contribute
All contributions shall be licensed under the [0BSD license](https://spdx.org/licenses/0BSD.html).
