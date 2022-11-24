# Universal Code Formatter

A code formatter that formats any code. 

## Usage
```
ucf <file_name>
```
Remember that `ucf` modifies the file in place and there is no option to output to stdout at the moment.

```
ucf --help
```
prints help.

## Inner working
`ucf` is a very simple program. It determines the file extension and calls a predetermined code formatter on the file as listed in the table below with necessary arguments to modify the file in place.

## Currently used formatters
| Formatter called   | File Extension                                                                               |
|--------------------|-----------------------------------------------------------------------------------------|
| black              | .py                                                                                  |
| clang-format       | .c, .cpp, .cc                                                                                  |
| gofmt              | .go                                                                                  |
| google-java-format | .java                                                                                    |
| prettier           | .css, .gfm, .html, .js, .json, .jsx, .less, .md, .mdx, .sass, .scss, .ts, .vue, .yaml |
| rustfmt            | .rs

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
