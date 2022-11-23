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
###

## Usage

```
ucf <file_name>
```
It modifies the file in place.

```
ucf --help
```
prints help.

## Inner working
`ucf` is a very simple program. It determines the language used (using the file extension) and calls a predetermined code formatter on the file, eg. if the file ends with .py, it calls `black` on the file. 

## Why does this program exist?
Basically, I wanted to format my code when using `neovim`.
I know solutions exists, but I needed a simpler solution.
I press <F5> and the file saves and calls a code formatter.
But the problem is that each programming language has its own formatter and each of them work differently. Some output to stdout and some change the file in place, some need additional arguements to modify the file in place and etc. 
And finding the proper code formatter for each language takes time. 
So, I decided to write a program that will take care of all those for us and when we call the code formatter, it just formats the code. 

## Why not use a generic code formatter?
I understand generic code formatters exist (check [this repo](https://github.com/rishirdua/awesome-code-formatters) for a list), but theyt don't support all languages (supporting all languages perfectly is an impossible task) and the native code formatter specific for each language is much superior in terms of correctness and functionality. 

This approach of just calling other code formatters is very extensible, easy, simple and practical. 

## Why use rust for such a simple task?
1. Why not?
2. I really like rust's project management (dependency handling, versioning and the whole ecosystem)
3. I wanted to get more comfortable with the language.

## Contribute
All patches are welcome. Fork the repo, make your changes and submit a pull request. 
