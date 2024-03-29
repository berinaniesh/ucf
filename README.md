# Universal Code Formatter

[![Latest Version](https://img.shields.io/crates/v/ucf?color=g)](https://crates.io/crates.ucf)

A code formatter that formats any code.

## Install

### With cargo

```
cargo install ucf
```

### Arch linux

`ucf` is available in the aur [here](https://aur.archlinux.org/packages/ucf). With an aur helper like `yay`, it can be installed with
```
yay -S ucf
```

## Usage

```
ucf <file_name>
```

**Caution:** `ucf` modifies the file in place and there is no option to output to stdout at the moment.

```
ucf --help
```

prints help.

## Inner working

`ucf` is a simple program. It determines the file extension and calls a predetermined code formatter on the file as listed in the table below with necessary arguments to modify the file in place. **The formatter called should be present in the system's `PATH`**.

One advantage this has is that the config files for the respective programs (rustfmt.toml, .prettierrc etc) are read automatically.

## Currently used formatters

| Formatter called | File Extension                                                                 |
| ---------------- | ------------------------------------------------------------------------------ | 
| black            | .py                                                                            |
| clang-format     | .c, .cpp, .cc, .cs, .cxx, .cp, .cs, .h, .hpp, .hxx .java, .json, .m (objective c, not matlab)            |
| cmake-format     | .cmake                                                                         |
| gofmt            | .go                                                                            |
| ocamlformat		| .ocaml |
| prettier         | .css, .gfm, .graphql, .gql, .html, .js, .jsx, .less, .md, .mdx, .prettierrc, .sass, .scss, .svelte, .ts, .vue, .yaml |
| rustfmt          | .rs                                                                            |
| shfmt		   | .sh, .ebuild    |
| stylua           | .lua                                                                           |
| stylish-haskell  | .hs                                                                            |
| taplo		   | .toml	|
| xmllint	     | 	   .xml |
| zig fmt 	| .zig	|

## Editor Integration

### vim / neovim

After saving the file, run `ucf` on the file as a shell command.

```
:!ucf %
```

This can be automated in many ways, eg. To bind `<F5>` to save and format the file, add the following line in `init.lua`,

```
vim.keymap.set('n', '<F5>', ':w | :!ucf % <CR><CR>', {noremap=true, silent=true})
```

## Config file

To ignore certain file extensions, add the extensions without the dot in the `ignored_extensions` array in `$XDG_CONFIG_HOME/ucf/config.toml` file. `ucf` formats code overriding the config file if a custom formatter is given. 

## Todo

- [ ] Add option to read folders and run formatter on all files
- [ ] Add option to format `SQL`

## For packagers

If you would like to package `ucf` for any distro, make sure to add the packages which provide the binaries from the table above as optional dependencies.

## Contribute

All patches are welcome. Fork the repo, make your changes and submit a pull request.
