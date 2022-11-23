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
yay -S ucf --noconfirm
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

## Why does this program exist?
Basically, I wanted a single program to format any code I throw at it. 
I did not want to hunt for a formatter for each language I write. 
I did not want to read the man page for each code formatter I found. 
But at the end of the day, I went through all the above and wasted so much time. I don't want any others to go through the same process again. 
And each code formatter has its own way of doing things. One will output to stdout and the other will modify in place. Each code formatter needed different arguments to do the same job.
So, I decided to write a program that will take care of all those shenanigans, remove the burden from users. 
Here, you call `ucf` and it formats code in place. That's all, super easy, barely an inconvenience. 

## Why not use a generic code formatter?
I understand generic code formatters exist (check [this repo](https://github.com/rishirdua/awesome-code-formatters) for a list), but 
* They don't support all languages (supporting all languages perfectly is an impossible task)
* In most cases, the native code formatter or a language specific one is much superior in terms of correctness and functionality (for eg. `black` checks the [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) for each python file. I am not sure if prettier does that). So, I want to use `black` for python and `prettier` for javascript. 

## Inner working
`ucf` is a very simple program. It determines the language used (using the file extension) and calls a predetermined code formatter on the file, eg. if the file ends with .py, it calls `black` on the file, if the file ends with .cpp or .c or cc, it calls `clang-format` on the file. 

I think this approach is more extensible, practical and simple. 

## Why use rust for such a simple task?
1. Why not?
2. I really like rust's project management (dependency handling, versioning and the whole ecosystem)
3. I wanted to get more comfortable with the language.

## Planned features
* Read from stdin and output to stdout (so that this program can be used in IDEs seamlessly). 

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
