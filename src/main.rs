mod args;

use args::UCF_Args;
use clap::Parser;
use std::process::{exit, Command};

fn main() {
    let args: UCF_Args = UCF_Args::parse();
    let file_name: String = args.file_name.clone();
    let file_extension: String = find_extension(&file_name);
    let formatter: String;
    let mut formatter_args: Vec<&str> = Vec::new();
    match file_extension.as_str() {
        "c" | "cpp" | "cc" => {
            formatter = String::from("clang-format");
            formatter_args.push("-i");
        }
        "go" => {
            formatter = String::from("gofmt");
            formatter_args.push("-w");
        }
        "java" => {
            formatter = String::from("google-java-format");
            formatter_args.push("-i");
        }
        "css"|"gfm"|"html"|"js"|"json"|"jsx"|"less"|"md"|"mdx"|"sass"|"scss"|"ts"|"vue"|"yaml" => {
            formatter = String::from("prettier");
            formatter_args.push("-w");
        }
        "py" => {
            formatter = String::from("black");
        }
        "rs" => {
            formatter = String::from("rustfmt");
        }
        _ => {
            exit(0);
        }
    }
    format_code(&file_name, &formatter, &formatter_args);
}

fn find_extension(file_name: &String) -> String {
    let split = file_name.split(".");
    let mut split_vec: Vec<&str> = split.collect();
    return String::from(split_vec.pop().unwrap());
}

fn format_code(file_name: &String, formatter: &String, args: &Vec<&str>) {
    let status = Command::new(formatter)
        .arg(&file_name)
        .args(args)
        .status()
        .expect("Some error");
    assert!(status.success());
}
