# Universal Code Formatter

A code formatter that formats any code. 

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
`ucf` is a simple program. It determines the file extension and calls a predetermined code formatter on the file as listed in the table below with necessary arguments to modify the file in place. The formatter called should be present in the system's `PATH`.

## Currently used formatters
| Formatter called   | File Extension                                                                               |
|--------------------|-----------------------------------------------------------------------------------------|
| black              | .py                                                                                  |
| clang-format       | .c, .cpp, .cc, .cs, .h, .hpp, .java, .json, .m
| cmake-format       | .cmake |                                                                               |
| gofmt              | .go                                                                                  |
| prettier           | .css, .gfm, .html, .js, .jsx, .less, .md, .mdx, .sass, .scss, .ts, .vue, .yaml |
| rustfmt            | .rs
| stylua	     | .lua   	|
| stylish-haskell    | .hs	|

## Editor Integration

One of the reasons I wrote this wrapper is for simpler editor integrations. 
For `vim` / `neovim`, after saving the file, run this program on your file as a shell command. 
```
:!ucf %
```
I bind saving and running code formatter as <F5>. In `init.lua`,
```
vim.keymap.set('n', '<F5>', ':w | :!ucf % <CR><CR>', {noremap=true, silent=true})
```
saves the file and runs the formatter on the files. 

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
