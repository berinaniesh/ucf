use std::env;

const PROGRAM_NAME: &'static str = "ucf";
const PROGRAM_DESC: &'static str = "Universal Code Formatter";

fn print_help() {
    let help_string: String = String::from("Help");
}

fn parse() {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{}", args[1]);
    if args.len() != 2 {
        panic!("Improper number of arguements given");
    }

    let _languages = [("C", ".c", "clang-format"), ("C++", ".cpp", "clang-format"), ("C++", ".cc", "clang-format"), ("Java", ".java", "google-java-format"), ("Python", ".py", "black"), ("Javascript", ".js", "prettier"), ("Typescript", "ts", "prettier")];

    match args[1].as_str() {
       "--help" => print_help(),
       _ => {}
    }

}
