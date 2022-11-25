mod args;

use args::UCF_Args;
use clap::Parser;
use std::process::{exit, Command};

fn main() {
    let args: UCF_Args = UCF_Args::parse();
    let file_name: String = args.file_name.clone();
    let file_extension: String = find_extension(&file_name);
    let mut formatter: String = String::new();
    let mut formatter_args: Vec<&str> = Vec::new();
    match file_extension.as_str() {
        "c" | "cpp" | "cc" | "cs" | "h" | "hpp" | "java" | "json" | "m" => {
            formatter = String::from("clang-format");
            formatter_args.push("-i");
        }
        "cmake" => {
            formatter = String::from("cmake-format");
            formatter_args.push("-i");
        }
        "go" => {
            formatter = String::from("gofmt");
            formatter_args.push("-w");
        }
        "css" | "gfm" | "html" | "js" | "jsx" | "less" | "md" | "mdx" | "sass" | "scss" | "ts"
        | "vue" | "yaml" => {
            formatter = String::from("prettier");
            formatter_args.push("-w");
        }
        "py" | "ipynb" => {
            formatter = String::from("black");
        }
        "rs" => {
            formatter = String::from("rustfmt");
        }
        "hs" => {
            formatter = String::from("stylish-haskell");
            formatter_args.push("-i");
        }
        "lua" => {
            formatter = String::from("stylua");
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
            formatter_args = x.split(" ").collect::<Vec<&str>>();
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

fn format_code(file_name: &String, formatter: &String, args: &Vec<&str>) -> bool {
    let status = Command::new(formatter)
        .arg(&file_name)
        .args(args)
        .status()
        .expect("Some error");
    return status.success();
}
