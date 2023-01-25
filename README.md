# Universal Code Formatter

[![Latest Version](https://img.shields.io/crates/v/ucf?color=g)](https://crates.io/crates.ucf)

A code formatter that formats any code.

## Install

### With cargo

```
cargo install ucf
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

## Currently used formatters

| Formatter called | File Extension                                                                 |
| ---------------- | ------------------------------------------------------------------------------ | 
| black            | .py                                                                            |
| clang-format     | .c, .cpp, .cc, .cs, .h, .hpp, .java, .json, .m                                 |
| cmake-format     | .cmake                                                                         |
| gofmt            | .go                                                                            |
| ocamlformat		| .ocaml |
| prettier         | .css, .gfm, .graphql, .gql, .html, .js, .jsx, .less, .md, .mdx, .prettierrc, .sass, .scss, .svelte, .ts, .vue, .yaml |
| rustfmt          | .rs                                                                            |
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


## Contribute

All patches are welcome. Fork the repo, make your changes and submit a pull request.


Read more [here](https://berinaniesh.github.io/projects/ucf)
