mod args;

use args::UCF_Args;
use clap::Parser;
use serde_derive::{Deserialize, Serialize};
use std::process::{exit, Command};
use std::{env, fs};

// Struct to load the config file into
#[derive(Default, Debug, Serialize, Deserialize)]
struct MyConfig {
    ignored_extensions: Vec<String>,
}

pub struct UCF;

impl UCF {
    pub fn new() -> Self {
        return UCF {};
    }
    pub fn run(&self) {
        let args: UCF_Args = UCF_Args::parse();
        let mut file_name: String = args.file_name.clone();
        let is_cf: bool; // is_cf = is custom formatter given
        let cf = args.formatter.clone();
        match cf {
            Some(_x) => {
                is_cf = true;
            }
            None => {
                is_cf = false;
            }
        }

        // Load the config file form disk
        let cfg: MyConfig =
            confy::load("ucf", "config").expect("Something went wrong parsing the config files");

        let file_extension: String = find_extension(&file_name);
        // Skip processing of file if the extension is specified in the config file
        for i in cfg.ignored_extensions.iter() {
            if i.eq(&file_extension) {
                if is_cf == false {
                    println!(
                        "File extension present in ignored extension list in the config file. Skipping"
                    );
                    exit(0);
                }
            }
        }
        let mut formatter: String = String::new();
        let mut formatter_args: Vec<String> = Vec::new();
        // Find what formatter to use and what formatter arguments to supply depending on the extension
        match file_extension.as_str() {
            "c" | "C" | "cpp" | "CPP" | "cc" | "c++" | "cxx" | "cp" | "cs" | "h" | "hpp"
            | "hxx" | "java" | "json" | "m" => {
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
            "css" | "cjs" | "gfm" | "graphql" | "gql" | "html" | "js" | "jsx" | "less" | "md"
            | "mdx" | "prettierrc" | "sass" | "scss" | "svelte" | "ts" | "vue" | "yaml" => {
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
            "sh" | "ebuild" => {
                formatter = String::from("shfmt");
                formatter_args.push(String::from("-w"));
            }
            "toml" => {
                formatter = String::from("taplo");
                formatter_args.push(String::from("fmt"));
                formatter_args.push(file_name.clone());
                if is_cf == false {
                    file_name.clear();
                }
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
                if is_cf == false {
                    file_name.clear();
                }
            }
            _ => {
                if is_cf == false {
                    println!("The extension of {} is not supported by ucf (yet)!", file_extension);
                    exit(1);
                }
            }
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

        let success = format_code(&file_name, &formatter, &formatter_args);
        if success.is_some() {
            let res = success.unwrap();
            if res {
                println!("{} formatted successfully", &file_name);
            } else {
                println!("Your code was not formatted.");
                println!("The file extension was {}.", file_extension);
                println!(
                    "{} refused to format the file. Maybe there are errors in the code?",
                    &formatter
                );
            }
        } else {
            println!("Your code was not formatted.");
            println!("The file extension was {}.", file_extension);
            println!(
                "Make sure `{}` is present in your system's $PATH",
                &formatter
            );
        }
    }
}

fn is_program_in_path(program: &str) -> bool {
    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    return false;
}

fn find_extension(file_name: &String) -> String {
    let split = file_name.split(".");
    let mut split_vec: Vec<&str> = split.collect();
    return String::from(split_vec.pop().unwrap());
}

fn format_code(file_name: &String, formatter: &String, args: &Vec<String>) -> Option<bool> {
    if is_program_in_path(&formatter) {
        let mut command = Command::new(formatter);
        command.args(args);
        if file_name.trim().is_empty() {
        } else {
            command.arg(&file_name);
        }
        let status = command.status().expect("Something went wrong");
        return Some(status.success());
    }
    return None;
}
