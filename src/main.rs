mod args;

use args::UCF_Args;
use clap::Parser;
use std::process::{exit, Command};

fn main() {
    let args: UCF_Args = UCF_Args::parse();
    let mut file_name: String = args.file_name.clone();
    let file_extension: String = find_extension(&file_name);
    let mut formatter: String = String::new();
    let mut formatter_args: Vec<String> = Vec::new();
    match file_extension.as_str() {
        "c" | "cpp" | "cc" | "cs" | "h" | "hpp" | "java" | "json" | "m" => {
            formatter = String::from("clang-format");
            formatter_args.push(String::from("-i"));
        }
        "cmake" => {
            formatter = String::from("cmake-format");
            formatter_args.push(String::from("-i"));
        }
        "go" => {
            formatter = String::from("gofmt");
            formatter_args.push(String::from("-w"));
        }
        "css" | "gfm" | "graphql" | "gql" | "html" | "js" | "jsx" | "less" | "md" | "mdx"
        | "sass" | "scss" | "svelte" | "ts" | "vue" | "yaml" => {
            formatter = String::from("prettier");
            formatter_args.push(String::from("-w"));
        }
        "py" | "ipynb" => {
            formatter = String::from("black");
        }
        "rs" => {
            formatter = String::from("rustfmt");
        }
        "hs" => {
            formatter = String::from("stylish-haskell");
            formatter_args.push(String::from("-i"));
        }
        "lua" => {
            formatter = String::from("stylua");
        }
        "ocaml" => {
            formatter = String::from("ocamlformat");
            formatter_args.push(String::from("-i"));
        }
        "toml" => {
            formatter = String::from("taplo");
            formatter_args.push(String::from("fmt"));
            formatter_args.push(file_name.clone());
            file_name.clear(); // Check condition where custom formatter is given
        }
        "xml" => {
            formatter = String::from("xmllint");
            formatter_args.push(String::from("--format"));
            formatter_args.push(String::from("--output"));
            formatter_args.push(file_name.clone());
        }
        "zig" => {
            formatter = String::from("zig");
            formatter_args.push(String::from("fmt"));
            formatter_args.push(file_name.clone());
            file_name.clear(); // Check condition where custom formatter is given
        }
        _ => {}
    }

    let custom_formatter = args.formatter.clone();
    match custom_formatter {
        Some(x) => {
            formatter = x;
            formatter_args = Vec::new();
        }
        None => {}
    }

    let custom_formatter_args = args.args_formatter.clone();
    match custom_formatter_args {
        Some(ref x) => {
            formatter_args = x.split(" ").map(String::from).collect::<Vec<String>>();
        }
        None => {}
    }

    let success: bool = format_code(&file_name, &formatter, &formatter_args);
    if success {
        exit(0)
    } else {
        exit(1)
    }
}

fn find_extension(file_name: &String) -> String {
    let split = file_name.split(".");
    let mut split_vec: Vec<&str> = split.collect();
    return String::from(split_vec.pop().unwrap());
}

fn format_code(file_name: &String, formatter: &String, args: &Vec<String>) -> bool {
    let mut command = Command::new(formatter);
    if file_name.trim().is_empty() {}
    else {
        command.arg(&file_name);
    }
    command.args(args);
    let status = command.status().expect("Something went wrong");
    return status.success();
}
