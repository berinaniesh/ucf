# Universal Code Formatter

A code formatter that formats any code. 

## Install

Remember this is still alpha software. So, until everything is stable, I m not going to push it to crates.io or the aur. 

Currently, the easiest way to use `ucf` is to have the rust toolchain installed, clone the repo, build it and move the executable to the local path. With `cargo` installed
```
git clone https://github.com/berinaniesh/ucf.git
cd ucf
cargo build --release
cp target/release/ucf ~/.local/bin/
```
There are also linux and windows binaries provided in the releases page. (MacOS WIP, use the above method). 

<!---
### Using cargo
```
cargo install ucf
```
### Arch Linux (todo)
For Arch Linux, `ucf` is available in the [aur](https://aur.archlinux.org)

```
yay -S ucf
```
--->

### Dependencies
Since `ucf` doesn't format code on its own, it expects the respective code formatters to be present in the system's path. Install whatever language formatters you need. Every formatter would be listed as an optional dependency (with common package managers). 

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
`ucf` is a very simple program. It determines the language used (using the file extension) and calls a predetermined code formatter on the file, eg. if the file ends with .py, it calls `black` on the file with necessary arguments to modify the file in place.

## Currently used formatters
| Formatter          | Languages                                                                               |
|--------------------|-----------------------------------------------------------------------------------------|
| black              | Python                                                                                  |
| clang-format       | C, C++                                                                                  |
| gofmt              | Golang                                                                                  |
| google-java-format | Java                                                                                    |
| prettier           | CSS, GFM, HTML, JavaScript, JSON, JSX, less, md, mdx, sass, scss, TypeScript, Vue, YAML |
| rustfmt            | Rust

Adding support for a language is a trivial task as adding another case in the match statement. Raise issues or pull requests to support additional languages. 

<!---
## Planned features
* Read from stdin and output to stdout (so that this program can be used in IDEs seamlessly). 
* Additional languages.
--->

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
