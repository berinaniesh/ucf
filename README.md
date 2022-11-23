# Universal Code Formatter

A code formatter that formats any code. 

## Install
### Using cargo
```
cargo install ucf
```
### Arch Linux
For Arch Linux, `ucf` is available in the [aur](https://aur.archlinux.org)

```
yay -S ucf
```
### Dependencies
Since `ucf` doesn't format code on its own, it expects the respective code formatters to be present in the system's path. The dependencies are `black`, `clang-format`, `prettier` and `rustfmt`.

## Usage

```
ucf <file_name>
```
Remember that `ucf` modifies the file in place.

```
ucf --help
```
prints help.

## Inner working
`ucf` is a very simple program. It determines the language used (using the file extension) and calls a predetermined code formatter on the file, eg. if the file ends with .py, it calls `black` on the file, if the file ends with .cpp or .c or cc, it calls `clang-format` on the file. 

I think this approach is more extensible, practical and simple. 

## Planned features
* Read from stdin and output to stdout (so that this program can be used in IDEs seamlessly). 
* Additional languages.

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
